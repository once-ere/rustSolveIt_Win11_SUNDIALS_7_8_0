# Exclusions

## Serial examples excluded on both sides (20 programs, 20 variants)

These need KLU or SuperLU_MT. Neither library is installed, the C build skips
them, and the Rust port excludes them by specification — so they are excluded
symmetrically and no comparison is affected.

| example | argv | dir | reason |
|---|---|---|---|
| `cvRoberts_block_klu` |  | cvode/serial | excluded(klu) |
| `cvRoberts_klu` |  | cvode/serial | excluded(klu) |
| `cvRoberts_sps` |  | cvode/serial | excluded(superlu) |
| `cvsRoberts_ASAi_klu` |  | cvodes/serial | excluded(klu) |
| `cvsRoberts_FSA_klu` | `-sensi stg1 t` | cvodes/serial | excluded(klu) |
| `cvsRoberts_klu` |  | cvodes/serial | excluded(klu) |
| `cvsRoberts_ASAi_sps` |  | cvodes/serial | excluded(superlu) |
| `cvsRoberts_FSA_sps` | `-sensi stg1 t` | cvodes/serial | excluded(superlu) |
| `cvsRoberts_sps` |  | cvodes/serial | excluded(superlu) |
| `kinFerTron_klu` |  | kinsol/serial | excluded(klu) |
| `kinRoboKin_slu` |  | kinsol/serial | excluded(superlu) |
| `idaHeat2D_klu` |  | ida/serial | excluded(klu) |
| `idaRoberts_klu` |  | ida/serial | excluded(klu) |
| `idaRoberts_sps` |  | ida/serial | excluded(superlu) |
| `idasRoberts_ASAi_klu` |  | idas/serial | excluded(klu) |
| `idasRoberts_FSA_klu` | `-sensi stg t` | idas/serial | excluded(klu) |
| `idasRoberts_klu` |  | idas/serial | excluded(klu) |
| `idasRoberts_ASAi_sps` |  | idas/serial | excluded(superlu) |
| `idasRoberts_FSA_sps` | `-sensi stg t` | idas/serial | excluded(superlu) |
| `idasRoberts_sps` |  | idas/serial | excluded(superlu) |

> ### Correction — three of these "absent" backends are installed
>
> An earlier revision of this file stated that "MS-MPI, PETSc, hypre, KLU,
> SuperLU, LAPACK and any Fortran compiler are genuinely absent". That was
> wrong, and the error was mine: I probed a handful of default install paths
> and generalised from the misses. Intel oneAPI is installed on this machine
> and supplies three of them:
>
> | claimed absent | actually present |
> |---|---|
> | LAPACK | **oneMKL** 2025.3 / 2026.1 / latest — `mkl_lapack.h`, ILP64 libraries |
> | MPI | **Intel MPI** 2021.17 / 2021.18 — `mpi.h`, `impi.lib`, `mpiexec.exe` |
> | Fortran compiler | **`ifx`** 2026.1.1 and 2025.3 |
>
> Genuinely absent, still: PETSc, hypre, KLU/SuiteSparse, SuperLU_MT,
> Trilinos, Ginkgo, RAJA, Kokkos. A CUDA toolkit (v13.0) is also present.
>
> **Consequence: the failure counts below understate what this machine can
> build.** With oneMKL the four `*L` examples can link real LAPACK instead of
> the documented substitution, and with Intel MPI the 34 files that failed on
> a missing `mpi.h` can at least be attempted (the 5 PETSc ones would still
> fail on PETSc). Rebuilding with those enabled has **not** been done yet, so
> every "missing header" row below remains an accurate record of *this* build
> — it is just not the best this machine could do.

## Directories outside the comparison

Every one needs a backend present on neither side. Counts are C source files.

| directory | programs | requires |
|---|---:|---|
| `arkode/C_klu`, `arkode/C_superlu-mt` | 2 | KLU / SuperLU_MT |
| `arkode/C_manyvector`, `cvode/C_mpimanyvector` | 2 | ManyVector (+ MPI) |
| `arkode/C_openmp`, `cvode/C_openmp`, `cvodes/C_openmp`, `ida/C_openmp`, `idas/C_openmp`, `kinsol/C_openmp` | 9 | OpenMP N_Vector |
| `arkode/C_openmpdev`, `cvode/C_openmpdev` | 4 | OpenMP device offload |
| `arkode/C_parallel`, `cvode/parallel`, `cvodes/parallel`, `ida/parallel`, `idas/parallel`, `kinsol/parallel` | 28 | MPI |
| `arkode/C_parhyp`, `cvode/parhyp` | 2 | *hypre* |
| `arkode/C_petsc`, `cvode/petsc`, `ida/petsc` | 5 | PETSc |
| C++ sources (`*.cpp`) | 46 | CUDA / HIP / SYCL / RAJA / Kokkos / Ginkgo / Trilinos / C++ interface |
| Fortran sources (`*.f90`) | 51 | Fortran 2003 interface |
| CUDA sources (`*.cu`) | 7 | CUDA |

The Rust port excludes all of these by specification: it is serial-only, with
no MPI, GPU, KLU, SuperLU, LAPACK, Fortran or XBraid backend. Porting them
would mean first porting those backends, which is out of scope for a
`std`-only translation.
