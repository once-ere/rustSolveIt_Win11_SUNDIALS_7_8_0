/* libm_oracle.c — glibc reference generator for the deterministic libm in
 * crates/sundials_core/src/sundials_libm/.
 *
 * SUNDIALS_7_8_Rust_port_for_Windows11. Build and run on a **glibc x86-64**
 * host — in practice a WSL2 Linux guest, driven by
 * tools/libm_differential_win.sh. The Rust side is built and run natively on
 * Windows; that crossing is the measurement.
 *
 *   cc -O2 -o libm_oracle tools/libm_oracle.c -lm
 *   ./libm_oracle exp 4000000 > logs/oracle/exp.bin
 *
 * Stream format (little-endian throughout):
 *
 *   u64  magic       0x4c49424d4f524331  ("LIBMORC1")
 *   u64  fn id       index into FNS below
 *   u64  n           number of results
 *   u64  input hash  FNV-1a over the bit patterns of all n arguments
 *   u64 * n          IEEE-754 bit patterns of f(x), in corpus order
 *
 * The corpus itself is NOT transmitted: both sides regenerate it from the
 * same splitmix64 recurrence. The input hash is the guard against the two
 * generators drifting apart — if it disagrees the Rust test fails loudly
 * instead of silently comparing different inputs. `gen()` below and
 * `corpus.rs`'s `gen()` are transliterations of each other and must be kept
 * that way, statement for statement.
 *
 * No libm call is used to *build* the corpus (only fabs/isfinite, which are
 * bit operations), so the arguments cannot depend on the host libm.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

enum {
  FN_EXP, FN_LOG, FN_SIN, FN_COS, FN_ATAN, FN_ASIN, FN_ACOS,
  FN_SINH, FN_COSH, FN_ACOSH, FN_EXPM1, FN_LOG1P, N_FN
};

static const char *FNS[N_FN] = {
  "exp", "log", "sin", "cos", "atan", "asin", "acos",
  "sinh", "cosh", "acosh", "expm1", "log1p"
};

#define MAGIC 0x4c49424d4f524331ull

static uint64_t st;

static uint64_t nxt(void)
{
  uint64_t z = (st += 0x9E3779B97F4A7C15ull);
  z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9ull;
  z = (z ^ (z >> 27)) * 0x94D049BB133111EBull;
  return z ^ (z >> 31);
}

static double bits_to_f64(uint64_t b) { double d; memcpy(&d, &b, 8); return d; }
static uint64_t f64_to_bits(double d) { uint64_t b; memcpy(&b, &d, 8); return b; }

/* [0,1) from the top 53 bits. */
static double unit(void) { return (double)(nxt() >> 11) * 0x1p-53; }

/* 2^-k by repeated halving: exact, and calls no libm routine. */
static double half_pow(unsigned k)
{
  double e = 1.0;
  for (unsigned j = 0; j < k; j++) { e *= 0.5; }
  return e;
}

/* Exceptional inputs, evaluated first in every corpus, before the random
 * modes below. One shared table for all twelve functions: a value that is
 * out of domain for one is in domain for another, so evaluating all of them
 * everywhere is what reaches the NaN/infinity/overflow/underflow branches
 * each routine carries. Without this the differential measures only the
 * arithmetic core and leaves every special-value branch unpinned.
 *
 * Keep byte-for-byte in step with SPECIAL in corpus.rs. */
#define N_SPECIAL 56
static const uint64_t SPECIAL[N_SPECIAL] = {
  0x7ff8000000000000ull, 0xfff8000000000000ull, 0x7ff0000000000000ull, 0xfff0000000000000ull,
  0x0000000000000000ull, 0x8000000000000000ull, 0x3ff0000000000000ull, 0xbff0000000000000ull,
  0x4000000000000000ull, 0xc000000000000000ull, 0x3fe0000000000000ull, 0xbfe0000000000000ull,
  0x7fefffffffffffffull, 0xffefffffffffffffull, 0x0010000000000000ull, 0x8010000000000000ull,
  0x000fffffffffffffull, 0x800fffffffffffffull, 0x0000000000000001ull, 0x8000000000000001ull,
  0x3c90000000000000ull, 0xbc90000000000000ull, 0x3e30000000000000ull, 0xbe30000000000000ull,
  0x3e50000000000000ull, 0xbe50000000000000ull, 0x3c70000000000000ull, 0xbc70000000000000ull,
  0x40862e42fefa39efull, 0x40862e42fefa39f0ull, 0x40862e6666666666ull, 0xc0874910d52d3051ull,
  0xc08749999999999aull, 0x4090000000000000ull, 0xc090000000000000ull, 0xc043687fa440e825ull,
  0x408633ce8fb9f87dull, 0xc08633ce8fb9f87dull, 0x4086340000000000ull, 0xc086340000000000ull,
  0x4036000000000000ull, 0xc036000000000000ull, 0x3ff0000000000001ull, 0xbff0000000000001ull,
  0x41b0000000000000ull, 0x7e37e43c8800759cull, 0x3fefffffffffffffull, 0xbff8000000000000ull,
  0x7fe1ccf385ebc8a0ull, 0x000730d67819e8d2ull, 0x3ff921fb54442d18ull, 0x400921fb54442d18ull,
  0x401921fb54442d18ull, 0x4480f0cf064dd592ull, 0x43e0000000000000ull, 0x7fe0000000000000ull,
};

/* Argument corpus. The special table above first, then ten sub-modes per function, selected by i % 10, so every
 * corpus mixes the operating range, the extremes and the awkward
 * neighbourhoods (near 1 for log, near +-1 for asin/acos, near multiples of
 * pi/2 for sin/cos, huge arguments that force Payne-Hanek reduction). */
static double gen(int fn, unsigned i)
{
  if (i < N_SPECIAL) { return bits_to_f64(SPECIAL[i]); }
  unsigned m = (i - N_SPECIAL) % 10;
  switch (fn)
  {
    case FN_EXP:
      if (m < 6) { return unit() * 1417.7 - 708.0; }
      if (m < 8) { return unit() * 100.0 - 50.0; }
      for (;;) { double x = bits_to_f64(nxt()); if (isfinite(x) && fabs(x) < 800.0) { return x; } }

    case FN_LOG:
      if (m < 5) { for (;;) { double x = bits_to_f64(nxt()); if (isfinite(x) && x > 0.0) { return x; } } }
      if (m < 8) { double x = unit() * 1e6; if (x == 0.0) { x = 1.0; } return x; }
      { double u = unit(); unsigned k = (unsigned)(nxt() % 60);
        return 1.0 + (2.0 * u - 1.0) * half_pow(k); }

    case FN_SIN:
    case FN_COS:
      if (m < 4) { return unit() * 200.0 - 100.0; }
      if (m < 7) { return unit() * 2.0e6 - 1.0e6; }
      if (m < 9) { for (;;) { double x = bits_to_f64(nxt()); if (isfinite(x)) { return x; } } }
      { double k = (double)(int64_t)(nxt() % 1000) - 500.0; double u = unit();
        return k * 1.5707963267948966 + (2.0 * u - 1.0) * 1e-9; }

    case FN_ATAN:
      if (m < 5) { return unit() * 200.0 - 100.0; }
      if (m < 8) { for (;;) { double x = bits_to_f64(nxt()); if (isfinite(x)) { return x; } } }
      return unit() * 2.0 - 1.0;

    case FN_ASIN:
    case FN_ACOS:
      if (m < 7) { return unit() * 2.0 - 1.0; }
      { double u = unit(); unsigned k = (unsigned)(nxt() % 40);
        double v = 1.0 - u * half_pow(k);
        return (nxt() & 1) ? -v : v; }

    case FN_SINH:
    case FN_COSH:
      if (m < 5) { return unit() * 80.0 - 40.0; }
      if (m < 8) { return unit() * 1420.0 - 710.0; }
      return unit() * 2.0 - 1.0;

    case FN_ACOSH:
      if (m < 5) { return 1.0 + unit() * 1e6; }
      if (m < 8) { double u = unit(); unsigned k = (unsigned)(nxt() % 50);
                   return 1.0 + u * half_pow(k); }
      return 1.0 + unit() * 1e300;

    case FN_EXPM1:
      if (m < 5) { return unit() * 80.0 - 40.0; }
      if (m < 8) { double u = unit(); unsigned k = (unsigned)(nxt() % 60);
                   return (2.0 * u - 1.0) * half_pow(k); }
      return unit() * 1500.0 - 750.0;

    case FN_LOG1P:
      if (m < 5) { return unit() * 1e6; }
      if (m < 8) { double u = unit(); unsigned k = (unsigned)(nxt() % 60);
                   return (2.0 * u - 1.0) * half_pow(k); }
      return -1.0 + unit();

    default:
      return 0.0;
  }
}

static double apply(int fn, double x)
{
  switch (fn)
  {
    case FN_EXP:   return exp(x);
    case FN_LOG:   return log(x);
    case FN_SIN:   return sin(x);
    case FN_COS:   return cos(x);
    case FN_ATAN:  return atan(x);
    case FN_ASIN:  return asin(x);
    case FN_ACOS:  return acos(x);
    case FN_SINH:  return sinh(x);
    case FN_COSH:  return cosh(x);
    case FN_ACOSH: return acosh(x);
    case FN_EXPM1: return expm1(x);
    case FN_LOG1P: return log1p(x);
    default:       return 0.0;
  }
}

int main(int argc, char **argv)
{
  if (argc < 2)
  {
    fprintf(stderr, "usage: %s <fn> [n]\nfunctions:", argv[0]);
    for (int i = 0; i < N_FN; i++) { fprintf(stderr, " %s", FNS[i]); }
    fprintf(stderr, "\n");
    return 2;
  }
  int fn = -1;
  for (int i = 0; i < N_FN; i++) { if (strcmp(argv[1], FNS[i]) == 0) { fn = i; } }
  if (fn < 0) { fprintf(stderr, "unknown function: %s\n", argv[1]); return 2; }

  uint64_t n = (argc > 2) ? strtoull(argv[2], NULL, 10) : 4000000ull;

  /* Seed is the function index + 1, so every function has its own corpus and
   * none of them is the pow corpus (which seeds 1 and 2). */
  st = (uint64_t)fn + 101ull;

  double *xs = malloc((size_t)n * sizeof *xs);
  if (!xs) { fprintf(stderr, "out of memory\n"); return 1; }

  uint64_t h = 0xcbf29ce484222325ull;
  for (uint64_t i = 0; i < n; i++)
  {
    xs[i] = gen(fn, (unsigned)i);
    uint64_t b = f64_to_bits(xs[i]);
    for (int k = 0; k < 8; k++)
    {
      h ^= (b >> (8 * k)) & 0xff;
      h *= 0x00000100000001B3ull;
    }
  }

  uint64_t hdr[4] = { MAGIC, (uint64_t)fn, n, h };
  if (fwrite(hdr, sizeof hdr, 1, stdout) != 1) { return 1; }
  for (uint64_t i = 0; i < n; i++)
  {
    uint64_t r = f64_to_bits(apply(fn, xs[i]));
    if (fwrite(&r, sizeof r, 1, stdout) != 1) { return 1; }
  }
  free(xs);
  fprintf(stderr, "%s: %llu results, input hash %016llx\n",
          FNS[fn], (unsigned long long)n, (unsigned long long)h);
  return 0;
}
