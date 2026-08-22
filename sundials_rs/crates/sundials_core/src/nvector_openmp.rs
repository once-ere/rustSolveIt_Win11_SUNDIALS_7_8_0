//! `nvector_openmp` — the OpenMP N_Vector.
//!
//! Translated from `src/nvector/openmp/nvector_openmp.c` and
//! `include/nvector/nvector_openmp.h` of SUNDIALS 7.8.0.
//!
//! # Relationship to `nvector_serial`
//!
//! Upstream wrote `nvector_openmp.c` as `nvector_serial.c` with
//! `#pragma omp parallel for` added, and this module is derived the same way:
//! the content carries a thread count, the constructors take one, and the five
//! **sum** reductions are chunked. Everything else is arithmetically identical
//! to the serial vector.
//!
//! # Why the thread count changes the answer
//!
//! Five ops carry `reduction(+ : sum) schedule(static)`: [`N_VDotProd_OpenMP`],
//! [`N_VWL2Norm_OpenMP`], [`N_VL1Norm_OpenMP`], [`N_VWSqrSumLocal_OpenMP`] and
//! [`N_VWSqrSumMaskLocal_OpenMP`]. OpenMP gives each thread a private partial,
//! sums a contiguous chunk into it, then combines the partials — so the
//! *order* of the additions, and therefore the last bits of the result, depend
//! on how many threads there are. This is not a defect in either
//! implementation; floating-point addition is not associative.
//!
//! Measured on this machine with `cvAdvDiff_bnd_omp`: the C build prints four
//! different results at 1, 2, 4 and 8 threads, and the same result on every
//! run at a fixed count. The reference output shipped with SUNDIALS was
//! produced at **4 threads**, and the MSVC C build reproduces it exactly
//! there.
//!
//! So this port does not attempt a thread-count-independent answer, which
//! would be meaningless. It reproduces the OpenMP arithmetic exactly, for
//! whatever `num_threads` the vector was created with — see
//! [`omp_sum_static`].
//!
//! The min/max reductions (`N_VMaxNorm`, `N_VMin`, `N_VMinQuotient`,
//! `N_VConstrMask`, `N_VInvTest`) use `#pragma omp critical` rather than a
//! sum reduction. Min and max are associative and exact, so their result does
//! not depend on the thread count and they need no chunking.

use std::cell::RefMut;

use crate::sundials_context::SUNContext;
use crate::sundials_errors::SUN_SUCCESS;
use crate::sundials_math::*;
use crate::sundials_nvector::*;
use crate::sundials_types::*;
use crate::sundials_utils::{sun_format_e, SUNFile};

const ZERO: sunrealtype = 0.0;
const HALF: sunrealtype = 0.5;
const ONE: sunrealtype = 1.0;
const ONEPT5: sunrealtype = 1.5;

pub struct _N_VectorContent_OpenMP {
    pub length: sunindextype,
    pub num_threads: i32,
    pub own_data: sunbooleantype,
    pub data: Vec<sunrealtype>,
}

pub type N_VectorContent_OpenMP = _N_VectorContent_OpenMP;

/// C macro `NV_CONTENT_OMP(v)` (mutable borrow of the serial content).
fn content_mut(v: &N_Vector) -> RefMut<'_, N_VectorContent_OpenMP> {
    RefMut::map(v.content.borrow_mut(), |c| {
        c.downcast_mut::<N_VectorContent_OpenMP>()
            .expect("openmp N_Vector content")
    })
}

/// C macro `NV_LENGTH_OMP(v)`.
pub fn NV_LENGTH_OMP(v: &N_Vector) -> sunindextype {
    content_mut(v).length
}

/// C macro `NV_NUM_THREADS_OMP(v)`.
pub fn NV_NUM_THREADS_OMP(v: &N_Vector) -> i32 {
    content_mut(v).num_threads
}

/// The iteration space `0..n` split the way OpenMP's `schedule(static)`
/// splits it: `nthreads` contiguous chunks, the first `n % nthreads` of them
/// one element longer.
fn static_chunks(n: usize, nthreads: usize) -> Vec<(usize, usize)> {
    let nt = nthreads.max(1).min(n.max(1));
    let q = n / nt;
    let r = n % nt;
    let mut out = Vec::with_capacity(nt);
    let mut lo = 0usize;
    for i in 0..nt {
        let len = q + usize::from(i < r);
        out.push((lo, lo + len));
        lo += len;
    }
    out
}

/// `#pragma omp parallel for reduction(+ : sum) schedule(static)`.
///
/// Each chunk is summed in index order into its own partial — that is the
/// thread-private copy the `reduction` clause creates — and the partials are
/// then added into the initial value in ascending chunk order. Reproducing
/// this order is the whole reason the port matches the C bit for bit at a
/// given thread count; summing straight through `0..n` would not.
pub fn omp_sum_static(n: usize, nthreads: i32, f: impl Fn(usize) -> sunrealtype) -> sunrealtype {
    let mut sum = ZERO;
    for (lo, hi) in static_chunks(n, nthreads.max(1) as usize) {
        let mut partial = ZERO;
        for i in lo..hi {
            partial += f(i);
        }
        sum += partial;
    }
    sum
}

/// C macro `NV_OWN_DATA_OMP(v)`.
pub fn NV_OWN_DATA_OMP(v: &N_Vector) -> sunbooleantype {
    content_mut(v).own_data
}

/// C macro `NV_DATA_OMP(v)` — the data pointer as a `RefMut` guard.
/// Drop the guard before calling any other op on the same vector.
pub fn NV_DATA_OMP(v: &N_Vector) -> RefMut<'_, Vec<sunrealtype>> {
    RefMut::map(v.content.borrow_mut(), |c| {
        &mut c
            .downcast_mut::<N_VectorContent_OpenMP>()
            .expect("openmp N_Vector content")
            .data
    })
}

/// Alias detection: C pointer equality of the vector handles.
fn same(a: &N_Vector, b: &N_Vector) -> bool {
    std::rc::Rc::ptr_eq(a, b)
}

/* ----------------------------------------------------------------------------
 * Function to create a new empty serial vector
 */

pub fn N_VNewEmpty_OpenMP(
    length: sunindextype,
    num_threads: i32,
    sunctx: &SUNContext,
) -> Option<N_Vector> {
    if length < 0 {
        return None;
    }

    /* Create an empty vector object */
    let v = N_VNewEmpty(sunctx)?;

    /* Attach operations */
    {
        let mut ops = v.ops.borrow_mut();

        /* constructors, destructors, and utility operations */
        ops.nvgetvectorid = Some(N_VGetVectorID_OpenMP);
        ops.nvclone = Some(N_VClone_OpenMP);
        ops.nvcloneempty = Some(N_VCloneEmpty_OpenMP);
        ops.nvdestroy = Some(N_VDestroy_OpenMP);
        ops.nvspace = Some(N_VSpace_OpenMP);
        ops.nvgetarraypointer = Some(N_VGetArrayPointer_OpenMP);
        ops.nvsetarraypointer = Some(N_VSetArrayPointer_OpenMP);
        ops.nvgetlength = Some(N_VGetLength_OpenMP);
        ops.nvgetlocallength = Some(N_VGetLength_OpenMP);

        /* standard vector operations */
        ops.nvlinearsum = Some(N_VLinearSum_OpenMP);
        ops.nvconst = Some(N_VConst_OpenMP);
        ops.nvprod = Some(N_VProd_OpenMP);
        ops.nvdiv = Some(N_VDiv_OpenMP);
        ops.nvscale = Some(N_VScale_OpenMP);
        ops.nvabs = Some(N_VAbs_OpenMP);
        ops.nvinv = Some(N_VInv_OpenMP);
        ops.nvaddconst = Some(N_VAddConst_OpenMP);
        ops.nvdotprod = Some(N_VDotProd_OpenMP);
        ops.nvmaxnorm = Some(N_VMaxNorm_OpenMP);
        ops.nvwrmsnormmask = Some(N_VWrmsNormMask_OpenMP);
        ops.nvwrmsnorm = Some(N_VWrmsNorm_OpenMP);
        ops.nvmin = Some(N_VMin_OpenMP);
        ops.nvwl2norm = Some(N_VWL2Norm_OpenMP);
        ops.nvl1norm = Some(N_VL1Norm_OpenMP);
        ops.nvcompare = Some(N_VCompare_OpenMP);
        ops.nvinvtest = Some(N_VInvTest_OpenMP);
        ops.nvconstrmask = Some(N_VConstrMask_OpenMP);
        ops.nvminquotient = Some(N_VMinQuotient_OpenMP);

        /* fused and vector array operations are disabled (NULL) by default */

        /* local reduction operations */
        ops.nvdotprodlocal = Some(N_VDotProd_OpenMP);
        ops.nvmaxnormlocal = Some(N_VMaxNorm_OpenMP);
        ops.nvminlocal = Some(N_VMin_OpenMP);
        ops.nvl1normlocal = Some(N_VL1Norm_OpenMP);
        ops.nvinvtestlocal = Some(N_VInvTest_OpenMP);
        ops.nvconstrmasklocal = Some(N_VConstrMask_OpenMP);
        ops.nvminquotientlocal = Some(N_VMinQuotient_OpenMP);
        ops.nvwsqrsumlocal = Some(N_VWSqrSumLocal_OpenMP);
        ops.nvwsqrsummasklocal = Some(N_VWSqrSumMaskLocal_OpenMP);

        /* single buffer reduction operations */
        ops.nvdotprodmultilocal = Some(N_VDotProdMulti_OpenMP);

        /* XBraid interface operations */
        ops.nvbufsize = Some(N_VBufSize_OpenMP);
        ops.nvbufpack = Some(N_VBufPack_OpenMP);
        ops.nvbufunpack = Some(N_VBufUnpack_OpenMP);

        /* debugging functions */
        ops.nvprint = Some(N_VPrint_OpenMP);
        ops.nvprintfile = Some(N_VPrintFile_OpenMP);
    }

    /* Create and attach content */
    *v.content.borrow_mut() = Box::new(N_VectorContent_OpenMP {
        length,
        num_threads,
        own_data: SUNFALSE,
        data: Vec::new(),
    });

    Some(v)
}

/* ----------------------------------------------------------------------------
 * Function to create a new serial vector
 */

pub fn N_VNew_OpenMP(
    length: sunindextype,
    num_threads: i32,
    sunctx: &SUNContext,
) -> Option<N_Vector> {
    if length < 0 {
        return None;
    }
    let v = N_VNewEmpty_OpenMP(length, num_threads, sunctx)?;

    /* Create and attach data */
    if length > 0 {
        let mut content = content_mut(&v);
        content.own_data = SUNTRUE;
        content.data = vec![0.0; length as usize];
    }

    Some(v)
}

/* ----------------------------------------------------------------------------
 * Function to create a serial N_Vector with user data component
 * (Rust: the vector takes ownership of the provided buffer)
 */

pub fn N_VMake_OpenMP(
    length: sunindextype,
    num_threads: i32,
    v_data: Vec<sunrealtype>,
    sunctx: &SUNContext,
) -> Option<N_Vector> {
    if length < 0 {
        return None;
    }
    let v = N_VNewEmpty_OpenMP(length, num_threads, sunctx)?;

    if length > 0 {
        /* Attach data */
        let mut content = content_mut(&v);
        content.own_data = SUNFALSE;
        content.data = v_data;
    }

    Some(v)
}

pub fn N_VGetVectorID_OpenMP(_v: &N_Vector) -> N_Vector_ID {
    SUNDIALS_NVEC_OPENMP
}

pub fn N_VGetLength_OpenMP(v: &N_Vector) -> sunindextype {
    NV_LENGTH_OMP(v)
}

pub fn N_VPrint_OpenMP(x: &N_Vector) {
    N_VPrintFile_OpenMP(x, &SUNFile::Stdout);
}

pub fn N_VPrintFile_OpenMP(x: &N_Vector, outfile: &SUNFile) {
    let n = NV_LENGTH_OMP(x);
    let xd = NV_DATA_OMP(x);
    for i in 0..n as usize {
        outfile.write_str(&format!("{}\n", sun_format_e(xd[i])));
    }
}

pub fn N_VCloneEmpty_OpenMP(w: &N_Vector) -> Option<N_Vector> {
    /* Create vector */
    let v = N_VNewEmpty(&w.sunctx.borrow())?;

    /* Attach operations */
    N_VCopyOps(w, &v);

    /* Create, attach, initialize content */
    *v.content.borrow_mut() = Box::new(N_VectorContent_OpenMP {
        length: NV_LENGTH_OMP(w),
        num_threads: NV_NUM_THREADS_OMP(w),
        own_data: SUNFALSE,
        data: Vec::new(),
    });

    Some(v)
}

pub fn N_VClone_OpenMP(w: &N_Vector) -> Option<N_Vector> {
    let v = N_VCloneEmpty_OpenMP(w)?;

    let length = NV_LENGTH_OMP(w);

    /* Create data */
    if length > 0 {
        let mut content = content_mut(&v);
        content.own_data = SUNTRUE;
        content.data = vec![0.0; length as usize];
    }

    Some(v)
}

pub fn N_VDestroy_OpenMP(v: N_Vector) {
    drop(v);
}

pub fn N_VSpace_OpenMP(v: &N_Vector, lrw: &mut sunindextype, liw: &mut sunindextype) {
    *lrw = NV_LENGTH_OMP(v);
    *liw = 1;
}

pub fn N_VGetArrayPointer_OpenMP(v: &N_Vector) -> Option<RefMut<'_, Vec<sunrealtype>>> {
    Some(NV_DATA_OMP(v))
}

pub fn N_VSetArrayPointer_OpenMP(v_data: Vec<sunrealtype>, v: &N_Vector) {
    if NV_LENGTH_OMP(v) > 0 {
        content_mut(v).data = v_data;
    }
}

/* ----------------------------------------------------------------------------
 * Alias-safe elementwise helpers. Each preserves the C loop
 * `for i: z[i] = f(x[i], [y[i]])` exactly; the branches only decide which
 * RefCell borrows are taken so that aliased operands share one borrow.
 */

fn unop(x: &N_Vector, z: &N_Vector, f: impl Fn(sunrealtype) -> sunrealtype) {
    let n = NV_LENGTH_OMP(x) as usize;
    if same(x, z) {
        let mut zd = NV_DATA_OMP(z);
        for i in 0..n {
            zd[i] = f(zd[i]);
        }
    } else {
        let xd = NV_DATA_OMP(x);
        let mut zd = NV_DATA_OMP(z);
        for i in 0..n {
            zd[i] = f(xd[i]);
        }
    }
}

fn binop(
    x: &N_Vector,
    y: &N_Vector,
    z: &N_Vector,
    f: impl Fn(sunrealtype, sunrealtype) -> sunrealtype,
) {
    let n = NV_LENGTH_OMP(x) as usize;
    let xz = same(x, z);
    let yz = same(y, z);
    if xz && yz {
        let mut zd = NV_DATA_OMP(z);
        for i in 0..n {
            zd[i] = f(zd[i], zd[i]);
        }
    } else if xz {
        let yd = NV_DATA_OMP(y);
        let mut zd = NV_DATA_OMP(z);
        for i in 0..n {
            zd[i] = f(zd[i], yd[i]);
        }
    } else if yz {
        let xd = NV_DATA_OMP(x);
        let mut zd = NV_DATA_OMP(z);
        for i in 0..n {
            zd[i] = f(xd[i], zd[i]);
        }
    } else {
        /* x may alias y: two shared borrows are fine */
        let xd = NV_DATA_OMP(x);
        let yd = if same(x, y) { None } else { Some(NV_DATA_OMP(y)) };
        let mut zd = NV_DATA_OMP(z);
        match yd {
            Some(yd) => {
                for i in 0..n {
                    zd[i] = f(xd[i], yd[i]);
                }
            }
            None => {
                for i in 0..n {
                    zd[i] = f(xd[i], xd[i]);
                }
            }
        }
    }
}

pub fn N_VLinearSum_OpenMP(
    a: sunrealtype,
    x: &N_Vector,
    b: sunrealtype,
    y: &N_Vector,
    z: &N_Vector,
) {
    if (b == ONE) && same(z, y) {
        /* BLAS usage: axpy y <- ax+y */
        Vaxpy_OpenMP(a, x, y);
        return;
    }

    if (a == ONE) && same(z, x) {
        /* BLAS usage: axpy x <- by+x */
        Vaxpy_OpenMP(b, y, x);
        return;
    }

    /* Case: a == b == 1.0 */
    if (a == ONE) && (b == ONE) {
        VSum_OpenMP(x, y, z);
        return;
    }

    /* Cases: (1) a == 1.0, b = -1.0, (2) a == -1.0, b == 1.0 */
    let test = (a == ONE) && (b == -ONE);
    if test || ((a == -ONE) && (b == ONE)) {
        let v1 = if test { y } else { x };
        let v2 = if test { x } else { y };
        VDiff_OpenMP(v2, v1, z);
        return;
    }

    /* Cases: (1) a == 1.0, b == other or 0.0, (2) a == other or 0.0, b == 1.0 */
    /* if a or b is 0.0, then user should have called N_VScale */
    let test = a == ONE;
    if test || (b == ONE) {
        let c = if test { b } else { a };
        let v1 = if test { y } else { x };
        let v2 = if test { x } else { y };
        VLin1_OpenMP(c, v1, v2, z);
        return;
    }

    /* Cases: (1) a == -1.0, b != 1.0, (2) a != 1.0, b == -1.0 */
    let test = a == -ONE;
    if test || (b == -ONE) {
        let c = if test { b } else { a };
        let v1 = if test { y } else { x };
        let v2 = if test { x } else { y };
        VLin2_OpenMP(c, v1, v2, z);
        return;
    }

    /* Case: a == b (catches a == b == 0.0 - user should have called N_VConst) */
    if a == b {
        VScaleSum_OpenMP(a, x, y, z);
        return;
    }

    /* Case: a == -b */
    if a == -b {
        VScaleDiff_OpenMP(a, x, y, z);
        return;
    }

    /* Do all cases not handled above:
    (1) a == other, b == 0.0 - user should have called N_VScale
    (2) a == 0.0, b == other - user should have called N_VScale
    (3) a,b == other, a !=b, a != -b */
    binop(x, y, z, |xi, yi| (a * xi) + (b * yi));
}

pub fn N_VConst_OpenMP(c: sunrealtype, z: &N_Vector) {
    let n = NV_LENGTH_OMP(z) as usize;
    let mut zd = NV_DATA_OMP(z);
    for i in 0..n {
        zd[i] = c;
    }
}

pub fn N_VProd_OpenMP(x: &N_Vector, y: &N_Vector, z: &N_Vector) {
    binop(x, y, z, |xi, yi| xi * yi);
}

pub fn N_VDiv_OpenMP(x: &N_Vector, y: &N_Vector, z: &N_Vector) {
    binop(x, y, z, |xi, yi| xi / yi);
}

pub fn N_VScale_OpenMP(c: sunrealtype, x: &N_Vector, z: &N_Vector) {
    if same(z, x) {
        /* BLAS usage: scale x <- cx */
        VScaleBy_OpenMP(c, x);
        return;
    }

    if c == ONE {
        VCopy_OpenMP(x, z);
    } else if c == -ONE {
        VNeg_OpenMP(x, z);
    } else {
        unop(x, z, |xi| c * xi);
    }
}

pub fn N_VAbs_OpenMP(x: &N_Vector, z: &N_Vector) {
    unop(x, z, SUNRabs);
}

pub fn N_VInv_OpenMP(x: &N_Vector, z: &N_Vector) {
    unop(x, z, |xi| ONE / xi);
}

pub fn N_VAddConst_OpenMP(x: &N_Vector, b: sunrealtype, z: &N_Vector) {
    unop(x, z, |xi| xi + b);
}

/// C `N_VDotProd_OpenMP` — `reduction(+ : sum) schedule(static)`.
pub fn N_VDotProd_OpenMP(x: &N_Vector, y: &N_Vector) -> sunrealtype {
    let n = NV_LENGTH_OMP(x) as usize;
    let nt = NV_NUM_THREADS_OMP(x);
    let xd = NV_DATA_OMP(x);
    if same(x, y) {
        omp_sum_static(n, nt, |i| xd[i] * xd[i])
    } else {
        let yd = NV_DATA_OMP(y);
        omp_sum_static(n, nt, |i| xd[i] * yd[i])
    }
}

pub fn N_VMaxNorm_OpenMP(x: &N_Vector) -> sunrealtype {
    let n = NV_LENGTH_OMP(x) as usize;
    let mut max = ZERO;
    let xd = NV_DATA_OMP(x);
    for i in 0..n {
        if SUNRabs(xd[i]) > max {
            max = SUNRabs(xd[i]);
        }
    }
    max
}

pub fn N_VWrmsNorm_OpenMP(x: &N_Vector, w: &N_Vector) -> sunrealtype {
    let norm = N_VWSqrSumLocal_OpenMP(x, w);
    SUNRsqrt(norm / NV_LENGTH_OMP(x) as sunrealtype)
}

/// C `N_VWSqrSumLocal_OpenMP` — `reduction(+ : sum) schedule(static)`.
pub fn N_VWSqrSumLocal_OpenMP(x: &N_Vector, w: &N_Vector) -> sunrealtype {
    let n = NV_LENGTH_OMP(x) as usize;
    let nt = NV_NUM_THREADS_OMP(x);
    let xd = NV_DATA_OMP(x);
    if same(x, w) {
        omp_sum_static(n, nt, |i| SUNSQR(xd[i] * xd[i]))
    } else {
        let wd = NV_DATA_OMP(w);
        omp_sum_static(n, nt, |i| SUNSQR(xd[i] * wd[i]))
    }
}

pub fn N_VWrmsNormMask_OpenMP(x: &N_Vector, w: &N_Vector, id: &N_Vector) -> sunrealtype {
    let norm = N_VWSqrSumMaskLocal_OpenMP(x, w, id);
    SUNRsqrt(norm / NV_LENGTH_OMP(x) as sunrealtype)
}

/// C `N_VWSqrSumMaskLocal_OpenMP` — `reduction(+ : sum) schedule(static)`.
pub fn N_VWSqrSumMaskLocal_OpenMP(x: &N_Vector, w: &N_Vector, id: &N_Vector) -> sunrealtype {
    let n = NV_LENGTH_OMP(x) as usize;
    let nt = NV_NUM_THREADS_OMP(x);
    let xd = NV_DATA_OMP(x);
    let idd = NV_DATA_OMP(id);
    if same(x, w) {
        omp_sum_static(n, nt, |i| {
            if idd[i] > ZERO {
                SUNSQR(xd[i] * xd[i])
            } else {
                ZERO
            }
        })
    } else {
        let wd = NV_DATA_OMP(w);
        omp_sum_static(n, nt, |i| {
            if idd[i] > ZERO {
                SUNSQR(xd[i] * wd[i])
            } else {
                ZERO
            }
        })
    }
}

pub fn N_VMin_OpenMP(x: &N_Vector) -> sunrealtype {
    let n = NV_LENGTH_OMP(x) as usize;
    let xd = NV_DATA_OMP(x);
    let mut min = xd[0];
    for i in 1..n {
        if xd[i] < min {
            min = xd[i];
        }
    }
    min
}

/// C `N_VWL2Norm_OpenMP` — `reduction(+ : sum) schedule(static)`.
pub fn N_VWL2Norm_OpenMP(x: &N_Vector, w: &N_Vector) -> sunrealtype {
    let n = NV_LENGTH_OMP(x) as usize;
    let nt = NV_NUM_THREADS_OMP(x);
    let xd = NV_DATA_OMP(x);
    let sum = if same(x, w) {
        omp_sum_static(n, nt, |i| SUNSQR(xd[i] * xd[i]))
    } else {
        let wd = NV_DATA_OMP(w);
        omp_sum_static(n, nt, |i| SUNSQR(xd[i] * wd[i]))
    };
    SUNRsqrt(sum)
}

/// C `N_VL1Norm_OpenMP` — `reduction(+ : sum) schedule(static)`.
pub fn N_VL1Norm_OpenMP(x: &N_Vector) -> sunrealtype {
    let n = NV_LENGTH_OMP(x) as usize;
    let nt = NV_NUM_THREADS_OMP(x);
    let xd = NV_DATA_OMP(x);
    omp_sum_static(n, nt, |i| SUNRabs(xd[i]))
}

pub fn N_VCompare_OpenMP(c: sunrealtype, x: &N_Vector, z: &N_Vector) {
    unop(x, z, |xi| if SUNRabs(xi) >= c { ONE } else { ZERO });
}

pub fn N_VInvTest_OpenMP(x: &N_Vector, z: &N_Vector) -> sunbooleantype {
    let n = NV_LENGTH_OMP(x) as usize;
    let mut no_zero_found = SUNTRUE;
    if same(x, z) {
        let mut zd = NV_DATA_OMP(z);
        for i in 0..n {
            if zd[i] == ZERO {
                no_zero_found = SUNFALSE;
            } else {
                zd[i] = ONE / zd[i];
            }
        }
    } else {
        let xd = NV_DATA_OMP(x);
        let mut zd = NV_DATA_OMP(z);
        for i in 0..n {
            if xd[i] == ZERO {
                no_zero_found = SUNFALSE;
            } else {
                zd[i] = ONE / xd[i];
            }
        }
    }
    no_zero_found
}

pub fn N_VConstrMask_OpenMP(c: &N_Vector, x: &N_Vector, m: &N_Vector) -> sunbooleantype {
    let n = NV_LENGTH_OMP(x) as usize;
    let mut temp = ZERO;

    /* m never aliases c or x on any in-scope call path; the C code would
    read the freshly-zeroed mask in that case, which the branches below
    reproduce if it ever happens. */
    let mc = same(m, c);
    let mx = same(m, x);
    let cx = same(c, x);

    let mut md = NV_DATA_OMP(m);
    let cd = if mc { None } else { Some(NV_DATA_OMP(c)) };
    let xd = if mx {
        None
    } else if cx && cd.is_some() {
        None
    } else {
        Some(NV_DATA_OMP(x))
    };

    for i in 0..n {
        md[i] = ZERO;

        let cdi = match &cd {
            Some(cd) => cd[i],
            None => md[i],
        };

        /* Continue if no constraints were set for the variable */
        if cdi == ZERO {
            continue;
        }

        let xdi = if mx {
            md[i]
        } else {
            match &xd {
                Some(xd) => xd[i],
                None => cdi,
            }
        };

        /* Check if a set constraint has been violated */
        let test = (SUNRabs(cdi) > ONEPT5 && xdi * cdi <= ZERO)
            || (SUNRabs(cdi) > HALF && xdi * cdi < ZERO);
        if test {
            temp = ONE;
            md[i] = ONE;
        }
    }

    /* Return false if any constraint was violated */
    if temp == ONE {
        SUNFALSE
    } else {
        SUNTRUE
    }
}

pub fn N_VMinQuotient_OpenMP(num: &N_Vector, denom: &N_Vector) -> sunrealtype {
    let n = NV_LENGTH_OMP(num) as usize;
    let nd = NV_DATA_OMP(num);
    let dd = if same(num, denom) {
        None
    } else {
        Some(NV_DATA_OMP(denom))
    };
    let mut not_even_once = SUNTRUE;
    let mut min = SUN_BIG_REAL;

    for i in 0..n {
        let ddi = match &dd {
            Some(dd) => dd[i],
            None => nd[i],
        };
        if ddi == ZERO {
            continue;
        } else if !not_even_once {
            min = SUNMIN(min, nd[i] / ddi);
        } else {
            min = nd[i] / ddi;
            not_even_once = SUNFALSE;
        }
    }

    min
}

/*
 * -----------------------------------------------------------------
 * fused vector operations
 * -----------------------------------------------------------------
 */

pub fn N_VLinearCombination_OpenMP(
    nvec: i32,
    c: &[sunrealtype],
    X: &[N_Vector],
    z: &N_Vector,
) -> SUNErrCode {
    /* should have called N_VScale */
    if nvec == 1 {
        N_VScale_OpenMP(c[0], &X[0], z);
        return SUN_SUCCESS;
    }

    /* should have called N_VLinearSum */
    if nvec == 2 {
        N_VLinearSum_OpenMP(c[0], &X[0], c[1], &X[1], z);
        return SUN_SUCCESS;
    }

    let n = NV_LENGTH_OMP(z) as usize;

    /*
     * X[0] += c[i]*X[i], i = 1,...,nvec-1
     */
    if same(&X[0], z) && (c[0] == ONE) {
        let mut zd = NV_DATA_OMP(z);
        for i in 1..nvec as usize {
            if same(&X[i], z) {
                for j in 0..n {
                    zd[j] += c[i] * zd[j];
                }
            } else {
                let xd = NV_DATA_OMP(&X[i]);
                for j in 0..n {
                    zd[j] += c[i] * xd[j];
                }
            }
        }
        return SUN_SUCCESS;
    }

    /*
     * X[0] = c[0] * X[0] + sum{ c[i] * X[i] }, i = 1,...,nvec-1
     */
    if same(&X[0], z) {
        let mut zd = NV_DATA_OMP(z);
        for j in 0..n {
            zd[j] *= c[0];
        }
        for i in 1..nvec as usize {
            if same(&X[i], z) {
                for j in 0..n {
                    zd[j] += c[i] * zd[j];
                }
            } else {
                let xd = NV_DATA_OMP(&X[i]);
                for j in 0..n {
                    zd[j] += c[i] * xd[j];
                }
            }
        }
        return SUN_SUCCESS;
    }

    /*
     * z = sum{ c[i] * X[i] }, i = 0,...,nvec-1
     */
    {
        let mut zd = NV_DATA_OMP(z);
        {
            let xd = NV_DATA_OMP(&X[0]);
            for j in 0..n {
                zd[j] = c[0] * xd[j];
            }
        }
        for i in 1..nvec as usize {
            if same(&X[i], z) {
                for j in 0..n {
                    zd[j] += c[i] * zd[j];
                }
            } else {
                let xd = NV_DATA_OMP(&X[i]);
                for j in 0..n {
                    zd[j] += c[i] * xd[j];
                }
            }
        }
    }
    SUN_SUCCESS
}

pub fn N_VScaleAddMulti_OpenMP(
    nvec: i32,
    a: &[sunrealtype],
    x: &N_Vector,
    Y: &[N_Vector],
    Z: &[N_Vector],
) -> SUNErrCode {
    /* should have called N_VLinearSum */
    if nvec == 1 {
        N_VLinearSum_OpenMP(a[0], x, ONE, &Y[0], &Z[0]);
        return SUN_SUCCESS;
    }

    let n = NV_LENGTH_OMP(x) as usize;

    /*
     * Y[i][j] += a[i] * x[j]  (C tests array-pointer equality Y == Z)
     */
    if std::ptr::eq(Y.as_ptr(), Z.as_ptr()) {
        let xd = NV_DATA_OMP(x);
        for i in 0..nvec as usize {
            let mut yd = NV_DATA_OMP(&Y[i]);
            for j in 0..n {
                yd[j] += a[i] * xd[j];
            }
        }
        return SUN_SUCCESS;
    }

    /*
     * Z[i][j] = Y[i][j] + a[i] * x[j]
     */
    let xd = NV_DATA_OMP(x);
    for i in 0..nvec as usize {
        if same(&Y[i], &Z[i]) {
            let mut zd = NV_DATA_OMP(&Z[i]);
            for j in 0..n {
                zd[j] = a[i] * xd[j] + zd[j];
            }
        } else {
            let yd = NV_DATA_OMP(&Y[i]);
            let mut zd = NV_DATA_OMP(&Z[i]);
            for j in 0..n {
                zd[j] = a[i] * xd[j] + yd[j];
            }
        }
    }
    SUN_SUCCESS
}

pub fn N_VDotProdMulti_OpenMP(
    nvec: i32,
    x: &N_Vector,
    Y: &[N_Vector],
    dotprods: &mut [sunrealtype],
) -> SUNErrCode {
    /* should have called N_VDotProd */
    if nvec == 1 {
        dotprods[0] = N_VDotProd_OpenMP(x, &Y[0]);
        return SUN_SUCCESS;
    }

    let n = NV_LENGTH_OMP(x) as usize;
    let xd = NV_DATA_OMP(x);

    /* compute multiple dot products */
    for i in 0..nvec as usize {
        dotprods[i] = ZERO;
        if same(x, &Y[i]) {
            for j in 0..n {
                dotprods[i] += xd[j] * xd[j];
            }
        } else {
            let yd = NV_DATA_OMP(&Y[i]);
            for j in 0..n {
                dotprods[i] += xd[j] * yd[j];
            }
        }
    }

    SUN_SUCCESS
}

/*
 * -----------------------------------------------------------------
 * vector array operations
 * -----------------------------------------------------------------
 */

/// C array-pointer equality test (`X == Z` on `N_Vector*` arguments).
fn same_array(a: &[N_Vector], b: &[N_Vector]) -> bool {
    std::ptr::eq(a.as_ptr(), b.as_ptr())
}

pub fn N_VLinearSumVectorArray_OpenMP(
    nvec: i32,
    a: sunrealtype,
    X: &[N_Vector],
    b: sunrealtype,
    Y: &[N_Vector],
    Z: &[N_Vector],
) -> SUNErrCode {
    /* should have called N_VLinearSum */
    if nvec == 1 {
        N_VLinearSum_OpenMP(a, &X[0], b, &Y[0], &Z[0]);
        return SUN_SUCCESS;
    }

    /* BLAS usage: axpy y <- ax+y */
    if (b == ONE) && same_array(Z, Y) {
        VaxpyVectorArray_OpenMP(nvec, a, X, Y);
        return SUN_SUCCESS;
    }

    /* BLAS usage: axpy x <- by+x */
    if (a == ONE) && same_array(Z, X) {
        VaxpyVectorArray_OpenMP(nvec, b, Y, X);
        return SUN_SUCCESS;
    }

    /* Case: a == b == 1.0 */
    if (a == ONE) && (b == ONE) {
        VSumVectorArray_OpenMP(nvec, X, Y, Z);
        return SUN_SUCCESS;
    }

    /* Cases: (1) a == 1.0, b = -1.0, (2) a == -1.0, b == 1.0 */
    let test = (a == ONE) && (b == -ONE);
    if test || ((a == -ONE) && (b == ONE)) {
        let v1 = if test { Y } else { X };
        let v2 = if test { X } else { Y };
        VDiffVectorArray_OpenMP(nvec, v2, v1, Z);
        return SUN_SUCCESS;
    }

    /* Cases: (1) a == 1.0, b == other or 0.0, (2) a == other or 0.0, b == 1.0 */
    let test = a == ONE;
    if test || (b == ONE) {
        let c = if test { b } else { a };
        let v1 = if test { Y } else { X };
        let v2 = if test { X } else { Y };
        VLin1VectorArray_OpenMP(nvec, c, v1, v2, Z);
        return SUN_SUCCESS;
    }

    /* Cases: (1) a == -1.0, b != 1.0, (2) a != 1.0, b == -1.0 */
    let test = a == -ONE;
    if test || (b == -ONE) {
        let c = if test { b } else { a };
        let v1 = if test { Y } else { X };
        let v2 = if test { X } else { Y };
        VLin2VectorArray_OpenMP(nvec, c, v1, v2, Z);
        return SUN_SUCCESS;
    }

    /* Case: a == b */
    if a == b {
        VScaleSumVectorArray_OpenMP(nvec, a, X, Y, Z);
        return SUN_SUCCESS;
    }

    /* Case: a == -b */
    if a == -b {
        VScaleDiffVectorArray_OpenMP(nvec, a, X, Y, Z);
        return SUN_SUCCESS;
    }

    /* compute linear sum for each vector pair in vector arrays */
    for i in 0..nvec as usize {
        binop(&X[i], &Y[i], &Z[i], |xj, yj| a * xj + b * yj);
    }

    SUN_SUCCESS
}

pub fn N_VScaleVectorArray_OpenMP(
    nvec: i32,
    c: &[sunrealtype],
    X: &[N_Vector],
    Z: &[N_Vector],
) -> SUNErrCode {
    /* should have called N_VScale */
    if nvec == 1 {
        N_VScale_OpenMP(c[0], &X[0], &Z[0]);
        return SUN_SUCCESS;
    }

    let n = NV_LENGTH_OMP(&Z[0]) as usize;

    /*
     * X[i] *= c[i]
     */
    if same_array(X, Z) {
        for i in 0..nvec as usize {
            let mut xd = NV_DATA_OMP(&X[i]);
            for j in 0..n {
                xd[j] *= c[i];
            }
        }
        return SUN_SUCCESS;
    }

    /*
     * Z[i] = c[i] * X[i]
     */
    for i in 0..nvec as usize {
        if same(&X[i], &Z[i]) {
            let mut zd = NV_DATA_OMP(&Z[i]);
            for j in 0..n {
                zd[j] = c[i] * zd[j];
            }
        } else {
            let xd = NV_DATA_OMP(&X[i]);
            let mut zd = NV_DATA_OMP(&Z[i]);
            for j in 0..n {
                zd[j] = c[i] * xd[j];
            }
        }
    }
    SUN_SUCCESS
}

pub fn N_VConstVectorArray_OpenMP(nvec: i32, c: sunrealtype, Z: &[N_Vector]) -> SUNErrCode {
    /* should have called N_VConst */
    if nvec == 1 {
        N_VConst_OpenMP(c, &Z[0]);
        return SUN_SUCCESS;
    }

    let n = NV_LENGTH_OMP(&Z[0]) as usize;

    /* set each vector in the vector array to a constant */
    for i in 0..nvec as usize {
        let mut zd = NV_DATA_OMP(&Z[i]);
        for j in 0..n {
            zd[j] = c;
        }
    }

    SUN_SUCCESS
}

pub fn N_VWrmsNormVectorArray_OpenMP(
    nvec: i32,
    X: &[N_Vector],
    W: &[N_Vector],
    nrm: &mut [sunrealtype],
) -> SUNErrCode {
    /* should have called N_VWrmsNorm */
    if nvec == 1 {
        nrm[0] = N_VWrmsNorm_OpenMP(&X[0], &W[0]);
        return SUN_SUCCESS;
    }

    let n = NV_LENGTH_OMP(&X[0]) as usize;

    /* compute the WRMS norm for each vector in the vector array */
    for i in 0..nvec as usize {
        let xd = NV_DATA_OMP(&X[i]);
        let wd = if same(&X[i], &W[i]) {
            None
        } else {
            Some(NV_DATA_OMP(&W[i]))
        };
        nrm[i] = ZERO;
        match &wd {
            Some(wd) => {
                for j in 0..n {
                    nrm[i] += SUNSQR(xd[j] * wd[j]);
                }
            }
            None => {
                for j in 0..n {
                    nrm[i] += SUNSQR(xd[j] * xd[j]);
                }
            }
        }
        nrm[i] = SUNRsqrt(nrm[i] / n as sunrealtype);
    }

    SUN_SUCCESS
}

pub fn N_VWrmsNormMaskVectorArray_OpenMP(
    nvec: i32,
    X: &[N_Vector],
    W: &[N_Vector],
    id: &N_Vector,
    nrm: &mut [sunrealtype],
) -> SUNErrCode {
    /* should have called N_VWrmsNorm */
    if nvec == 1 {
        nrm[0] = N_VWrmsNormMask_OpenMP(&X[0], &W[0], id);
        return SUN_SUCCESS;
    }

    let n = NV_LENGTH_OMP(&X[0]) as usize;
    let idd = NV_DATA_OMP(id);

    /* compute the WRMS norm for each vector in the vector array */
    for i in 0..nvec as usize {
        let xd = if same(&X[i], id) {
            None
        } else {
            Some(NV_DATA_OMP(&X[i]))
        };
        let wd = if same(&W[i], id) || same(&W[i], &X[i]) {
            None
        } else {
            Some(NV_DATA_OMP(&W[i]))
        };
        nrm[i] = ZERO;
        for j in 0..n {
            let xj = match &xd {
                Some(xd) => xd[j],
                None => idd[j],
            };
            let wj = match &wd {
                Some(wd) => wd[j],
                None => {
                    if same(&W[i], id) {
                        idd[j]
                    } else {
                        xj
                    }
                }
            };
            if idd[j] > ZERO {
                nrm[i] += SUNSQR(xj * wj);
            }
        }
        nrm[i] = SUNRsqrt(nrm[i] / n as sunrealtype);
    }

    SUN_SUCCESS
}

pub fn N_VScaleAddMultiVectorArray_OpenMP(
    nvec: i32,
    nsum: i32,
    a: &[sunrealtype],
    X: &[N_Vector],
    Y: &[Vec<N_Vector>],
    Z: &[Vec<N_Vector>],
) -> SUNErrCode {
    /* ---------------------------
     * Special cases for nvec == 1
     * --------------------------- */

    if nvec == 1 {
        /* should have called N_VLinearSum */
        if nsum == 1 {
            N_VLinearSum_OpenMP(a[0], &X[0], ONE, &Y[0][0], &Z[0][0]);
            return SUN_SUCCESS;
        }

        /* should have called N_VScaleAddMulti */
        let YY: Vec<N_Vector> = (0..nsum as usize).map(|j| Y[j][0].clone()).collect();
        let ZZ: Vec<N_Vector> = (0..nsum as usize).map(|j| Z[j][0].clone()).collect();
        let ier = N_VScaleAddMulti_OpenMP(nsum, a, &X[0], &YY, &ZZ);
        if ier != SUN_SUCCESS {
            return ier;
        }
        return SUN_SUCCESS;
    }

    /* --------------------------
     * Special cases for nvec > 1
     * -------------------------- */

    /* should have called N_VLinearSumVectorArray */
    if nsum == 1 {
        let ier = N_VLinearSumVectorArray_OpenMP(nvec, a[0], X, ONE, &Y[0], &Z[0]);
        if ier != SUN_SUCCESS {
            return ier;
        }
        return SUN_SUCCESS;
    }

    /* ----------------------------
     * Compute multiple linear sums
     * ---------------------------- */

    let n = NV_LENGTH_OMP(&X[0]) as usize;

    /*
     * Y[i][j] += a[i] * x[j]  (C tests array-pointer equality Y == Z)
     */
    if std::ptr::eq(Y.as_ptr(), Z.as_ptr()) {
        for i in 0..nvec as usize {
            let xd = NV_DATA_OMP(&X[i]);
            for j in 0..nsum as usize {
                let mut yd = NV_DATA_OMP(&Y[j][i]);
                for k in 0..n {
                    yd[k] += a[j] * xd[k];
                }
            }
        }
        return SUN_SUCCESS;
    }

    /*
     * Z[i][j] = Y[i][j] + a[i] * x[j]
     */
    for i in 0..nvec as usize {
        let xd = NV_DATA_OMP(&X[i]);
        for j in 0..nsum as usize {
            if same(&Y[j][i], &Z[j][i]) {
                let mut zd = NV_DATA_OMP(&Z[j][i]);
                for k in 0..n {
                    zd[k] = a[j] * xd[k] + zd[k];
                }
            } else {
                let yd = NV_DATA_OMP(&Y[j][i]);
                let mut zd = NV_DATA_OMP(&Z[j][i]);
                for k in 0..n {
                    zd[k] = a[j] * xd[k] + yd[k];
                }
            }
        }
    }
    SUN_SUCCESS
}

pub fn N_VLinearCombinationVectorArray_OpenMP(
    nvec: i32,
    nsum: i32,
    c: &[sunrealtype],
    X: &[Vec<N_Vector>],
    Z: &[N_Vector],
) -> SUNErrCode {
    /* ---------------------------
     * Special cases for nvec == 1
     * --------------------------- */

    if nvec == 1 {
        /* should have called N_VScale */
        if nsum == 1 {
            N_VScale_OpenMP(c[0], &X[0][0], &Z[0]);
            return SUN_SUCCESS;
        }

        /* should have called N_VLinearSum */
        if nsum == 2 {
            N_VLinearSum_OpenMP(c[0], &X[0][0], c[1], &X[1][0], &Z[0]);
            return SUN_SUCCESS;
        }

        /* should have called N_VLinearCombination */
        let Y: Vec<N_Vector> = (0..nsum as usize).map(|i| X[i][0].clone()).collect();
        let ier = N_VLinearCombination_OpenMP(nsum, c, &Y, &Z[0]);
        if ier != SUN_SUCCESS {
            return ier;
        }
        return SUN_SUCCESS;
    }

    /* --------------------------
     * Special cases for nvec > 1
     * -------------------------- */

    /* should have called N_VScaleVectorArray */
    if nsum == 1 {
        let ctmp: Vec<sunrealtype> = vec![c[0]; nvec as usize];
        let ier = N_VScaleVectorArray_OpenMP(nvec, &ctmp, &X[0], Z);
        if ier != SUN_SUCCESS {
            return ier;
        }
        return SUN_SUCCESS;
    }

    /* should have called N_VLinearSumVectorArray */
    if nsum == 2 {
        let ier = N_VLinearSumVectorArray_OpenMP(nvec, c[0], &X[0], c[1], &X[1], Z);
        if ier != SUN_SUCCESS {
            return ier;
        }
        return SUN_SUCCESS;
    }

    /* --------------------------
     * Compute linear combination
     * -------------------------- */

    let n = NV_LENGTH_OMP(&Z[0]) as usize;

    /*
     * X[0][j] += c[i]*X[i][j], i = 1,...,nvec-1
     */
    if same_array(&X[0], Z) && (c[0] == ONE) {
        for j in 0..nvec as usize {
            let mut zd = NV_DATA_OMP(&Z[j]);
            for i in 1..nsum as usize {
                if same(&X[i][j], &Z[j]) {
                    for k in 0..n {
                        zd[k] += c[i] * zd[k];
                    }
                } else {
                    let xd = NV_DATA_OMP(&X[i][j]);
                    for k in 0..n {
                        zd[k] += c[i] * xd[k];
                    }
                }
            }
        }
        return SUN_SUCCESS;
    }

    /*
     * X[0][j] = c[0] * X[0][j] + sum{ c[i] * X[i][j] }, i = 1,...,nvec-1
     */
    if same_array(&X[0], Z) {
        for j in 0..nvec as usize {
            let mut zd = NV_DATA_OMP(&Z[j]);
            for k in 0..n {
                zd[k] *= c[0];
            }
            for i in 1..nsum as usize {
                if same(&X[i][j], &Z[j]) {
                    for k in 0..n {
                        zd[k] += c[i] * zd[k];
                    }
                } else {
                    let xd = NV_DATA_OMP(&X[i][j]);
                    for k in 0..n {
                        zd[k] += c[i] * xd[k];
                    }
                }
            }
        }
        return SUN_SUCCESS;
    }

    /*
     * Z[j] = sum{ c[i] * X[i][j] }, i = 0,...,nvec-1
     */
    for j in 0..nvec as usize {
        {
            if same(&X[0][j], &Z[j]) {
                let mut zd = NV_DATA_OMP(&Z[j]);
                for k in 0..n {
                    zd[k] = c[0] * zd[k];
                }
            } else {
                let xd = NV_DATA_OMP(&X[0][j]);
                let mut zd = NV_DATA_OMP(&Z[j]);
                for k in 0..n {
                    zd[k] = c[0] * xd[k];
                }
            }
        }
        let mut zd = NV_DATA_OMP(&Z[j]);
        for i in 1..nsum as usize {
            if same(&X[i][j], &Z[j]) {
                for k in 0..n {
                    zd[k] += c[i] * zd[k];
                }
            } else {
                let xd = NV_DATA_OMP(&X[i][j]);
                for k in 0..n {
                    zd[k] += c[i] * xd[k];
                }
            }
        }
    }
    SUN_SUCCESS
}

/*
 * -----------------------------------------------------------------
 * OPTIONAL XBraid interface operations
 * -----------------------------------------------------------------
 */

pub fn N_VBufSize_OpenMP(x: &N_Vector, size: &mut sunindextype) -> SUNErrCode {
    *size = NV_LENGTH_OMP(x) * (std::mem::size_of::<sunrealtype>() as sunindextype);
    SUN_SUCCESS
}

pub fn N_VBufPack_OpenMP(x: &N_Vector, buf: &mut [sunrealtype]) -> SUNErrCode {
    let n = NV_LENGTH_OMP(x) as usize;
    let xd = NV_DATA_OMP(x);
    buf[..n].copy_from_slice(&xd[..n]);
    SUN_SUCCESS
}

pub fn N_VBufUnpack_OpenMP(x: &N_Vector, buf: &[sunrealtype]) -> SUNErrCode {
    let n = NV_LENGTH_OMP(x) as usize;
    let mut xd = NV_DATA_OMP(x);
    xd[..n].copy_from_slice(&buf[..n]);
    SUN_SUCCESS
}

/*
 * -----------------------------------------------------------------
 * private functions for special cases of vector operations
 * -----------------------------------------------------------------
 */

fn VCopy_OpenMP(x: &N_Vector, z: &N_Vector) {
    unop(x, z, |xi| xi);
}

fn VSum_OpenMP(x: &N_Vector, y: &N_Vector, z: &N_Vector) {
    binop(x, y, z, |xi, yi| xi + yi);
}

fn VDiff_OpenMP(x: &N_Vector, y: &N_Vector, z: &N_Vector) {
    binop(x, y, z, |xi, yi| xi - yi);
}

fn VNeg_OpenMP(x: &N_Vector, z: &N_Vector) {
    unop(x, z, |xi| -xi);
}

fn VScaleSum_OpenMP(c: sunrealtype, x: &N_Vector, y: &N_Vector, z: &N_Vector) {
    binop(x, y, z, |xi, yi| c * (xi + yi));
}

fn VScaleDiff_OpenMP(c: sunrealtype, x: &N_Vector, y: &N_Vector, z: &N_Vector) {
    binop(x, y, z, |xi, yi| c * (xi - yi));
}

fn VLin1_OpenMP(a: sunrealtype, x: &N_Vector, y: &N_Vector, z: &N_Vector) {
    binop(x, y, z, |xi, yi| (a * xi) + yi);
}

fn VLin2_OpenMP(a: sunrealtype, x: &N_Vector, y: &N_Vector, z: &N_Vector) {
    binop(x, y, z, |xi, yi| (a * xi) - yi);
}

fn Vaxpy_OpenMP(a: sunrealtype, x: &N_Vector, y: &N_Vector) {
    let n = NV_LENGTH_OMP(x) as usize;
    if same(x, y) {
        let mut yd = NV_DATA_OMP(y);
        if a == ONE {
            for i in 0..n {
                let xi = yd[i];
                yd[i] += xi;
            }
            return;
        }
        if a == -ONE {
            for i in 0..n {
                let xi = yd[i];
                yd[i] -= xi;
            }
            return;
        }
        for i in 0..n {
            let xi = yd[i];
            yd[i] += a * xi;
        }
        return;
    }

    let xd = NV_DATA_OMP(x);
    let mut yd = NV_DATA_OMP(y);

    if a == ONE {
        for i in 0..n {
            yd[i] += xd[i];
        }
        return;
    }

    if a == -ONE {
        for i in 0..n {
            yd[i] -= xd[i];
        }
        return;
    }

    for i in 0..n {
        yd[i] += a * xd[i];
    }
}

fn VScaleBy_OpenMP(a: sunrealtype, x: &N_Vector) {
    let n = NV_LENGTH_OMP(x) as usize;
    let mut xd = NV_DATA_OMP(x);
    for i in 0..n {
        xd[i] *= a;
    }
}

/*
 * -----------------------------------------------------------------
 * private functions for special cases of vector array operations
 * -----------------------------------------------------------------
 */

fn VSumVectorArray_OpenMP(nvec: i32, X: &[N_Vector], Y: &[N_Vector], Z: &[N_Vector]) {
    for i in 0..nvec as usize {
        binop(&X[i], &Y[i], &Z[i], |xj, yj| xj + yj);
    }
}

fn VDiffVectorArray_OpenMP(nvec: i32, X: &[N_Vector], Y: &[N_Vector], Z: &[N_Vector]) {
    for i in 0..nvec as usize {
        binop(&X[i], &Y[i], &Z[i], |xj, yj| xj - yj);
    }
}

fn VScaleSumVectorArray_OpenMP(
    nvec: i32,
    c: sunrealtype,
    X: &[N_Vector],
    Y: &[N_Vector],
    Z: &[N_Vector],
) {
    for i in 0..nvec as usize {
        binop(&X[i], &Y[i], &Z[i], |xj, yj| c * (xj + yj));
    }
}

fn VScaleDiffVectorArray_OpenMP(
    nvec: i32,
    c: sunrealtype,
    X: &[N_Vector],
    Y: &[N_Vector],
    Z: &[N_Vector],
) {
    for i in 0..nvec as usize {
        binop(&X[i], &Y[i], &Z[i], |xj, yj| c * (xj - yj));
    }
}

fn VLin1VectorArray_OpenMP(
    nvec: i32,
    a: sunrealtype,
    X: &[N_Vector],
    Y: &[N_Vector],
    Z: &[N_Vector],
) {
    for i in 0..nvec as usize {
        binop(&X[i], &Y[i], &Z[i], |xj, yj| (a * xj) + yj);
    }
}

fn VLin2VectorArray_OpenMP(
    nvec: i32,
    a: sunrealtype,
    X: &[N_Vector],
    Y: &[N_Vector],
    Z: &[N_Vector],
) {
    for i in 0..nvec as usize {
        binop(&X[i], &Y[i], &Z[i], |xj, yj| (a * xj) - yj);
    }
}

fn VaxpyVectorArray_OpenMP(nvec: i32, a: sunrealtype, X: &[N_Vector], Y: &[N_Vector]) {
    for i in 0..nvec as usize {
        Vaxpy_OpenMP(a, &X[i], &Y[i]);
    }
}

/*
 * -----------------------------------------------------------------
 * Enable / Disable fused and vector array operations
 * -----------------------------------------------------------------
 */

pub fn N_VEnableFusedOps_OpenMP(v: &N_Vector, tf: sunbooleantype) -> SUNErrCode {
    let mut ops = v.ops.borrow_mut();
    if tf {
        /* enable all fused vector operations */
        ops.nvlinearcombination = Some(N_VLinearCombination_OpenMP);
        ops.nvscaleaddmulti = Some(N_VScaleAddMulti_OpenMP);
        ops.nvdotprodmulti = Some(N_VDotProdMulti_OpenMP);
        /* enable all vector array operations */
        ops.nvlinearsumvectorarray = Some(N_VLinearSumVectorArray_OpenMP);
        ops.nvscalevectorarray = Some(N_VScaleVectorArray_OpenMP);
        ops.nvconstvectorarray = Some(N_VConstVectorArray_OpenMP);
        ops.nvwrmsnormvectorarray = Some(N_VWrmsNormVectorArray_OpenMP);
        ops.nvwrmsnormmaskvectorarray = Some(N_VWrmsNormMaskVectorArray_OpenMP);
        ops.nvscaleaddmultivectorarray = Some(N_VScaleAddMultiVectorArray_OpenMP);
        ops.nvlinearcombinationvectorarray = Some(N_VLinearCombinationVectorArray_OpenMP);
        /* enable single buffer reduction operations */
        ops.nvdotprodmultilocal = Some(N_VDotProdMulti_OpenMP);
    } else {
        /* disable all fused vector operations */
        ops.nvlinearcombination = None;
        ops.nvscaleaddmulti = None;
        ops.nvdotprodmulti = None;
        /* disable all vector array operations */
        ops.nvlinearsumvectorarray = None;
        ops.nvscalevectorarray = None;
        ops.nvconstvectorarray = None;
        ops.nvwrmsnormvectorarray = None;
        ops.nvwrmsnormmaskvectorarray = None;
        ops.nvscaleaddmultivectorarray = None;
        ops.nvlinearcombinationvectorarray = None;
        /* disable single buffer reduction operations */
        ops.nvdotprodmultilocal = None;
    }
    SUN_SUCCESS
}

pub fn N_VEnableLinearCombination_OpenMP(v: &N_Vector, tf: sunbooleantype) -> SUNErrCode {
    v.ops.borrow_mut().nvlinearcombination = if tf {
        Some(N_VLinearCombination_OpenMP)
    } else {
        None
    };
    SUN_SUCCESS
}

pub fn N_VEnableScaleAddMulti_OpenMP(v: &N_Vector, tf: sunbooleantype) -> SUNErrCode {
    v.ops.borrow_mut().nvscaleaddmulti = if tf { Some(N_VScaleAddMulti_OpenMP) } else { None };
    SUN_SUCCESS
}

pub fn N_VEnableDotProdMulti_OpenMP(v: &N_Vector, tf: sunbooleantype) -> SUNErrCode {
    let mut ops = v.ops.borrow_mut();
    ops.nvdotprodmulti = if tf { Some(N_VDotProdMulti_OpenMP) } else { None };
    ops.nvdotprodmultilocal = if tf { Some(N_VDotProdMulti_OpenMP) } else { None };
    SUN_SUCCESS
}

pub fn N_VEnableLinearSumVectorArray_OpenMP(v: &N_Vector, tf: sunbooleantype) -> SUNErrCode {
    v.ops.borrow_mut().nvlinearsumvectorarray = if tf {
        Some(N_VLinearSumVectorArray_OpenMP)
    } else {
        None
    };
    SUN_SUCCESS
}

pub fn N_VEnableScaleVectorArray_OpenMP(v: &N_Vector, tf: sunbooleantype) -> SUNErrCode {
    v.ops.borrow_mut().nvscalevectorarray = if tf {
        Some(N_VScaleVectorArray_OpenMP)
    } else {
        None
    };
    SUN_SUCCESS
}

pub fn N_VEnableConstVectorArray_OpenMP(v: &N_Vector, tf: sunbooleantype) -> SUNErrCode {
    v.ops.borrow_mut().nvconstvectorarray = if tf {
        Some(N_VConstVectorArray_OpenMP)
    } else {
        None
    };
    SUN_SUCCESS
}

pub fn N_VEnableWrmsNormVectorArray_OpenMP(v: &N_Vector, tf: sunbooleantype) -> SUNErrCode {
    v.ops.borrow_mut().nvwrmsnormvectorarray = if tf {
        Some(N_VWrmsNormVectorArray_OpenMP)
    } else {
        None
    };
    SUN_SUCCESS
}

pub fn N_VEnableWrmsNormMaskVectorArray_OpenMP(v: &N_Vector, tf: sunbooleantype) -> SUNErrCode {
    v.ops.borrow_mut().nvwrmsnormmaskvectorarray = if tf {
        Some(N_VWrmsNormMaskVectorArray_OpenMP)
    } else {
        None
    };
    SUN_SUCCESS
}

pub fn N_VEnableScaleAddMultiVectorArray_OpenMP(v: &N_Vector, tf: sunbooleantype) -> SUNErrCode {
    v.ops.borrow_mut().nvscaleaddmultivectorarray = if tf {
        Some(N_VScaleAddMultiVectorArray_OpenMP)
    } else {
        None
    };
    SUN_SUCCESS
}

pub fn N_VEnableLinearCombinationVectorArray_OpenMP(
    v: &N_Vector,
    tf: sunbooleantype,
) -> SUNErrCode {
    v.ops.borrow_mut().nvlinearcombinationvectorarray = if tf {
        Some(N_VLinearCombinationVectorArray_OpenMP)
    } else {
        None
    };
    SUN_SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sundials_context::{SUNContext_Create, SUNContext};

    fn ctx() -> SUNContext {
        let mut c = None;
        assert_eq!(SUNContext_Create(0, &mut c), SUN_SUCCESS);
        c.expect("context created")
    }

    #[test]
    fn create_fill_ops() {
        let sunctx = ctx();
        let x = N_VNew_OpenMP(4, 2, &sunctx).expect("vector");
        assert_eq!(N_VGetLength(&x), 4);
        N_VConst(2.0, &x);
        let y = N_VClone(&x).expect("clone");
        N_VConst(3.0, &y);
        let z = N_VClone(&x).expect("clone");
        N_VLinearSum(1.0, &x, 1.0, &y, &z);
        assert_eq!(NV_DATA_OMP(&z)[0], 5.0);
        /* aliased: z = 2*z - x */
        N_VLinearSum(2.0, &z, -1.0, &x, &z);
        assert_eq!(NV_DATA_OMP(&z)[1], 8.0);
        assert_eq!(N_VDotProd(&x, &y), 24.0);
        assert_eq!(N_VMaxNorm(&z), 8.0);
        assert_eq!(N_VMin(&x), 2.0);
        /* wrms of constant vector v with weight w: |v*w| */
        let w = N_VClone(&x).expect("clone");
        N_VConst(0.5, &w);
        assert_eq!(N_VWrmsNorm(&x, &w), 1.0);
    }

    #[test]
    fn fused_and_arrays() {
        let sunctx = ctx();
        let x = N_VNew_OpenMP(3, 2, &sunctx).expect("vector");
        N_VEnableFusedOps_OpenMP(&x, SUNTRUE);
        N_VConst(1.0, &x);
        let ys = N_VCloneVectorArray(3, &x).expect("array");
        for (i, y) in ys.iter().enumerate() {
            N_VConst((i + 1) as sunrealtype, y);
        }
        let z = N_VClone(&x).expect("clone");
        let c = [1.0, 2.0, 3.0];
        assert_eq!(N_VLinearCombination(3, &c, &ys, &z), SUN_SUCCESS);
        /* 1*1 + 2*2 + 3*3 = 14 */
        assert_eq!(NV_DATA_OMP(&z)[0], 14.0);
    }
    /// `schedule(static)` splits `0..n` into `nthreads` contiguous chunks,
    /// the first `n % nthreads` of them one element longer. This is the
    /// split the C's OpenMP runtime uses, and reproducing it is what makes
    /// the port match the C bit for bit at a given thread count.
    #[test]
    fn static_schedule_chunking() {
        assert_eq!(static_chunks(10, 4), vec![(0, 3), (3, 6), (6, 8), (8, 10)]);
        assert_eq!(static_chunks(8, 4), vec![(0, 2), (2, 4), (4, 6), (6, 8)]);
        assert_eq!(static_chunks(5, 1), vec![(0, 5)]);
        /* every element is covered exactly once, for any split */
        for n in 0..40usize {
            for nt in 1..8usize {
                let cs = static_chunks(n, nt);
                assert_eq!(cs.first().map(|c| c.0).unwrap_or(0), 0);
                assert_eq!(cs.last().map(|c| c.1).unwrap_or(0), n);
                for w in cs.windows(2) {
                    assert_eq!(w[0].1, w[1].0);
                }
            }
        }
    }

    /// The thread count changes the answer, and that is the point: the
    /// partials are summed per chunk and then combined, so the rounding
    /// differs. A test that asserted independence would be asserting the
    /// port is wrong.
    #[test]
    fn sum_depends_on_thread_count_and_is_reproducible() {
        let n = 1000usize;
        let f = |i: usize| 1.0 / ((i + 1) as sunrealtype);
        let a = omp_sum_static(n, 1, f);
        let b = omp_sum_static(n, 4, f);
        assert_eq!(a, omp_sum_static(n, 1, f), "same count must give same bits");
        assert_eq!(b, omp_sum_static(n, 4, f), "same count must give same bits");
        assert_ne!(a.to_bits(), b.to_bits(), "different counts round differently");
        /* one thread is a straight sequential sum */
        let mut seq = ZERO;
        for i in 0..n {
            seq += f(i);
        }
        assert_eq!(a.to_bits(), seq.to_bits());
    }

}
