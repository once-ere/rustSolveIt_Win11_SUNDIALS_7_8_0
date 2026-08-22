//! `exp` — double-precision e^x, host-independent.
//!
//! Translated from musl `src/math/exp.c` and `exp_data.c` — the ARM
//! optimized-routines implementation, Copyright (c) 2018 Arm Limited. glibc
//! ≥ 2.28 ships the same algorithm as `sysdeps/ieee754/dbl-64/e_exp.c`, and
//! on x86-64 ifunc-dispatches to `__exp_fma`, that source rebuilt with
//! `-mfma -mavx2 -ffp-contract=fast`. This translation reproduces that
//! FMA-contracted build.
//!
//! The tables, the reduction constants and `specialcase` are shared with the
//! already-measured `pow` in [`crate::sundials_math`], which runs the same
//! algorithm as its exponential tail — so the two cannot disagree about the
//! data, and the contraction map below is the one that was established there
//! (`POW_FMA_EXACTNESS.md`): the polynomial and the final `scale + scale*tmp`
//! are fused; the argument reduction `x + kd*hi + kd*lo` is not.
//!
//! SPDX-License-Identifier: MIT

use crate::sundials_math::{
    pow_exp_specialcase, pow_math_oflow, pow_math_uflow, pow_top12, EXP_INVLN2N, EXP_NEGLN2HIN,
    EXP_NEGLN2LON, EXP_POLY, EXP_SHIFT, EXP_TAB,
};

/// C `exp(double x)` — musl `src/math/exp.c`.
pub fn exp(x: f64) -> f64 {
    let mut abstop = pow_top12(x) & 0x7ff;
    /* top12(0x1p-54) = 0x3c9, top12(512.0) = 0x408,
    top12(1024.0) = 0x409, top12(INFINITY) = 0x7ff. */
    if abstop.wrapping_sub(0x3c9) >= 0x408u32.wrapping_sub(0x3c9) {
        if abstop.wrapping_sub(0x3c9) >= 0x8000_0000 {
            /* Avoid spurious underflow for tiny x. Note: 0 is common input. */
            return 1.0 + x; /* WANT_ROUNDING */
        }
        if abstop >= 0x409 {
            if x.to_bits() == f64::NEG_INFINITY.to_bits() {
                return 0.0;
            }
            if abstop >= 0x7ff {
                return 1.0 + x;
            }
            if x.to_bits() >> 63 != 0 {
                return pow_math_uflow(0);
            } else {
                return pow_math_oflow(0);
            }
        }
        /* Large x is special cased below. */
        abstop = 0;
    }

    /* exp(x) = 2^(k/N) * exp(r), with exp(r) in [2^(-1/2N),2^(1/2N)]. */
    /* x = ln2/N*k + r, with int k and r in [-ln2/2N, ln2/2N]. */
    let z = f64::from_bits(EXP_INVLN2N) * x;
    /* z - kd is in [-1, 1] in non-nearest rounding modes
    (TOINT_INTRINSICS = 0, EXP_USE_TOINT_NARROW = 0 path). */
    let shift = f64::from_bits(EXP_SHIFT);
    let mut kd = z + shift;
    let ki = kd.to_bits();
    kd -= shift;
    let r = x + kd * f64::from_bits(EXP_NEGLN2HIN) + kd * f64::from_bits(EXP_NEGLN2LON);
    /* 2^(k/N) ~= scale * (1 + tail). */
    let idx = (2 * (ki % 128)) as usize; /* EXP_TABLE_BITS = 7 */
    let top = ki << (52 - 7);
    let tail = f64::from_bits(EXP_TAB[idx]);
    /* This is only a valid scale when -1023*N < k < 1024*N. */
    let sbits = EXP_TAB[idx + 1].wrapping_add(top);
    /* exp(x) = 2^(k/N) * exp(r) ~= scale + scale * (tail + exp(r) - 1). */
    let r2 = r * r;
    let c2 = f64::from_bits(EXP_POLY[0]);
    let c3 = f64::from_bits(EXP_POLY[1]);
    let c4 = f64::from_bits(EXP_POLY[2]);
    let c5 = f64::from_bits(EXP_POLY[3]);
    let tmp = (r2 * r2).mul_add(r.mul_add(c5, c4), r2.mul_add(r.mul_add(c3, c2), tail + r));
    if abstop == 0 {
        /* This is `specialcase` from musl's exp.c. It is not translated
        again here: `pow`'s exponential tail runs the same algorithm and
        already carries a translation of the same function, so reusing it
        keeps one copy of the subnormal-range double-rounding logic instead
        of two that could drift apart.

        The two C bodies are textually identical; the only difference is
        that `pow`'s is reached with a `sign_bias` already folded into
        `sbits` by its caller, which is 0 on every path into it from here.
        That equivalence is also *measured*, not merely argued: the corpus
        opens with the exp overflow and underflow thresholds
        (0x1.62e42fefa39efp+9 and -0x1.74910d52d3051p+9, both sides of each,
        plus +-1024, +-DBL_MAX and the subnormal ladder), which are exactly
        the inputs that reach this branch, and all of them agree bit for bit
        with glibc. */
        return pow_exp_specialcase(tmp, sbits, ki);
    }
    let scale = f64::from_bits(sbits);
    /* Note: tmp == 0 or |tmp| > 2^-200 and scale > 2^-739, so there is no
    spurious underflow here even without fma. */
    scale.mul_add(tmp, scale)
}
