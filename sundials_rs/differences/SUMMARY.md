# differences — summary

Generated 2026-08-11T12:34:51Z from commit `bc4e2b3`.

## C vs Rust, by class

| class | variants | meaning |
|---|---:|---|
| identical | 131 | same bytes after the timing filter |
| whitespace-only | 0 | every printed value identical, column spacing differs |
| content | 48 | at least one printed value differs |

## By example directory

| directory | variants | C==Rust | ws-only | content | Rust==ref | C==ref |
|---|---:|---:|---:|---:|---:|---:|
| `arkode/C_klu` | 1 | 0 | 0 | 1 | 0 | 0 |
| `arkode/C_manyvector` | 1 | 0 | 0 | 1 | 0 | 0 |
| `arkode/C_openmp` | 2 | 0 | 0 | 2 | 0 | 0 |
| `arkode/C_openmpdev` | 3 | 0 | 0 | 3 | 0 | 0 |
| `arkode/C_parallel` | 5 | 0 | 0 | 5 | 0 | 0 |
| `arkode/C_parhyp` | 1 | 0 | 0 | 1 | 0 | 0 |
| `arkode/C_petsc` | 1 | 0 | 0 | 1 | 0 | 0 |
| `arkode/C_serial` | 78 | 54 | 0 | 24 | 63 | 42 |
| `arkode/C_superlu-mt` | 1 | 0 | 0 | 1 | 0 | 0 |
| `cvode/C_mpimanyvector` | 1 | 0 | 0 | 1 | 0 | 0 |
| `cvode/C_openmp` | 1 | 0 | 0 | 1 | 0 | 0 |
| `cvode/C_openmpdev` | 1 | 0 | 0 | 1 | 0 | 0 |
| `cvode/parallel` | 4 | 0 | 0 | 4 | 0 | 0 |
| `cvode/parhyp` | 1 | 0 | 0 | 1 | 0 | 0 |
| `cvode/petsc` | 2 | 0 | 0 | 2 | 0 | 0 |
| `cvode/serial` | 21 | 13 | 0 | 8 | 18 | 10 |
| `cvodes/C_openmp` | 1 | 0 | 0 | 1 | 0 | 0 |
| `cvodes/parallel` | 9 | 0 | 0 | 9 | 0 | 0 |
| `cvodes/serial` | 33 | 22 | 0 | 11 | 27 | 20 |
| `ida/C_openmp` | 2 | 0 | 0 | 2 | 0 | 0 |
| `ida/parallel` | 4 | 0 | 0 | 4 | 0 | 0 |
| `ida/petsc` | 5 | 0 | 0 | 5 | 0 | 0 |
| `ida/serial` | 11 | 9 | 0 | 2 | 11 | 9 |
| `idas/C_openmp` | 2 | 0 | 0 | 2 | 0 | 0 |
| `idas/parallel` | 8 | 0 | 0 | 8 | 0 | 0 |
| `idas/serial` | 16 | 13 | 0 | 3 | 15 | 12 |
| `kinsol/C_openmp` | 1 | 0 | 0 | 1 | 0 | 0 |
| `kinsol/parallel` | 2 | 0 | 0 | 2 | 0 | 0 |
| `kinsol/serial` | 20 | 20 | 0 | 0 | 19 | 19 |
| **total** | **238** | **131** | **0** | **48** | **153** | **112** |

## Which side is right where they disagree

A disagreement is only a *defect* in one side if that side also disagrees with
the reference output shipped with SUNDIALS 7.8.0 — which was generated on a
glibc host by the upstream project.

| verdict | variants |
|---|---:|
| Rust matches the shipped reference, C does not | **41** |
| C matches the shipped reference, Rust does not | **0** |
| neither matches the shipped reference | **7** |
