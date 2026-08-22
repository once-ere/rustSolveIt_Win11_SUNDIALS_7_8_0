//! `log1p` — double-precision log(1+x), host-independent.
//!
//! Translated from: glibc 2.39 `sysdeps/ieee754/dbl-64/s_log1p.c` (fdlibm-derived,
//! `@(#)s_log1p.c 5.1 93/09/24`, SunPro / Sun Microsystems, with the 1997
//! Naohiko Shimizu pipelining rewrite of the polynomial).
//!
//! On x86-64 glibc ifunc-dispatches `log1p` to `__log1p_fma`
//! (`sysdeps/x86_64/fpu/multiarch/s_log1p-fma.c`), i.e. the very same generic
//! source rebuilt with `-mfma -mavx2 -ffp-contract=fast`. So every `a*b + c`
//! that GCC's `widening_mul` pass is allowed to fuse *is* fused, and this
//! translation must use [`f64::mul_add`] in exactly those places and nowhere
//! else. The rule the pass applies, and which drives the map below, is that a
//! multiply is contracted into an add/sub only when **every** use of the
//! product is an add/sub **in the same basic block**
//! (`gimple_bb (use_stmt) != gimple_bb (mul_stmt)` bails out). Consequences,
//! each marked at its site:
//!
//! * `s * (hfsq + R)` — the one that is easy to get wrong, and the only one
//!   this module's 4-million-input oracle actually pins. It is written in
//!   *both* arms of the final `if (k == 0)`, so `-fcode-hoisting` (on at
//!   `-O2`, and it runs well before `widening_mul`) lifts it into the
//!   dominating block; both uses are then in other blocks and it stays an
//!   ordinary rounded multiply on both paths. Fusing it costs 2670 inputs.
//! * `R = hfsq * (1.0 - 2/3*f)` in the `|f| < 2**-20` path is **not** fused
//!   into the `f - R` / `R - (k*ln2_lo + c)` that follow: those two uses live
//!   in the two arms of `if (k == 0)`, different blocks from the multiply.
//!   The inner `1.0 - 0.666...*f` *is* fused — same block.
//! * In `R = R1 + z2*R2 + z4*R3 + z6*R4` the leading `R1 = z*Lp[1]` is
//!   emitted before `z2*R2`, so the pass reaches it first and consumes the
//!   first `+`; `z2*R2` is then left as a bare multiply because its only use
//!   has become an `.FMA` call rather than a `PLUS_EXPR`. The two remaining
//!   `+` fuse normally.
//! * `x - x*x*0.5` contracts on the *outer* multiply, which is exact
//!   (`*0.5`), so it is indistinguishable from the unfused form.
//!
//! Note this routine computes its logarithm inline; it does not call
//! [`crate::sundials_libm::log`]. The multi-precision fallback paths that
//! other glibc routines lost in 2.28 never existed here.
//!
//! SPDX-License-Identifier: LGPL-2.1-or-later

/* static const double ln2_hi, ln2_lo, two54, Lp[]  — s_log1p.c.
Bit patterns, not decimal literals, so nothing depends on the host's
decimal->binary conversion; they reproduce the hex words in the C comments
exactly. */
const LN2_HI: f64 = f64::from_bits(0x3fe6_2e42_fee0_0000); /* 6.93147180369123816490e-01 */
const LN2_LO: f64 = f64::from_bits(0x3dea_39ef_3579_3c76); /* 1.90821492927058770002e-10 */
const TWO54: f64 = f64::from_bits(0x4350_0000_0000_0000); /* 1.80143985094819840000e+16 */

/* Lp[1..7]; Lp[0] is the unused 0.0 of the C array. */
const LP: [u64; 8] = [
    0x0000_0000_0000_0000, /* 0.0                      */
    0x3fe5_5555_5555_5593, /* 6.666666666666735130e-01 */
    0x3fd9_9999_9997_fa04, /* 3.999999999940941908e-01 */
    0x3fd2_4924_9422_9359, /* 2.857142874366239149e-01 */
    0x3fcc_71c5_1d8e_78af, /* 2.222219843214978396e-01 */
    0x3fc7_4664_96cb_03de, /* 1.818357216161805012e-01 */
    0x3fc3_9a09_d078_c69f, /* 1.531383769920937332e-01 */
    0x3fc2_f112_df3e_5244, /* 1.479819860511658591e-01 */
];

/* The `0.66666666666666666' decimal literal of the |f| < 2**-20 path. */
const TWO_THIRDS: f64 = f64::from_bits(0x3fe5_5555_5555_5555);

/* static const double zero = 0.0; */
const ZERO: f64 = 0.0;

/// C `GET_HIGH_WORD (i, d)` — `math_private.h`.
#[inline]
fn get_high_word(d: f64) -> i32 {
    (d.to_bits() >> 32) as u32 as i32
}

/// C `SET_HIGH_WORD (d, v)` — keeps the low word, replaces the high one.
#[inline]
fn set_high_word(d: f64, v: i32) -> f64 {
    f64::from_bits((d.to_bits() & 0x0000_0000_ffff_ffff) | ((v as u32 as u64) << 32))
}

/// C `double __log1p (double x)` — glibc 2.39 `dbl-64/s_log1p.c`.
pub fn log1p(x: f64) -> f64 {
    /* double hfsq, f, c, s, z, R, u; int32_t k, hx, hu, ax;
    The C leaves f, c, hu uninitialised and relies on the control flow; Rust
    needs a value, so they start at 0 and every path still writes them
    before the first read. */
    let mut f = 0.0f64;
    let mut c = 0.0f64;
    let mut u: f64;
    let mut hu: i32 = 0;

    /* GET_HIGH_WORD (hx, x); ax = hx & 0x7fffffff; */
    let hx: i32 = get_high_word(x);
    let ax: i32 = hx & 0x7fff_ffff;

    let mut k: i32 = 1;
    if hx < 0x3FDA_827A {
        /* x < 0.41422 */
        if ax >= 0x3ff0_0000 {
            /* x <= -1.0 */
            if x == -1.0 {
                return -TWO54 / ZERO; /* log1p(-1) = -inf */
            } else {
                return (x - x) / (x - x); /* log1p(x<-1) = NaN */
            }
        }
        if ax < 0x3e20_0000 {
            /* |x| < 2**-29 */
            /* math_force_eval (two54 + x) — raise inexact, value discarded. */
            let _ = std::hint::black_box(TWO54 + x);
            if ax < 0x3c90_0000 {
                /* |x| < 2**-54 */
                /* math_check_force_underflow (x) — flag only, x unchanged. */
                return x;
            } else {
                /* `x - x*x*0.5'. GCC contracts the outer `*0.5', which is
                exact, so this is bit-identical either way. */
                return x - x * x * 0.5;
            }
        }
        if hx > 0 || hx <= 0xbfd2_bec3u32 as i32 {
            /* -0.2929 < x < 0.41422 */
            k = 0;
            f = x;
            hu = 1;
        }
    } else if hx >= 0x7ff0_0000 {
        return x + x; /* +inf, +NaN */
    }

    if k != 0 {
        /* Argument reduction: 1+x = 2^k * (1+f). */
        if hx < 0x4340_0000 {
            /* x < 2**53 */
            u = 1.0 + x;
            hu = get_high_word(u);
            k = (hu >> 20) - 1023;
            /* correction term c = (1+x) - u, exactly */
            c = if k > 0 { 1.0 - (u - x) } else { x - (u - 1.0) };
            c /= u;
        } else {
            u = x;
            hu = get_high_word(u);
            k = (hu >> 20) - 1023;
            c = 0.0;
        }
        hu &= 0x000f_ffff;
        if hu < 0x6_a09e {
            u = set_high_word(u, hu | 0x3ff0_0000); /* normalize u */
        } else {
            k += 1;
            u = set_high_word(u, hu | 0x3fe0_0000); /* normalize u/2 */
            hu = (0x0010_0000 - hu) >> 2;
        }
        f = u - 1.0;
    }

    /* hfsq = 0.5 * f * f;  — (0.5*f) is exact, so one rounding, no FMA. */
    let hfsq = 0.5 * f * f;

    if hu == 0 {
        /* |f| < 2**-20 */
        if f == ZERO {
            if k == 0 {
                return ZERO;
            } else {
                /* c += k*ln2_lo; return k*ln2_hi + c;  — both multiplies sit
                in this block with their adds, so both fuse. */
                let kd = k as f64;
                c = kd.mul_add(LN2_LO, c);
                return kd.mul_add(LN2_HI, c);
            }
        }
        /* R = hfsq * (1.0 - 0.66666666666666666 * f);
        The inner `1.0 - (2/3)*f' fuses (same block). The outer
        `hfsq * ...' does NOT fuse into the `f - R' / `R - ...' below:
        those uses are in the two arms of the following `if', i.e. other
        basic blocks, and GCC's widening_mul refuses to cross blocks. */
        let r_small = hfsq * (-TWO_THIRDS).mul_add(f, 1.0);
        if k == 0 {
            return f - r_small;
        } else {
            let kd = k as f64;
            /* k*ln2_hi - ((R - (k*ln2_lo + c)) - f) */
            let t = kd.mul_add(LN2_LO, c); /* k*ln2_lo + c  — fused */
            let t = (r_small - t) - f;
            return kd.mul_add(LN2_HI, -t); /* k*ln2_hi - t  — fused (FMS) */
        }
    }

    /* s = f/(2+f); R(z) with z = s*s, Shimizu's four-way split. */
    let s = f / (2.0 + f);
    let z = s * s;
    /* R1 = z*Lp[1]; R2 = Lp[2] + z*Lp[3]; R3 = Lp[4] + z*Lp[5];
    R4 = Lp[6] + z*Lp[7];  — the three `Lp[i] + z*Lp[i+1]' all fuse. */
    let z2 = z * z;
    let r2 = z.mul_add(f64::from_bits(LP[3]), f64::from_bits(LP[2]));
    let z4 = z2 * z2;
    let r3 = z.mul_add(f64::from_bits(LP[5]), f64::from_bits(LP[4]));
    let z6 = z4 * z2;
    let r4 = z.mul_add(f64::from_bits(LP[7]), f64::from_bits(LP[6]));
    /* R = R1 + z2*R2 + z4*R3 + z6*R4.
    `R1 = z*Lp[1]' is the earlier statement, so the pass claims the first
    `+' for it; `z2*R2' is then only reachable through an .FMA call and
    stays a bare multiply. The other two `+' fuse on their own products. */
    let t = z2 * r2;
    let t = z.mul_add(f64::from_bits(LP[1]), t);
    let t = z4.mul_add(r3, t);
    let r = z6.mul_add(r4, t);

    /* Both arms of the `if (k == 0)' below open with `s * (hfsq + R)', so
    -fcode-hoisting (on at -O2, and it runs long before widening_mul) lifts
    that multiply into this dominating block. Its two uses are then in other
    basic blocks, and widening_mul will not contract across a block — so
    `s * (hfsq + R)' is a plain rounded multiply on both paths, and only the
    surrounding adds/subs stay as they are. This is the one non-obvious
    fusion decision in the routine and it is worth 2670 of 4000000 inputs;
    x = -0x1.611aad03f10f4p-3 (k == 0) and x = -0x1.3cf7a8350c0d4p-2
    (k != 0) are the first two the oracle rejects if it is fused. */

    if k == 0 {
        /* f - (hfsq - s*(hfsq + R)) */
        f - (hfsq - s * (hfsq + r))
    } else {
        /* k*ln2_hi - ((hfsq - (s*(hfsq + R) + (k*ln2_lo + c))) - f)
        `k*ln2_lo + c' fuses and `k*ln2_hi - ...' fuses (both are local to
        this block); the hoisted `s*(hfsq + R)' does not. */
        let kd = k as f64;
        let t = kd.mul_add(LN2_LO, c);
        let t = s * (hfsq + r) + t;
        let t = (hfsq - t) - f;
        kd.mul_add(LN2_HI, -t)
    }
}
