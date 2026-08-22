//! `expm1` — double-precision e^x - 1, host-independent.
//!
//! Translated from: glibc 2.39 `sysdeps/ieee754/dbl-64/s_expm1.c` (fdlibm-derived)
//!
//! On x86-64 glibc does not run that file as compiled for the SSE2 baseline:
//! `sysdeps/x86_64/fpu/multiarch/s_expm1.c` ifunc-dispatches to `__expm1_fma`,
//! which is `s_expm1-fma.c` — the same generic source re-included with
//! `-mfma -mavx2 -ffp-contract=fast`. Every `a*b + c` the compiler can see is
//! therefore emitted as a single fused multiply-add, and this translation uses
//! [`f64::mul_add`] in exactly those places. Rust never contracts on its own,
//! so the map below is explicit and reproducible:
//!
//! | C expression | fused? | why |
//! |---|---|---|
//! | `invln2 * x + ±0.5` | yes | contracted; the truncation to `int` makes it immaterial either way |
//! | `x - t * ln2_hi` | n/a | `t*ln2_hi` is *exact* (11-bit `t`, 32-bit `ln2_hi`), so fusing cannot change it — written as the C writes it |
//! | `one + hxs*Q[1]`, `Q[2] + hxs*Q[3]`, `Q[4] + hxs*Q[5]` | yes | each product feeds exactly one add |
//! | `R1 + h2*R2 + h4*R3` | yes, twice | nested: `fma(h4, R3, fma(h2, R2, R1))` |
//! | `3.0 - r1*hfx`, `6.0 - x*t` | yes | negated-product forms of the same contraction |
//! | `x*e - hxs`, `x*(e - c) - c` | yes | product feeds one add |
//! | `0.5*(x - e) - 0.5`, `one + 2.0*(x - e)` | yes | contracted, but the multiplier is a power of two so the product is exact and the choice is immaterial |
//!
//! The IBM Accurate Mathematical Library multi-precision slow paths are not
//! relevant here — this file never had one — and `math_narrow_eval()` is a
//! no-op on x86-64 (`FLT_EVAL_METHOD == 0`), so it is dropped. The
//! `math_force_eval`/`huge + x` idioms exist only to raise `inexact` and
//! `underflow`; they are kept where they carry the returned value and
//! commented where they do not.
//!
//! SPDX-License-Identifier: LGPL-2.1-or-later

/* Constants of `static const double` in s_expm1.c, as IEEE-754 bit
patterns. Emitted by a throwaway generator that parses the C literals and
cross-checks each one against the hexadecimal words the source carries in
its trailing comments; never transcribed by hand. */
const HUGE: u64 = 0x7e37_e43c_8800_759c; /* huge  = 1.0e+300 */
const TINY: u64 = 0x01a5_6e1f_c2f8_f359; /* tiny  = 1.0e-300 */
const O_THRESHOLD: u64 = 0x4086_2e42_fefa_39ef; /* 7.09782712893383973096e+02 */
const LN2_HI: u64 = 0x3fe6_2e42_fee0_0000; /* 6.93147180369123816490e-01 */
const LN2_LO: u64 = 0x3dea_39ef_3579_3c76; /* 1.90821492927058770002e-10 */
const INVLN2: u64 = 0x3ff7_1547_652b_82fe; /* 1.44269504088896338700e+00 */

/* scaled coefficients related to expm1; `one` is Q[0] as in the C */
const Q: [u64; 6] = [
    0x3ff0_0000_0000_0000, /* Q[0] =  1.0                       */
    0xbfa1_1111_1111_10f4, /* Q[1] = -3.33333333333331316428e-02 */
    0x3f5a_01a0_19fe_5585, /* Q[2] =  1.58730158725481460165e-03 */
    0xbf14_ce19_9eaa_dbb7, /* Q[3] = -7.93650757867487942473e-05 */
    0x3ed0_cfca_86e6_5239, /* Q[4] =  4.00821782732936239552e-06 */
    0xbe8a_fdb7_6e09_c32d, /* Q[5] = -2.01099218183624371326e-07 */
];

/// `GET_HIGH_WORD` — math_private.h.
#[inline]
fn get_high_word(x: f64) -> u32 {
    (x.to_bits() >> 32) as u32
}

/// `SET_HIGH_WORD` — math_private.h; the low word is preserved.
#[inline]
fn set_high_word(x: f64, hi: u32) -> f64 {
    f64::from_bits((x.to_bits() & 0x0000_0000_ffff_ffff) | ((hi as u64) << 32))
}

/// C `__expm1 (double x)` — glibc 2.39 `sysdeps/ieee754/dbl-64/s_expm1.c`.
pub fn expm1(x: f64) -> f64 {
    let one = f64::from_bits(Q[0]);
    let huge = f64::from_bits(HUGE);
    let tiny = f64::from_bits(TINY);

    let mut x = x;

    /* GET_HIGH_WORD (hx, x); the C's `y = |x|` alongside is dead — it is
    overwritten before every use — so only the sign bit is kept. */
    let mut hx: u32 = get_high_word(x);
    let xsb: u32 = hx & 0x8000_0000; /* sign bit of x */
    hx &= 0x7fff_ffff; /* high word of |x| */

    /* filter out huge and non-finite argument */
    if hx >= 0x4043_687A {
        /* if |x|>=56*ln2 */
        if hx >= 0x4086_2E42 {
            /* if |x|>=709.78... */
            if hx >= 0x7ff0_0000 {
                let low: u32 = x.to_bits() as u32; /* GET_LOW_WORD */
                if ((hx & 0xf_ffff) | low) != 0 {
                    return x + x; /* NaN */
                } else {
                    /* exp(+-inf)={inf,-1} */
                    return if xsb == 0 { x } else { -1.0 };
                }
            }
            if x > f64::from_bits(O_THRESHOLD) {
                /* __set_errno (ERANGE) — not modelled */
                return huge * huge; /* overflow */
            }
        }
        if xsb != 0 {
            /* x < -56*ln2, return -1.0 with inexact */
            /* math_force_eval (x + tiny) raises inexact only */
            return tiny - one; /* return -1 */
        }
    }

    /* argument reduction.  The C leaves `c` uninitialised on the k == 0 path
    and never reads it there ("c is 0"); here the two are produced together
    so no path reads an undefined value. */
    let (k, c): (i32, f64) = if hx > 0x3fd6_2e42 {
        /* if  |x| > 0.5 ln2 */
        let (hi, lo, k): (f64, f64, i32) = if hx < 0x3FF0_A2B2 {
            /* and |x| < 1.5 ln2 */
            if xsb == 0 {
                (x - f64::from_bits(LN2_HI), f64::from_bits(LN2_LO), 1)
            } else {
                (x + f64::from_bits(LN2_HI), -f64::from_bits(LN2_LO), -1)
            }
        } else {
            /* k = invln2 * x + (xsb == 0 ? 0.5 : -0.5) — contracted by
            -ffp-contract=fast, then truncated toward zero by the
            assignment to int32_t. */
            let k = f64::from_bits(INVLN2)
                .mul_add(x, if xsb == 0 { 0.5 } else { -0.5 }) as i32;
            let t = k as f64;
            /* t*ln2_hi is exact here (|t| <= 1024 is 11 bits, ln2_hi is 32
            significant bits), so contracting this product into the
            subtraction cannot change the result. */
            let hi = x - t * f64::from_bits(LN2_HI);
            let lo = t * f64::from_bits(LN2_LO);
            (hi, lo, k)
        };
        x = hi - lo;
        (k, (hi - x) - lo)
    } else if hx < 0x3c90_0000 {
        /* when |x|<2**-54, return x */
        /* math_check_force_underflow (x) raises underflow only */
        let t = huge + x; /* return x with inexact flags when x!=0 */
        return x - (t - (huge + x));
    } else {
        (0, 0.0)
    };

    /* x is now in primary range */
    let hfx = 0.5 * x;
    let hxs = x * hfx;
    /* R1 = one + hxs*Q[1];  h2 = hxs*hxs;
    R2 = Q[2] + hxs*Q[3];  h4 = h2*h2;
    R3 = Q[4] + hxs*Q[5];
    r1 = R1 + h2*R2 + h4*R3;   -- all contracted */
    let big_r1 = hxs.mul_add(f64::from_bits(Q[1]), one);
    let h2 = hxs * hxs;
    let big_r2 = hxs.mul_add(f64::from_bits(Q[3]), f64::from_bits(Q[2]));
    let h4 = h2 * h2;
    let big_r3 = hxs.mul_add(f64::from_bits(Q[5]), f64::from_bits(Q[4]));
    let r1 = h4.mul_add(big_r3, h2.mul_add(big_r2, big_r1));
    let t = (-r1).mul_add(hfx, 3.0); /* t = 3.0 - r1*hfx */
    let e = hxs * ((r1 - t) / (-x).mul_add(t, 6.0)); /* 6.0 - x*t */

    if k == 0 {
        return x - x.mul_add(e, -hxs); /* c is 0 */
    }

    /* e = (x * (e - c) - c);  e -= hxs; */
    let mut e = x.mul_add(e - c, -c);
    e -= hxs;

    if k == -1 {
        return 0.5f64.mul_add(x - e, -0.5); /* 0.5*(x-e) - 0.5 */
    }
    if k == 1 {
        if x < -0.25 {
            return -2.0 * (e - (x + 0.5));
        } else {
            return 2.0f64.mul_add(x - e, one); /* one + 2.0*(x-e) */
        }
    }
    if k <= -2 || k > 56 {
        /* suffice to return exp(x)-1 */
        let y = one - (e - x);
        /* add k to y's exponent */
        let y = set_high_word(y, get_high_word(y).wrapping_add((k as u32) << 20));
        return y - one;
    }

    let mut t = one;
    let y;
    if k < 20 {
        t = set_high_word(t, 0x3ff0_0000 - (0x0020_0000u32 >> k)); /* t=1-2^-k */
        let y0 = t - (e - x);
        /* add k to y's exponent */
        y = set_high_word(y0, get_high_word(y0).wrapping_add((k as u32) << 20));
    } else {
        t = set_high_word(t, ((0x3ff - k) as u32) << 20); /* 2^-k */
        let mut y0 = x - (e + t);
        y0 += one;
        /* add k to y's exponent */
        y = set_high_word(y0, get_high_word(y0).wrapping_add((k as u32) << 20));
    }
    y
}
