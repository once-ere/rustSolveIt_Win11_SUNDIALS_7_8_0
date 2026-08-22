//! libm_probe.rs — host-libm fingerprint, in the language the port is
//! written in.
//!
//! Standalone: no crate, no dependencies.
//!
//!   rustc -O tools/libm_probe.rs -o logs/libm_probe          # or .exe
//!   logs/libm_probe
//!
//! For every transcendental the port or its examples evaluate through
//! `f64`'s *unspecified-precision* methods, this prints a 64-bit hash of
//! 1,000,000 results over a fixed corpus. Two hosts agree on a function iff
//! the hashes match. Run it natively on Windows and inside a Linux guest and
//! diff the two outputs: what differs is exactly what stands between this
//! port and byte-identical output against the upstream (glibc-generated)
//! reference files.
//!
//! `pow` appears twice on purpose:
//!   * `powf`    — the host routine, i.e. what the port would call if it did
//!                 not carry its own. Expected to differ across hosts.
//!   * `sqrt`    — IEEE-754 specified, so it must match everywhere; it is the
//!                 control that shows the corpora really are in step.
//! The deterministic `pow` in `sundials_math.rs` is not probed here: it is
//! not a host routine, and `tools/pow_differential_win.sh` measures it
//! directly against a real glibc oracle.
//!
//! The corpus is regenerated from a splitmix64 recurrence, the same one the
//! `pow` oracle uses, so no data has to be transmitted between hosts.
//!
//! SPDX-License-Identifier: BSD-3-Clause

const N: usize = 1_000_000;

struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// [0,1) from the top 53 bits. `0x1p-53` as a bit pattern, so that
    /// building the corpus cannot itself call a libm routine.
    fn unit(&mut self) -> f64 {
        ((self.next() >> 11) as f64) * f64::from_bits(0x3ca0_0000_0000_0000)
    }
}

/// FNV-1a over the IEEE-754 bit pattern of each result.
fn mix(h: &mut u64, v: f64) {
    let b = v.to_bits();
    for i in 0..8 {
        *h ^= (b >> (8 * i)) & 0xff;
        *h = h.wrapping_mul(0x0000_0100_0000_01B3);
    }
}

fn probe(name: &str, seed: u64, map: impl Fn(f64) -> f64, arg: impl Fn(&mut SplitMix64) -> f64) {
    let mut rng = SplitMix64(seed);
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for _ in 0..N {
        let x = arg(&mut rng);
        mix(&mut h, map(x));
    }
    println!("{name:8} {h:016x}");
}

fn main() {
    println!(
        "# libm fingerprint  target={} corpus={}",
        std::env::consts::OS,
        N
    );

    /* Wide-range arguments, the range the examples' right-hand sides
    actually reach. */
    probe("sin", 11, f64::sin, |r| r.unit() * 200.0 - 100.0);
    probe("cos", 12, f64::cos, |r| r.unit() * 200.0 - 100.0);
    probe("tan", 13, f64::tan, |r| r.unit() * 200.0 - 100.0);
    probe("exp", 14, f64::exp, |r| r.unit() * 100.0 - 50.0);
    probe("ln", 15, f64::ln, |r| r.unit() * 1e6 + f64::MIN_POSITIVE);
    probe("log10", 16, f64::log10, |r| r.unit() * 1e6 + f64::MIN_POSITIVE);

    /* Domain-restricted arguments. */
    probe("asin", 21, f64::asin, |r| r.unit() * 2.0 - 1.0);
    probe("acos", 22, f64::acos, |r| r.unit() * 2.0 - 1.0);
    probe("atan", 23, f64::atan, |r| r.unit() * 200.0 - 100.0);
    probe("sinh", 24, f64::sinh, |r| r.unit() * 40.0 - 20.0);
    probe("cosh", 25, f64::cosh, |r| r.unit() * 40.0 - 20.0);
    probe("acosh", 26, f64::acosh, |r| r.unit() * 100.0 + 1.0);
    probe("tanh", 27, f64::tanh, |r| r.unit() * 40.0 - 20.0);

    /* The host `pow`, and the IEEE-754 control. */
    let mut rng = SplitMix64(31);
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for _ in 0..N {
        let mut x = rng.unit() * 100.0;
        if x == 0.0 {
            x = 100.0;
        }
        let s = rng.next();
        let y = if s % 14 == 0 {
            rng.unit() * 2.0 - 1.0
        } else {
            let v = 1.0 / ((s % 13) + 1) as f64;
            if s & 0x100 != 0 { -v } else { v }
        };
        mix(&mut h, x.powf(y));
    }
    println!("{:8} {h:016x}", "powf");

    probe("sqrt", 32, f64::sqrt, |r| r.unit() * 1e6);
}
