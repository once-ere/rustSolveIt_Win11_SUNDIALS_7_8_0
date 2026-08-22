//! `sinh`, `cosh`, `acosh` — double-precision hyperbolics, host-independent.
//!
//! Translated from: glibc 2.39 `sysdeps/ieee754/dbl-64/e_sinh.c`, `e_cosh.c`,
//! `e_acosh.c` (fdlibm-derived, `@(#)e_sinh.c 5.1 93/09/24`, SunPro / Sun
//! Microsystems).
//!
//! # No FMA contraction in this file
//!
//! These three are the exception to the rule that governs the rest of this
//! module. `sysdeps/x86_64/fpu/multiarch/` contains `e_exp-fma.c`,
//! `e_log-fma.c`, `s_expm1-fma.c`, `s_log1p-fma.c` … but there is **no**
//! `e_sinh-fma.c`, `e_cosh-fma.c` or `e_acosh-fma.c`. There is no ifunc for
//! them at all: the x86-64 `libm` runs the generic source compiled against the
//! SSE2 baseline, which has no FMA instruction, so *nothing* here is
//! contracted. Consequently [`f64::mul_add`] appears nowhere in this file, and
//! every `a * b + c` below is deliberately two separately rounded operations —
//! e.g. `h * (2.0 * t - t * t / (t + one))` in [`sinh`], `half * t + half / t`
//! in [`cosh`] and `2.0 * t + t * t` in [`acosh`].
//!
//! The routines they call *are* FMA-built, and those are this module's own
//! [`super::exp`], [`super::expm1`], [`super::log`] and [`super::log1p`], which
//! reproduce the contracted builds. `sqrt` is IEEE-754 correctly rounded, so
//! [`f64::sqrt`] is the same instruction the C emits.
//!
//! `math_narrow_eval()` is a no-op on x86-64 (`FLT_EVAL_METHOD == 0`) and is
//! dropped; `math_check_force_underflow()` only raises the underflow flag and
//! is commented at its site. The IBM Accurate Mathematical Library
//! multi-precision slow paths are not relevant — these files never had one.
//!
//! SPDX-License-Identifier: LGPL-2.1-or-later

use super::{exp, log, expm1, log1p};

/* e_sinh.c: static const double one = 1.0, shuge = 1.0e307; */
const ONE: f64 = 1.0;
const SHUGE: f64 = 1.0e307;

/* e_cosh.c: static const double one = 1.0, half = 0.5, huge = 1.0e300; */
const HALF: f64 = 0.5;
const HUGE: f64 = 1.0e300;

/* e_acosh.c: ln2 = 6.93147180559945286227e-01; 0x3FE62E42, 0xFEFA39EF */
const LN2: f64 = f64::from_bits(0x3fe6_2e42_fefa_39ef);

/// `sinh(x)` — glibc 2.39 `__ieee754_sinh`, `sysdeps/ieee754/dbl-64/e_sinh.c`.
///
/// ```text
///     0        <= x <= 22      : sinh(x) := (E + E/(E+1))/2, E = expm1(x)
///     22       <= x <= lnovft  : sinh(x) := exp(x)/2
///     lnovft   <= x <= ln2ovft : sinh(x) := exp(x/2)/2 * exp(x/2)
///     ln2ovft  <  x            : sinh(x) := x*shuge  (overflow)
/// ```
pub fn sinh(x: f64) -> f64 {
    let t: f64;
    let w: f64;
    let mut h: f64;

    /* GET_HIGH_WORD (jx, x); ix = jx & 0x7fffffff; */
    let jx = (x.to_bits() >> 32) as u32 as i32;
    let ix = jx & 0x7fff_ffff;

    /* x is INF or NaN */
    if ix >= 0x7ff0_0000 {
        return x + x;
    }

    h = 0.5;
    if jx < 0 {
        h = -h;
    }

    /* |x| in [0,22], return sign(x)*0.5*(E+E/(E+1)) */
    if ix < 0x4036_0000 {
        /* |x|<22 */
        if ix < 0x3e30_0000 {
            /* |x|<2**-28 */
            /* math_check_force_underflow (x): raises underflow for subnormal
            x, no effect on the value. */
            if SHUGE + x > ONE {
                /* sinh(tiny) = tiny with inexact */
                return x;
            }
        }
        t = expm1(x.abs());
        if ix < 0x3ff0_0000 {
            return h * (2.0 * t - t * t / (t + ONE));
        }
        return h * (t + t / (t + ONE));
    }

    /* |x| in [22, log(maxdouble)] return 0.5*exp(|x|) */
    if ix < 0x4086_2e42 {
        return h * exp(x.abs());
    }

    /* |x| in [log(maxdouble), overflowthresold] */
    /* GET_LOW_WORD (lx, x); */
    let lx = x.to_bits() as u32;
    if ix < 0x4086_33ce || (ix == 0x4086_33ce && lx <= 0x8fb9_f87d) {
        w = exp(0.5 * x.abs());
        let t2 = h * w;
        return t2 * w;
    }

    /* |x| > overflowthresold, sinh(x) overflow */
    /* math_narrow_eval() is a no-op with FLT_EVAL_METHOD == 0 */
    x * SHUGE
}

/// `cosh(x)` — glibc 2.39 `__ieee754_cosh`, `sysdeps/ieee754/dbl-64/e_cosh.c`.
///
/// ```text
///     0        <= x <= ln2/2   : cosh(x) := 1 + (exp(x)-1)^2/(2*exp(x))
///     ln2/2    <= x <= 22      : cosh(x) := (exp(x) + 1/exp(x))/2
///     22       <= x <= lnovft  : cosh(x) := exp(x)/2
///     lnovft   <= x <= ln2ovft : cosh(x) := exp(x/2)/2 * exp(x/2)
///     ln2ovft  <  x            : cosh(x) := huge*huge  (overflow)
/// ```
pub fn cosh(x: f64) -> f64 {
    let t: f64;
    let w: f64;

    /* GET_HIGH_WORD (ix, x); ix &= 0x7fffffff; */
    let ix = ((x.to_bits() >> 32) as u32 as i32) & 0x7fff_ffff;

    /* |x| in [0,22] */
    if ix < 0x4036_0000 {
        /* |x| in [0,0.5*ln2], return 1+expm1(|x|)^2/(2*exp(|x|)) */
        if ix < 0x3fd6_2e43 {
            if ix < 0x3c80_0000 {
                /* cosh(tiny) = 1 */
                return ONE;
            }
            t = expm1(x.abs());
            w = ONE + t;
            return ONE + (t * t) / (w + w);
        }

        /* |x| in [0.5*ln2,22], return (exp(|x|)+1/exp(|x|))/2 */
        t = exp(x.abs());
        return HALF * t + HALF / t;
    }

    /* |x| in [22, log(maxdouble)] return half*exp(|x|) */
    if ix < 0x4086_2e42 {
        return HALF * exp(x.abs());
    }

    /* |x| in [log(maxdouble), overflowthresold] */
    /* EXTRACT_WORDS64 (fix, x); fix &= 0x7fffffffffffffff; */
    let fix = x.to_bits() & 0x7fff_ffff_ffff_ffff;
    if fix <= 0x4086_33ce_8fb9_f87d {
        w = exp(HALF * x.abs());
        let t2 = HALF * w;
        return t2 * w;
    }

    /* x is INF or NaN */
    if ix >= 0x7ff0_0000 {
        return x * x;
    }

    /* |x| > overflowthresold, cosh(x) overflow */
    /* math_narrow_eval() is a no-op with FLT_EVAL_METHOD == 0 */
    HUGE * HUGE
}

/// `acosh(x)` — glibc 2.39 `__ieee754_acosh`, `sysdeps/ieee754/dbl-64/e_acosh.c`.
///
/// ```text
///     acosh(x) := log(x) + ln2                        if x is large; else
///     acosh(x) := log(2x - 1/(sqrt(x*x-1)+x))         if x > 2;      else
///     acosh(x) := log1p(t + sqrt(2*t + t*t)), t = x-1
/// ```
pub fn acosh(x: f64) -> f64 {
    /* EXTRACT_WORDS64 (hx, x); — signed, so x < 0 lands in the final else */
    let hx = x.to_bits() as i64;

    if hx > 0x4000_0000_0000_0000_i64 {
        if hx >= 0x41b0_0000_0000_0000_i64 {
            /* x > 2**28 */
            if hx >= 0x7ff0_0000_0000_0000_i64 {
                /* x is inf or NaN */
                return x + x;
            } else {
                /* acosh(huge) = log(2x) */
                return log(x) + LN2;
            }
        }

        /* 2**28 > x > 2 */
        let t = x * x;
        log(2.0 * x - ONE / (x + (t - ONE).sqrt()))
    } else if hx > 0x3ff0_0000_0000_0000_i64 {
        /* 1<x<2 */
        let t = x - ONE;
        log1p(t + (2.0 * t + t * t).sqrt())
    } else if hx == 0x3ff0_0000_0000_0000_i64 {
        /* acosh(1) = 0 */
        0.0
    } else {
        /* x < 1 */
        (x - x) / (x - x)
    }
}
