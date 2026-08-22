//! Test-only twin of `tools/libm_oracle.c`.
//!
//! `gen` below is a statement-for-statement transliteration of the C
//! `gen()`; `run_differential` reads the oracle's bit-stream, re-derives the
//! same arguments, and compares. The stream carries an FNV-1a hash of the
//! argument bit patterns, checked before any result is compared, so the two
//! generators cannot silently drift apart and evaluate different inputs.
//!
//! With no `SUNDIALS_LIBM_ORACLE_DIR` in the environment every differential
//! test reports "not run" and passes: `cargo test` stays green on hosts
//! where no oracle was built, and on non-glibc hosts where one would be
//! meaningless. **A green `cargo test` on its own is therefore not evidence
//! that anything was compared.** Set `SUNDIALS_LIBM_ORACLE_STRICT` to turn a
//! missing oracle into a hard failure; `tools/libm_differential_win.sh` sets
//! it, so a run that was meant to measure cannot silently measure nothing.
//!
//! SPDX-License-Identifier: BSD-3-Clause

pub const MAGIC: u64 = 0x4c49_424d_4f52_4331;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Fn_ {
    Exp = 0,
    Log = 1,
    Sin = 2,
    Cos = 3,
    Atan = 4,
    Asin = 5,
    Acos = 6,
    Sinh = 7,
    Cosh = 8,
    Acosh = 9,
    Expm1 = 10,
    Log1p = 11,
}

impl Fn_ {
    pub fn name(self) -> &'static str {
        match self {
            Fn_::Exp => "exp",
            Fn_::Log => "log",
            Fn_::Sin => "sin",
            Fn_::Cos => "cos",
            Fn_::Atan => "atan",
            Fn_::Asin => "asin",
            Fn_::Acos => "acos",
            Fn_::Sinh => "sinh",
            Fn_::Cosh => "cosh",
            Fn_::Acosh => "acosh",
            Fn_::Expm1 => "expm1",
            Fn_::Log1p => "log1p",
        }
    }
}

pub struct SplitMix64(pub u64);

impl SplitMix64 {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// `[0,1)` from the top 53 bits. `0x1p-53` as a bit pattern, so building
    /// the corpus calls nothing.
    pub fn unit(&mut self) -> f64 {
        ((self.next() >> 11) as f64) * f64::from_bits(0x3ca0_0000_0000_0000)
    }
}

/// `2^-k` by repeated halving — exact, and calls no libm routine.
fn half_pow(k: u32) -> f64 {
    let mut e = 1.0f64;
    for _ in 0..k {
        e *= 0.5;
    }
    e
}

/// Exceptional inputs, evaluated first in every corpus. One shared table for
/// all twelve routines: a value out of domain for one is in domain for
/// another, so evaluating all of them everywhere is what reaches the
/// NaN/infinity/overflow/underflow branches each routine carries. Without
/// this the differential measures only the arithmetic core and leaves every
/// special-value branch unpinned.
///
/// Keep byte-for-byte in step with `SPECIAL` in `tools/libm_oracle.c`.
pub const SPECIAL: [u64; 56] = [
    0x7ff8000000000000, 0xfff8000000000000, 0x7ff0000000000000, 0xfff0000000000000,
    0x0000000000000000, 0x8000000000000000, 0x3ff0000000000000, 0xbff0000000000000,
    0x4000000000000000, 0xc000000000000000, 0x3fe0000000000000, 0xbfe0000000000000,
    0x7fefffffffffffff, 0xffefffffffffffff, 0x0010000000000000, 0x8010000000000000,
    0x000fffffffffffff, 0x800fffffffffffff, 0x0000000000000001, 0x8000000000000001,
    0x3c90000000000000, 0xbc90000000000000, 0x3e30000000000000, 0xbe30000000000000,
    0x3e50000000000000, 0xbe50000000000000, 0x3c70000000000000, 0xbc70000000000000,
    0x40862e42fefa39ef, 0x40862e42fefa39f0, 0x40862e6666666666, 0xc0874910d52d3051,
    0xc08749999999999a, 0x4090000000000000, 0xc090000000000000, 0xc043687fa440e825,
    0x408633ce8fb9f87d, 0xc08633ce8fb9f87d, 0x4086340000000000, 0xc086340000000000,
    0x4036000000000000, 0xc036000000000000, 0x3ff0000000000001, 0xbff0000000000001,
    0x41b0000000000000, 0x7e37e43c8800759c, 0x3fefffffffffffff, 0xbff8000000000000,
    0x7fe1ccf385ebc8a0, 0x000730d67819e8d2, 0x3ff921fb54442d18, 0x400921fb54442d18,
    0x401921fb54442d18, 0x4480f0cf064dd592, 0x43e0000000000000, 0x7fe0000000000000,
];

/// Transliteration of `gen()` in `tools/libm_oracle.c`. Keep in lockstep.
pub fn gen(f: Fn_, i: u32, r: &mut SplitMix64) -> f64 {
    if (i as usize) < SPECIAL.len() {
        return f64::from_bits(SPECIAL[i as usize]);
    }
    let m = (i - SPECIAL.len() as u32) % 10;
    match f {
        Fn_::Exp => {
            if m < 6 {
                return r.unit() * 1417.7 - 708.0;
            }
            if m < 8 {
                return r.unit() * 100.0 - 50.0;
            }
            loop {
                let x = f64::from_bits(r.next());
                if x.is_finite() && x.abs() < 800.0 {
                    return x;
                }
            }
        }
        Fn_::Log => {
            if m < 5 {
                loop {
                    let x = f64::from_bits(r.next());
                    if x.is_finite() && x > 0.0 {
                        return x;
                    }
                }
            }
            if m < 8 {
                let x = r.unit() * 1e6;
                return if x == 0.0 { 1.0 } else { x };
            }
            let u = r.unit();
            let k = (r.next() % 60) as u32;
            1.0 + (2.0 * u - 1.0) * half_pow(k)
        }
        Fn_::Sin | Fn_::Cos => {
            if m < 4 {
                return r.unit() * 200.0 - 100.0;
            }
            if m < 7 {
                return r.unit() * 2.0e6 - 1.0e6;
            }
            if m < 9 {
                loop {
                    let x = f64::from_bits(r.next());
                    if x.is_finite() {
                        return x;
                    }
                }
            }
            let k = ((r.next() % 1000) as i64) as f64 - 500.0;
            let u = r.unit();
            k * 1.5707963267948966 + (2.0 * u - 1.0) * 1e-9
        }
        Fn_::Atan => {
            if m < 5 {
                return r.unit() * 200.0 - 100.0;
            }
            if m < 8 {
                loop {
                    let x = f64::from_bits(r.next());
                    if x.is_finite() {
                        return x;
                    }
                }
            }
            r.unit() * 2.0 - 1.0
        }
        Fn_::Asin | Fn_::Acos => {
            if m < 7 {
                return r.unit() * 2.0 - 1.0;
            }
            let u = r.unit();
            let k = (r.next() % 40) as u32;
            let v = 1.0 - u * half_pow(k);
            if r.next() & 1 != 0 { -v } else { v }
        }
        Fn_::Sinh | Fn_::Cosh => {
            if m < 5 {
                return r.unit() * 80.0 - 40.0;
            }
            if m < 8 {
                return r.unit() * 1420.0 - 710.0;
            }
            r.unit() * 2.0 - 1.0
        }
        Fn_::Acosh => {
            if m < 5 {
                return 1.0 + r.unit() * 1e6;
            }
            if m < 8 {
                let u = r.unit();
                let k = (r.next() % 50) as u32;
                return 1.0 + u * half_pow(k);
            }
            1.0 + r.unit() * 1e300
        }
        Fn_::Expm1 => {
            if m < 5 {
                return r.unit() * 80.0 - 40.0;
            }
            if m < 8 {
                let u = r.unit();
                let k = (r.next() % 60) as u32;
                return (2.0 * u - 1.0) * half_pow(k);
            }
            r.unit() * 1500.0 - 750.0
        }
        Fn_::Log1p => {
            if m < 5 {
                return r.unit() * 1e6;
            }
            if m < 8 {
                let u = r.unit();
                let k = (r.next() % 60) as u32;
                return (2.0 * u - 1.0) * half_pow(k);
            }
            -1.0 + r.unit()
        }
    }
}

fn fnv(h: &mut u64, b: u64) {
    for k in 0..8 {
        *h ^= (b >> (8 * k)) & 0xff;
        *h = h.wrapping_mul(0x0000_0100_0000_01B3);
    }
}

/// Compare `port` against the glibc oracle stream for `f`.
///
/// Returns `None` when no oracle is available. Otherwise returns
/// `(n, mismatches, worst_ulp)` and prints the first few disagreements.
pub fn run_differential(f: Fn_, port: impl Fn(f64) -> f64) -> Option<(usize, usize, i64)> {
    /* Strict mode. Without it a missing oracle, a misspelled variable or a
    renamed stream makes every one of these tests pass while comparing
    nothing — a green `cargo test` would then be no evidence at all. The
    harness sets SUNDIALS_LIBM_ORACLE_STRICT, so a run that was *meant* to
    measure fails loudly instead of silently degrading to a no-op. */
    let strict = std::env::var("SUNDIALS_LIBM_ORACLE_STRICT").is_ok();
    let dir = match std::env::var("SUNDIALS_LIBM_ORACLE_DIR") {
        Ok(d) => d,
        Err(_) => {
            assert!(!strict, "SUNDIALS_LIBM_ORACLE_STRICT is set but \
                              SUNDIALS_LIBM_ORACLE_DIR is not");
            return None;
        }
    };
    let path = std::path::Path::new(&dir).join(format!("{}.bin", f.name()));
    if !path.exists() {
        assert!(!strict, "SUNDIALS_LIBM_ORACLE_STRICT is set but there is no \
                          oracle at {}", path.display());
        return None;
    }
    let blob = std::fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    assert!(blob.len() >= 32, "{}: truncated header", path.display());

    let word = |i: usize| -> u64 {
        let mut w = [0u8; 8];
        w.copy_from_slice(&blob[i * 8..i * 8 + 8]);
        u64::from_le_bytes(w)
    };
    assert_eq!(word(0), MAGIC, "{}: bad magic", path.display());
    assert_eq!(word(1), f as u64, "{}: stream is for another function", path.display());
    let n = word(2) as usize;
    let want_hash = word(3);
    assert!(blob.len() >= 32 + 8 * n, "{}: truncated body", path.display());

    /* Re-derive the corpus and check it against the oracle's hash before
    trusting a single result. */
    let mut r = SplitMix64(f as u64 + 101);
    let mut xs = Vec::with_capacity(n);
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for i in 0..n {
        let x = gen(f, i as u32, &mut r);
        fnv(&mut h, x.to_bits());
        xs.push(x);
    }
    assert_eq!(
        h,
        want_hash,
        "{}: corpus hash mismatch — corpus.rs::gen and libm_oracle.c::gen have drifted apart; \
         every comparison below would be meaningless",
        f.name()
    );

    let (mut bad, mut worst) = (0usize, 0i64);
    for (i, &x) in xs.iter().enumerate() {
        let want = word(4 + i);
        let got = port(x).to_bits();
        if got == want {
            continue;
        }
        let (wf, gf) = (f64::from_bits(want), f64::from_bits(got));
        if wf.is_nan() && gf.is_nan() {
            continue; /* NaN payloads are not architecturally specified */
        }
        if bad < 8 {
            eprintln!(
                "  {} mismatch: x={:016x} ({:e})  glibc={:016x}  port={:016x}",
                f.name(),
                x.to_bits(),
                x,
                want,
                got
            );
        }
        bad += 1;
        /* Both finite and same sign in every corpus we build, so the bit
        patterns are monotone in the value and their gap is the ulp
        distance. Infinity/NaN cases fall back to a large sentinel. */
        let d = if wf.is_finite() && gf.is_finite() && wf.is_sign_positive() == gf.is_sign_positive()
        {
            (want as i64 - got as i64).abs()
        } else {
            i64::MAX
        };
        if d > worst {
            worst = d;
        }
    }
    Some((n, bad, worst))
}

/// The body every per-function differential test shares.
pub fn assert_bit_exact(f: Fn_, port: impl Fn(f64) -> f64) {
    match run_differential(f, port) {
        None => eprintln!("{} differential: not run (no oracle)", f.name()),
        Some((n, bad, worst)) => {
            eprintln!("{} differential: {n} inputs, {bad} mismatches, worst {worst} ulp", f.name());
            assert_eq!(bad, 0, "{} disagrees with the glibc oracle", f.name());
        }
    }
}
