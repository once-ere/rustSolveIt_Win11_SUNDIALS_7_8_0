# Windows 11 C-reference comparison gates — rebound_rs 5.1.1 / reboundx_rs 5.1.0

Date: 2026-09-09 (measurement session 16:45–16:55 local; the C reference library and
the eleven REBOUND harnesses were compiled earlier the same day, 08:01–08:02, by a
previous session whose build logs sit beside this file and are cited below).

Repository: `C:\Users\youruser\Developer\github\rustSolveIt_Win11_SUNDIALS_7_8_0\rustSolveIt_Win11_SUNDIALS_7_8_0`
(HEAD was `6d99370c036ad3b781661076430f6467869d75eb` throughout the measurements; at 16:57:47 a
concurrent job in this repository committed `a5082b27db384084f5304c1d5ba1dffc7765a28b`, which under the
measured trees changed only `.gitignore` and removed the 16 extensionless macOS harness binaries from
the index — no harness source, Rust source, example or notebook changed). Nothing under `rebound_rust/src`,
`reboundx_rust/src` or any `.md`/`.tex` was edited; no git commit/checkout/reset/rm was run.
Everything here is measurement plus the regenerated dump files and this log.

## 0. Verdicts at a glance

| Gate (guide section) | Measured on this machine | Verdict |
|---|---|---|
| libm differential, 200,000 samples × 9 functions (§15.0) | sin, cos, tan, atan2, sqrt, fmod, exp, log: 0 differences; pow(xp,-0.234): 60 of 200,000 differ (0.0300 %), every one exactly 1 ULP | reproduced |
| bs_pow differential, 200,000 samples (§16) | 56 of 200,000 differ (0.0280 %), all exactly 1 ULP; 8 constant `pow(0.02, 1/(2k+1))` values identical | reproduced |
| Integrator matrix, 63 configurations × 500 steps (§15.1) | 63 of 63 bit-identical | reproduced |
| ias15 default run, 1000 adaptive steps (§19 step 6) | 7/7 lines identical, SHA-256 equal | reproduced |
| Shearing sheet, 400 steps (§15.2) | init and final dumps byte-identical; final SHA-256 `75BDAAB7109F125192F56AEB0CCDCAC554AF88A165FE11075EC5A871178521F0` | reproduced |
| Simulationarchive round trips, whfast-usafe and ias15 (§15.5) | C→Rust, Rust→C and write-vs-write all bit-identical (6 comparisons) | reproduced |
| Web server blob loaded by C (§15.6) | 3,448-byte blob, C-loaded state identical to Rust's own dump (6/6 lines, SHA equal) | reproduced |
| Orbital derivatives, 65 functions × 2 configs (§15.3) | 130/130 lines identical (C file has CRLF line ends, Rust LF; byte-identical after CRLF→LF) | reproduced |
| Frequency analysis MFT/FMFT/FMFT2 (§15.4) | 30/30 lines identical, SHA equal | reproduced |
| add_fmt / datasets (§15.7) | 12/12 lines identical, SHA equal | reproduced |
| movetocom_var (defect 3 probe, §15.10) | 5/5 lines identical, SHA equal | reproduced |
| movetocom_var_test (stdout probe) | all 5 raw-bit fields identical; text differs only in C `%e` exponent style (`e+00`) vs Rust (`e0`) | reproduced |
| kepler_rectilinear (defect 1 probe, §15.10) | 11 cases: all 11 raw-bit AFTER lines identical; 6/11 full stdout identical, the 5 `vy=0` cases differ only in the header's exponent style | reproduced |
| REBOUNDx tides_spin pseudo / kozai / migration, short and long (§15.8) | 6 of 6 bit-identical | reproduced |
| REBOUNDx binary: Rust self round trip; Rust reads C file; C reads Rust file (§15.9) | 25/25 and 25/25 checks; files 6,392 bytes with 6,366 identical, the 26 differing bytes are the git-hash header field; C readback of both files identical (28/28 lines) | reproduced |
| 17 notebooks re-executed (`run_notebooks.py`) | `All 17 notebooks executed with no errors.` | done |

Nothing failed. The only things that did not come out *literally* as the guide words them
are listed in §7 (all cosmetic: text formatting, CRLF, a notebook tally quirk, file sizes of
regenerable `.bin` files, and the absence of a git commit id for the REBOUNDx tree).

## 1. Machine and toolchain

```
OS        Windows 11 Pro for Workstations 10.0.26200, x86-64
vcvars    "C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
          banner: ** Visual Studio 2026 Developer Command Prompt v18.0 ** / [vcvarsall.bat] Environment initialized for: 'x64'
cl        Microsoft (R) C/C++ Optimizing Compiler Version 19.51.36256 for x64
link      Microsoft (R) Incremental Linker Version 14.51.36256.0
make      GNU Make 3.81 (C:\Program Files (x86)\GnuWin32\bin\make.exe)
rustc     rustc 1.91.1 (ed61e7d7e 2025-11-07)
cargo     cargo 1.91.1 (ea2d97820 2025-10-10)
python    Python 3.14.5, nbformat 5.10.4, nbclient 0.10.2
pwsh      7.6.5 (the tool shell); the matrix sweep was run under Windows PowerShell as documented
```

Every `vcvars64.bat` invocation prints `'vswhere.exe' is not recognized as an internal or
external command` first; it is noise from the developer-prompt script and has no effect
(cl/lib/link are found; every build below exited 0).

## 2. Reference sources

| Tree | Location | Identity |
|---|---|---|
| REBOUND | `<repo>\rebound\rebound` = directory junction → `C:\Users\youruser\Developer\github\rustSolveIt_Win11_SUNDIALS_7_8_0\rebound\rebound` | `git log -1`: `dad5f97806ecbb408dcaff728851c64e67f9f6eb  Tue Aug 18 06:18:47 2026 -0700  Patch (#931)`; `git describe --tags`: `5.1.1-4-gdad5f978`; `src/rebound.c`: `reb_version_str = "5.1.1"`; branch `main`, remote `https://github.com/hannorein/rebound` |
| REBOUNDx | `<repo>\reboundx` = junction → `C:\Users\youruser\Developer\github\rustSolveIt_Win11_SUNDIALS_7_8_0\reboundx` | **not a git checkout** (no `.git`; the outer folder also holds `reboundx-main.zip`, the evident origin). `version.txt` = `5.1.0`; `src/core.c`: `rebx_version_str = "5.1.0"`; `changelog.md` tops out at "Version 5.1.0". No commit id can be reported. |

Both junctions already existed (created 08:00 by the previous session, `<JUNCTION>` per
`dir /AL`); both paths are gitignored (`/rebound/`, `/reboundx/`) and were not git-added.

## 3. Compiled artifacts: reused versus rebuilt

Rule applied: reuse only what a log proves was built with the documented flags; rebuild the rest.

**Reused (flags proven by log):**

* `librebound.dll` / `librebound.lib` (import library) and the 31 `*.obj` in `rebound\rebound\src`
  — `logs/win11_cref_rebound_make_20260909.log` records every per-file command, e.g.
  `cl -c /DBUILDINGLIBREBOUND /D_GNU_SOURCE /D_CRT_SECURE_NO_WARNINGS /D_CRT_NONSTDC_NO_WARNINGS /Ox /fp:precise -DSERVER -DGITHASH=dad5f97806ecbb408dcaff728851c64e67f9f6eb /Fo:rebound.obj rebound.c`
  with the cl 19.51.36256 banner after each, then `link` into `librebound.dll`, then the
  documented `problem.c` build of `rebound.exe`. SHA-256 of the copies in `rebound_rust\porttest`
  equal those in `rebound\rebound\src`:
  `D18EC6590FDA4F045A69B707A330C354465004CB9A95EB3037E8E7B497E5DD28 librebound.dll (794,112 bytes, 08:01)`,
  `67EAF87E7DA118B1A28A25FF4E2B7C1FB531AE348B231CA70188A5685E343C79 librebound.lib (65,930 bytes)`.
* The eleven REBOUND harness executables in `rebound_rust\porttest` (built 08:02) —
  `logs/win11_cref_harness_build_20260909.log` records each command verbatim, all of the form
  `"…\vcvars64.bat" >nul && cl /nologo /I"..\..\rebound\rebound\src" /D_GNU_SOURCE /D_CRT_SECURE_NO_WARNINGS /D_CRT_NONSTDC_NO_WARNINGS /Ox /fp:precise <name>.c librebound.lib /Fe:<name>.exe`
  (`libm_diff.c` and `bs_pow_diff.c` with `cl /nologo /Ox /fp:precise <name>.c /Fe:<name>.exe`;
  `problem_test.c` → `rebound_test.exe`). One warning only: `movetocom_var_test.c(28): warning C4477` (`%zu` with an int).
* The Rust side: `rebound_rust\target\release\examples\*.exe` and `reboundx_rust\target\release\examples\*.exe`
  (13 + 4 examples, built 07:40 by `logs/win11_build_20260909.log`). No cargo build was needed; the
  notebooks' own `cargo build --release --example …` cells reported the targets up to date.

**Rebuilt in this session (no log proved their flags):**

* `librebound_static.lib` (archive of the 31 make-built objects),
* all 31 REBOUNDx objects, the two MSVC VLA shims, and `libreboundx.lib`,
* the five REBOUNDx harness executables (relinked against the rebuilt libraries).

Stale copies were deleted first (`rm -f reboundx/src/*.obj reboundx/src/*.lib reboundx_rust/porttest/msvc_shim/*.obj rebound/rebound/src/librebound_static.lib`).

## 4. Commands run, in order (PowerShell; `$repo` is the repository path above)

Section 10a/19: static library from the make-built objects (cwd `$repo\rebound\rebound\src`):
```powershell
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && lib /nologo /OUT:librebound_static.lib *.obj'
Copy-Item librebound_static.lib "$repo\rebound_rust\porttest\librebound_static.lib" -Force
Copy-Item librebound_static.lib "$repo\reboundx_rust\porttest\librebound_static.lib" -Force
```
→ exit 0; `librebound_static.lib` 885,762 bytes (16:50:39); 31 objects archived.

Section 10b: REBOUNDx (cwd `$repo\reboundx\src`, 33 sources):
```powershell
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && cl /nologo /c /I"..\..\rebound\rebound\src" /I"." /D_GNU_SOURCE /D_CRT_SECURE_NO_WARNINGS /D_CRT_NONSTDC_NO_WARNINGS /DLIBREBOUNDX /Ox /fp:precise *.c'
```
→ cl exit 2, 31 objects produced; exactly the two documented VLA failures and nothing else:
```
gr_full.c(63): error C2057: expected constant expression
gr_full.c(63): error C2466: cannot allocate an array of constant size 0
gr_full.c(63): error C2133: 'a_const': unknown size
gr_full.c(185): error C2057: expected constant expression
gr_full.c(185): error C2466: cannot allocate an array of constant size 0
gr_full.c(185): error C2133: 'a_old': unknown size
interpolation.c(67): error C2057: expected constant expression
interpolation.c(67): error C2466: cannot allocate an array of constant size 0
interpolation.c(67): error C2133: 'u': unknown size
```
Shims (cwd `$repo\reboundx_rust\porttest\msvc_shim`), then copied in and archived:
```powershell
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && cl /nologo /c /I"..\..\..\rebound\rebound\src" /I"..\..\..\reboundx\src" /D_GNU_SOURCE /D_CRT_SECURE_NO_WARNINGS /D_CRT_NONSTDC_NO_WARNINGS /DLIBREBOUNDX /Ox /fp:precise gr_full.c interpolation.c'
Copy-Item gr_full.obj, interpolation.obj "$repo\reboundx\src\" -Force
Set-Location "$repo\reboundx\src"
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && lib /nologo /OUT:libreboundx.lib *.obj'
Copy-Item libreboundx.lib "$repo\reboundx_rust\porttest\libreboundx.lib" -Force
```
→ shims exit 0 (`gr_full.obj` 24,129 B, `interpolation.obj` 6,474 B); 33 objects; `libreboundx.lib` 334,960 bytes (16:50:55), exit 0.

REBOUNDx harnesses (cwd `$repo\reboundx_rust\porttest`), one per name in
`tides_spin_pseudo_c tides_spin_kozai_c tides_spin_migration_c rebx_binary_roundtrip_c rebx_binary_read_c`:
```powershell
cmd /c '"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >nul && cl /nologo /I"..\..\rebound\rebound\src" /I"..\..\reboundx\src" /D_GNU_SOURCE /D_CRT_SECURE_NO_WARNINGS /D_CRT_NONSTDC_NO_WARNINGS /Ox /fp:precise <name>.c libreboundx.lib librebound_static.lib /Fe:<name>.exe'
```
→ all five exit 0 (16:51:57–16:52:03). Expected `LNK4217`/`LNK4286` import-vs-static
warnings: 67, 68, 66, 57, 59. The three tides harnesses also emit `warning C4133 'function':
incompatible types - from 'void **' to 'rebx_node **'` (8, 8 and 16 times) from the harness
sources' `rebx_set_param_*` calls; they are the stock examples' own pointer-type looseness.

Measurements, REBOUND (cwd `$repo\rebound_rust\porttest`; every comparison is
`Compare-Object (Get-Content <c>) (Get-Content <rust>)` plus `Get-FileHash -Algorithm SHA256` on both):
```powershell
.\libm_diff.exe                      ; ..\target\release\examples\libm_diff.exe
.\bs_pow_diff.exe                    ; ..\target\release\examples\bs_pow_diff.exe
.\derivatives_test.exe               ; ..\target\release\examples\derivatives_test.exe
.\frequency_test.exe                 ; ..\target\release\examples\frequency_test.exe
.\addfmt_test.exe                    ; ..\target\release\examples\addfmt_test.exe
.\movetocom_var_c.exe                ; ..\target\release\examples\movetocom_var.exe
.\movetocom_var_test.exe             ; ..\target\release\examples\movetocom_var_test.exe        # stdout captured and compared
.\kepler_rectilinear_c.exe <vy> <n> <dt> ; ..\target\release\examples\kepler_rectilinear.exe <vy> <n> <dt>   # 11 cases, stdout compared
powershell -NoProfile -ExecutionPolicy Bypass -File .\run_integrator_matrix.ps1 500
.\integrators_test.exe               ; ..\target\release\examples\integrators_test.exe            # defaults: ias15, 1000 steps
Start-Process ..\target\release\examples\server_test.exe -WorkingDirectory (Get-Location).Path -PassThru -NoNewWindow -RedirectStandardOutput <scratch>\server_stdout.txt
Start-Sleep -Seconds 2
curl.exe -s http://localhost:12873/simulation --output served.bin
curl.exe -s http://localhost:12873/keyboard/81
.\archive_test.exe whfast load served.bin      # then Compare-Object archive_state_c.txt server_state_rust.txt
foreach ($integ in 'whfast-usafe','ias15') {
  .\archive_test.exe $integ write               ; Copy-Item archive_state_c.txt   <scratch>\arch_${integ}_c_write.txt
  ..\target\release\examples\archive_test.exe $integ continue      # Rust continues archive_c_<integ>.bin → archive_state_rust.txt
  ..\target\release\examples\archive_test.exe $integ write         ; Copy-Item archive_state_rust.txt <scratch>\arch_${integ}_rust_write.txt
  .\archive_test.exe $integ continue            # C continues archive_rust_<integ>.bin → archive_state_c.txt
}
.\rebound_test.exe 400               ; ..\target\release\examples\shearing_sheet_test.exe 400
Get-FileHash state_c_final.txt, state_rust_final.txt, state_c_init.txt, state_rust_init.txt -Algorithm SHA256
```
The shearing-sheet pair was run *after* the matrix sweep, as §15.1 instructs, so the
`state_*_final.txt` left on disk are the shearing-sheet dumps. `run_integrator_matrix.ps1` was
run with `-NoProfile` added, only to keep the user's PowerShell profile (which prints
Set-ExecutionPolicy errors and EMSDK banners, visible in the previous session's
`win11_cref_integrator_matrix_20260909.log`) out of the transcript; the script itself is unchanged.

Measurements, REBOUNDx (cwd `$repo\reboundx_rust\porttest`); the long runs first, the short
(notebook-argument) runs last so the short C dumps are the ones left on disk:
```powershell
foreach ($t in @(@('pseudo','628.3185307179586'),@('kozai','100000.0'),@('migration','628.3185307179586'),
                 @('pseudo','62.83185307179586'),@('kozai','1000.0'),@('migration','62.83185307179586'))) {
    $n = $t[0]
    & ".\tides_spin_${n}_c.exe" $t[1] 2>$null
    & "..\target\release\examples\tides_spin_$n.exe" $t[1] 2>$null
    Compare-Object (Get-Content "state_${n}_c.txt") (Get-Content "state_${n}_rust.txt")
}
..\target\release\examples\rebx_binary_roundtrip.exe                         # Rust writes rebx_binary_roundtrip.bin, reads it back
.\rebx_binary_roundtrip_c.exe rebx_c_reference.bin                           # C writes rebx_c_reference.bin
$a=[IO.File]::ReadAllBytes("$pwd\rebx_binary_roundtrip.bin"); $b=[IO.File]::ReadAllBytes("$pwd\rebx_c_reference.bin")
$d=0; for($i=0;$i -lt $a.Length;$i++){ if($a[$i] -ne $b[$i]){ $d++ } }; "differing bytes: $d"
..\target\release\examples\rebx_binary_roundtrip.exe rebx_c_reference.bin    # Rust reads the C file, re-serializes to rebx_binary_roundtrip2.bin
.\rebx_binary_read_c.exe rebx_binary_roundtrip.bin 2>$null | Out-File -Encoding ascii readback_from_rust.txt
.\rebx_binary_read_c.exe rebx_c_reference.bin      2>$null | Out-File -Encoding ascii readback_from_c.txt
Compare-Object (Get-Content readback_from_rust.txt) (Get-Content readback_from_c.txt)
```

libm / pow analysis: a 76-line Python script (scratchpad, not committed) that reads
`libm_c.txt`/`libm_rust.txt` (9 hex columns per line: sin cos tan atan2 pow(xp,-0.234) sqrt fmod exp log)
and `bs_pow_c.txt`/`bs_pow_rust.txt` (`P k bits` / `E k i bits` lines), counts differing
fields per function and computes ULP distances as the difference of the sign-ordered 64-bit patterns.

Notebooks (Git Bash, cwd `rebound_rust\notebooks`):
```bash
PYTHONIOENCODING=utf-8 python run_notebooks.py 2>&1 | tee run_all.log
```

## 5. Measured results

### 5.0 libm differential (§15.0)

`libm_c.txt` and `libm_rust.txt`: 30,600,000 bytes each, 200,000 lines, 9 functions per line.
Wall time of the two dump programs: C 0.243 s, Rust 0.065 s.

```
libm: lines with any difference = 60
  sin              differ=     0  bit-identical on all 200000 samples
  cos              differ=     0  bit-identical on all 200000 samples
  tan              differ=     0  bit-identical on all 200000 samples
  atan2            differ=     0  bit-identical on all 200000 samples
  pow(xp,-0.234)   differ=    60 (0.0300%)  ULP distribution={1: 60}
  sqrt             differ=     0  bit-identical on all 200000 samples
  fmod             differ=     0  bit-identical on all 200000 samples
  exp              differ=     0  bit-identical on all 200000 samples
  log              differ=     0  bit-identical on all 200000 samples
   first pow mismatches: sample 4209:  C 3fd6473aac8c764d  Rust 3fd6473aac8c764c  (1 ULP)
                         sample 7132:  C 3fcc2aa8df9ea835  Rust 3fcc2aa8df9ea836  (1 ULP)
                         sample 7669:  C 3fcd99d9217856fb  Rust 3fcd99d9217856fa  (1 ULP)
                         sample 12245: C 3fcd4db3668fca2d  Rust 3fcd4db3668fca2e  (1 ULP)
                         sample 13496: C 3fca5d6f779ed403  Rust 3fca5d6f779ed402  (1 ULP)
```
So on this machine the general-sweep pow difference is 60/200,000 and **all 60 are exactly 1 ULP**
(the guide's table says "≤ 2 ULP" for this sweep; the measured maximum here is 1).
`cbrt`, named in the guide's prose, is not a column of this harness and was therefore not measured.

### 5.1 pow ULP distribution, BS-controller shapes (§16)

`bs_pow_c.txt`/`bs_pow_rust.txt`: 5,311,288 bytes, 200,008 lines each (8 `P` constants + 200,000 `E` samples).
```
bs_pow: mismatches=56 of 200000 E samples (0.0280%); P-constant mismatches=0
bs_pow: ULP distribution={1: 56}
bs_pow: mismatches per k (exponent 1/(2k+1))={1: 5, 2: 8, 3: 8, 4: 3, 5: 7, 6: 8, 7: 7, 8: 10}
   C: E 1 9857  3f9cab338c1d039b   Rust: E 1 9857  3f9cab338c1d039a   (1 ULP)
   C: E 1 16869 3fc40e962b2a42bf   Rust: E 1 16869 3fc40e962b2a42c0   (1 ULP)
   C: E 1 18238 3fcc12be5af5abce   Rust: E 1 18238 3fcc12be5af5abcd   (1 ULP)
   C: E 1 18251 3fcc29ba1822caff   Rust: E 1 18251 3fcc29ba1822cb00   (1 ULP)
   C: E 1 20069 3fd601e55bb1c9f7   Rust: E 1 20069 3fd601e55bb1c9f6   (1 ULP)
```

### 5.2 Integrator matrix (§15.1)

`run_integrator_matrix.ps1 500`: all 63 lines read `identical`; tally
`63 of 63 configurations bit-identical.` / `ALL CONFIGURATIONS BIT-IDENTICAL`; exit 0; 2.3 s wall.
Configurations: none, ias15, leapfrog(2,4,6,8), whfast + c11 c17 dh whds bary mk comp lazy usafe,
saba + 1 2 3 4 cm2 cl2 104 864 h844 h864 h1064 usafe, janus + 2 4 8 10, eos + 0-0…8-8 2-7 5-8 usafe,
mercurius + usafe c4 c5 inf hill01, bs + tight loose maxdt, trace + pbs ias15 hill1 perinone eta001.

Default run (`integrators_test.exe` with no arguments = ias15, order 2, 1000 adaptive steps):
C prints `ias15 done: t=9.12795929765748610e+01 steps=1000`, Rust `ias15 done: t=9.127959297657486e1 steps=1000`;
dumps 7/7 lines identical, 403 bytes each, SHA-256 equal:
```
integrator ias15 order 2 steps 1000
t 4056d1e4d9f0a524
dt 3fa3537087ef2705
steps_done 1000
0 3f5df609da13430e 3fa79310f9d996fc 3e569f91968800a0 bf522e2c8b5160f3 3f5de8846d5daaf3 be2802427e971ba9
1 bfcd48e14f403138 bfd9835510667b92 beb905bdca05cf24 3ff1c0f2111b724c bff534f8788ec76a 3ed209c8137e3d08
2 400627e350ce85e5 3fbf4f6d92af1671 bfb8314720a5503f 3fd6d8f73864e237 3fd73b31a28bd1af bf8cc9a5f7cb9fc1
```

### 5.3 Shearing sheet (§15.2)

```
> .\rebound_test.exe 400                                    [exit=0, 2.230 s]
    Toomre wavelength: 61.009899 | N after init: 1482 | final: t=1.91217633050229269e+04 steps=400 collisions=102533
> ..\target\release\examples\shearing_sheet_test.exe 400    [exit=0, 1.700 s]
    Toomre wavelength: 61.009899 | N after init: 1482 | final: t=1.91217633050229269e+04 steps=400 collisions=102533
75BDAAB7109F125192F56AEB0CCDCAC554AF88A165FE11075EC5A871178521F0  state_c_final.txt     (208,019 bytes, 1,488 lines)
75BDAAB7109F125192F56AEB0CCDCAC554AF88A165FE11075EC5A871178521F0  state_rust_final.txt  (208,019 bytes)
82B4013994FFF67315AA126CAA3E993B45A66CD42028C1D00558D7F92EAC86F1  state_c_init.txt      (207,859 bytes, 1,483 lines)
82B4013994FFF67315AA126CAA3E993B45A66CD42028C1D00558D7F92EAC86F1  state_rust_init.txt   (207,859 bytes)
Compare-Object differing lines: final = 0, init = 0
header: N 1482 | t 40d2ac70d9fd4f96 1.91217633050229269e+04 | steps_done 400 | collisions_log_n 102533 | collisions_plog 40de083d75e3b756 3.07529603204050436e+04 | rand_seed 2089691152
```

### 5.4 Simulationarchive round trips (§15.5)

| Integrator | C write(300) vs Rust continue(200+100 from `archive_c_*.bin`) | Rust write(300) vs C continue(200+100 from `archive_rust_*.bin`) | C write vs Rust write | dump size |
|---|---|---|---|---|
| whfast-usafe | 0 differing lines of 6, SHA equal | 0 of 6, SHA equal | 0 of 6, SHA equal | 375 bytes |
| ias15 | 0 of 6, SHA equal | 0 of 6, SHA equal | 0 of 6, SHA equal | 368 bytes |

Printed end times: whfast-usafe C `write done: t=3.000000e+00`, Rust `write done: t=2.9999999999999583e0`,
continuations `continue done: t=2.9999999999999583e0` (Rust) / `t=3.000000e+00` (C);
ias15 `t=2.802771e+01` (C) / `t=2.8027708766369308e1` (Rust) in both roles.
Archive files this run: `archive_c_whfast-usafe.bin` 5,108 B, `archive_rust_whfast-usafe.bin` 5,174 B,
`archive_c_ias15.bin` 15,832 B, `archive_rust_ias15.bin` 15,865 B (all gitignored; the C ias15
file was 15,799 B in the 08:04 run — its diff blobs include wall-clock fields, so its length is
not stable between runs while the state it restores is).

whfast-usafe write dump (both languages):
```
integrator whfast-usafe
t 4007ffffffffffa2
dt 3f847ae147ae147b
0 3f5fcda0470be2e1 3f5435fdd064559e 3e1e69530b75bc87 3f4911bd33147842 3f6256efdba07d8d 3e25b3498d269a32
1 bfd5d1b183b20951 3fd10ee734953f05 3e9f64fe2717dd95 bfe87a6c67a5c7a8 bffbd1338aa0b56d bec51f12e4a3c579
2 bfcb33e757ae0ef8 3fe64d254bb6f923 bf8335c80d25bd66 bff1933977fa3b10 bfeb788869d3b52b 3f4e05090606f604
```
ias15 write dump (both languages): `t 403c0717ebf55e62`, `dt 3fa26ccb18d75140`, particles
`0 3f5e8f3c854e48ed 3f8bdc73210305f4 3e2e1f327169ef07 3f510169be7a9e6a 3f5fe4ec920abddc 3e2bcc41c9bbdc98`,
`1 bfd0f7e9047a67b2 3fda53f316b3a589 3ea79c1649d2e48c bff09b0c874e2bfd bff72567fb072314 bec309412a3d1e61`,
`2 3feed36870393ee6 3fee0fe723748d81 bf70b0f2312daf2a bfe9dc16e96590f6 3fcd23268b095c2c bf81c098bf552344`.

### 5.5 Web server (§15.6)

Port 12873 free before start. `server_test.exe` (pid 25432) stdout:
`REBOUND Webserver listening on http://localhost:12873 (not secure) ...` / `Server shutting down...` / `server_test done`;
exited with code 0 after `/keyboard/81` (response `ok.`). `served.bin`: 3,448 bytes, header
`REBOUND Binary File. Version: 5.`, SHA-256 `47368E55E6C4E933044B8A3C114220D963448A926E1DC13084B92091CB1403A2`.
`.\archive_test.exe whfast load served.bin` → `load done: t=1.000000e+00`; `archive_state_c.txt` vs
`server_state_rust.txt`: 6/6 lines identical, 369 bytes each, SHA equal:
```
integrator whfast
t 3ff0000000000003
dt 3f847ae147ae147b
0 3f2a08377669dfcb 3ef690892da06f26 3dddeccf2f809470 3f3a7964460845eb 3f1194eb0684d3b6 3def785e11e372be
1 3ff66c2fbcf51898 3fde9f7c28c7afff 3e9bdd0169570dd0 bfd9d9d6b8ea8493 3fdbb54f1db495cd 3ea73867d4109fca
2 3ff7b456bd6113a2 3fe2175f3de59b6e 3f779725388cc038 bfdba62abc78c772 3fdfea6d652387e4 bf80d061ac4c1453
```

### 5.6 derivatives / frequency / addfmt / movetocom_var / movetocom_var_test / kepler_rectilinear

* **derivatives_test** (§15.3): `derivatives_c.txt written (65 functions x 2 configs)` / same from Rust;
  130/130 lines, Compare-Object differing lines = 0; bytes 17,028 (C) vs 16,898 (Rust): the C file
  has 130 CR bytes (harness opens with `fopen(...,"w")`, text mode → CRLF on Windows), the Rust file
  none; C text with CRLF→LF equals the Rust text byte-for-byte (`True`).
* **frequency_test** (§15.4): 30/30 lines, 0 differing, 546 bytes each, SHA-256 equal
  (`352C0162E64102ADAA22CAB6F43B7AE55211AA45F7FDEB6ACE87AE5440BBC018`).
* **addfmt_test** (§15.7): `addfmt done N=12` both; 12/12 lines, 0 differing, 1,454 bytes, SHA equal
  (`7551686E6E484F1376FD36518D6732A7FE51094A7C40F3E50E40D7880E72846C`).
* **movetocom_var** (§15.10 defect 3): both print `com.m 3ff003e8f2d96218 com.x 3f7351b0cb646c7f`;
  5/5 lines, 0 differing, 436 bytes, SHA equal (`07DBBC89144DC335C0D487BE94634A3DAD1F3449A830DC8098C121177436E7D3`).
* **movetocom_var_test** (stdout-only probe): 8 lines each side; raw Compare-Object reports 14
  differing entries (7 lines × 2 sides) because C prints `%e` exponents as `e+00`/`e-03` and Rust
  as `e0`/`e-3`; after rewriting only that exponent notation the outputs are identical (0 differing);
  the five raw-bit fields agree exactly:
  `3ff003e8f543dbc4 3f753880595c6070 3f2649e164cc8800 bfcd1838694a61b4 bfc6cd3aa79f4e0b`
  (`dm(particles)=1.00095458799999992e+00  dm(particles_var)=0.00000000000000000e+00` on both sides).
* **kepler_rectilinear** (§15.10 defect 1): 11 cases — the notebook's `1e-12 1 2.0` and the documented
  vy ∈ {0, 1e-12} × dt ∈ {0.1, 0.5, 2.0, 5.0, −2.0} at n = 20. All 11 `AFTER:` raw-bit lines identical;
  full stdout identical in 6/11 — the five `vy=0` cases differ only in the `BEFORE:` header
  (`vy=0.00000000000000000e+00` in C vs `vy=0.00000000000000000e0` in Rust). AFTER lines:
  ```
  vy=1e-12 n=1  dt=2.0 : x=401c000000000000 y=3d819799812dea11 vx=4008000000000000 vy=3d719799812dea11
  vy=0     n=20 dt=0.1 : x=401a25b7357aa447 y=0000000000000000 vx=40059fa62e631832 vy=0000000000000000
  vy=0     n=20 dt=0.5 : x=403be81c2429448b y=0000000000000000 vx=4005462a784bf839 vy=0000000000000000
  vy=0     n=20 dt=2.0 : x=405de2243c66f45a y=0000000000000000 vx=4007a61d075c3ff2 vy=0000000000000000
  vy=0     n=20 dt=5.0 : x=4070a5878f25b778 y=0000000000000000 vx=40052d6777d805d6 vy=0000000000000000
  vy=0     n=20 dt=-2.0: x=c05d333adc74ea26 y=0000000000000000 vx=40077be41df9a0c2 vy=0000000000000000
  vy=1e-12 n=20 dt=0.1 : x=401a25b7357aa447 y=3d811469ec588e58 vx=40059fa62e631832 vy=3d70d0e89e16a0f2
  vy=1e-12 n=20 dt=0.5 : x=403be81c2429448b y=3da4e90949f58e33 vx=4005462a784bf839 vy=3d70922885df1fe9
  vy=1e-12 n=20 dt=2.0 : x=405de2243c66f45a y=3dc5be612de023a0 vx=4007a61d075c3ff2 vy=3d715ac1eb46a876
  vy=1e-12 n=20 dt=5.0 : x=4070a5878f25b778 y=3dd9d4e1f6892fd5 vx=40052d6777d805d6 vy=3d707f467ab6b0ed
  vy=1e-12 n=20 dt=-2.0: x=c05d333adc74ea26 y=bdc58ae9f5fd14a3 vx=40077be41df9a0c2 vy=3d712cc3e93159e9
  ```
  (the `1e-12 1 2.0` case returned normally in Rust — the pre-fix hang did not recur.)

### 5.7 REBOUNDx tides_spin (§15.8)

| Run | tmax | C wall | Rust wall | lines / bytes | Verdict |
|---|---|---|---|---|---|
| pseudo (long) | 628.3185307179586 | 2.46 s | 0.86 s | 8 / 471 | BIT-IDENTICAL |
| kozai (long) | 100000.0 | 1.38 s | 1.61 s | 10 / 590 | BIT-IDENTICAL |
| migration (long) | 628.3185307179586 | 5.67 s | 1.76 s | 10 / 663 | BIT-IDENTICAL |
| pseudo (short) | 62.83185307179586 | 0.26 s | 0.10 s | 8 / 471 | BIT-IDENTICAL |
| kozai (short) | 1000.0 | 0.03 s | 0.03 s | 10 / 590 | BIT-IDENTICAL |
| migration (short) | 62.83185307179586 | 0.52 s | 0.19 s | 10 / 663 | BIT-IDENTICAL |

Every row: Compare-Object differing lines = 0, SHA-256 equal. Printed end times: pseudo/migration long
`t=6.28318530717958652e+02` (C) / `e2` (Rust), short `6.28318530717958623e+01`; kozai `1.00000000000000000e+05` / `e+03`.
C stderr (a warning per timestep) was discarded with `2>$null` as the guide instructs, so the C wall
times here are far below the 83.6 s / 163.9 s the older port-test document quotes with printing on.
Long-run dumps were kept as `state_*_c_long.txt` / `state_*_rust_long.txt`; the short-run dumps are
the `state_*_c.txt` / `state_*_rust.txt` the notebooks compare against. Short-run kozai C dump:
```
example tides_spin_kozai tmax 408f400000000000
t 408f400000000000
dt 3fdfb0353c9af096
N 3
p 0 4030d168588d7d87 4032c4b0f270001d 3f4aa9435c08259e bfb30eb4eb3d86b0 3fb0c5d3841351ea bed2f14dd73ad188 3ff0000000000000
Omega 0 3e998f76a30319ba 40538913bf87a2ad 402b902ad31ab5ba
p 1 40306d618f880608 40331d542ceaa826 bffee0985f754479 bfe8935dd2f4d053 3fa60b26b4ced946 3fc16277432751ca 3f0b099bb527bf8d
Omega 1 3f55ea2802e8a6fe 4076773dd448392d 404fb27709cf85c7
p 2 c030d19fdd718af2 c032c4f18c611f7f bf47666a9cc39061 3fb3114d6243b0be bfb0c5f8c4216e69 bec4de97f7caf91e 3ff0000000000000
Omega 2 none
```

### 5.8 REBOUNDx binary files (§15.9)

1. Rust writes `rebx_binary_roundtrip.bin` (6,392 bytes) and reads it back: `25 / 25 checks passed`, `ROUND TRIP PASSED`
   (registered_params 107 entries; forces gr_potential + central_force; operators modify_mass + drift; all
   parameter values/orders as listed in the harness output).
2. C writes `rebx_c_reference.bin` (6,392 bytes).
3. Byte comparison: differing bytes = 26, identical bytes = 6,366; differing offsets 37..62; C bytes there
   are ASCII `notavailable00000000000000` (the git-hash header field), Rust bytes there are 26 zero bytes.
4. Rust reads the C file: `reading back rebx_c_reference.bin instead (foreign binary)`, `25 / 25 checks passed`,
   `re-serializing the reloaded state is byte-identical (6392 bytes)`; `rebx_binary_roundtrip2.bin` (the
   re-serialization) vs Rust's own file: 0 differing bytes; vs the C file: the same 26 header bytes only.
5. C reads both files: `readback_from_rust.txt` vs `readback_from_c.txt` 28/28 lines, 0 differing, SHA equal
   (`1C94D1CE8964217E7AE4DFF34BB0E9E4691F10B0A8D595682C04AA2AC92FB59C`); contents include
   `gr.c 40c3a8a8f5f13410`, `p1.tau_mass 4636b0a8e891ffff`, `p1.primary -12345`,
   `p1.Omega 3e8421f5f40d8376 c2b6bcc41e900000 3fd3333333333334`, `p1.force gr_potential`, `p0.params 0`.

SHA-256: `rebx_binary_roundtrip.bin` = `rebx_binary_roundtrip2.bin` = `E3E84D8A973D78E8986B9958F3A732AB820EA6D042D1F3DE2B7429A83D16A4C5`;
`rebx_c_reference.bin` = `F2B6C521A9BAE402CB5CC64CF6DAD22C04FEAF3D43FFC08FF78C0BA36B2551C6`.

## 6. Notebooks

`rebound_rust/notebooks/run_all.log` (16:53:30–16:54:17):
```
Executing 17 notebook(s), per-cell timeout 1800s

  addfmt_test.ipynb                  ok      (  1.9s)
  archive_test.ipynb                 ok      (  1.5s)
  bs_pow_diff.ipynb                  ok      (  1.7s)
  derivatives_test.ipynb             ok      (  1.6s)
  frequency_test.ipynb               ok      (  1.5s)
  integrators_test.ipynb             ok      (  4.1s)
  kepler_rectilinear.ipynb           ok      (  1.5s)
  libm_diff.ipynb                    ok      (  2.0s)
  movetocom_var.ipynb                ok      (  4.1s)
  movetocom_var_test.ipynb           ok      (  4.1s)
  rebx_binary_roundtrip.ipynb        ok      (  1.7s)
  server_test.ipynb                  ok      (  7.7s)
  shearing_sheet.ipynb               ok      (  3.9s)
  shearing_sheet_test.ipynb          ok      (  3.3s)
  tides_spin_kozai.ipynb             ok      (  1.8s)
  tides_spin_migration.ipynb         ok      (  1.7s)
  tides_spin_pseudo.ipynb            ok      (  1.7s)

All 17 notebooks executed with no errors.
```
(The raw log also carries a zmq `RuntimeWarning: Proactor event loop…` line before the first
notebook and one `Assertion failed: Connection reset by peer [10054] (…signaler.cpp:345)` line
emitted by libzmq while the `tides_spin_migration` kernel shut down; both are kernel-transport
noise, the notebook itself executed and is marked `ok`, and no cell in any notebook has an error output.)

Final-cell outputs, verbatim:

`libm_diff.ipynb` (cell 4):
```
compared 200000 samples
functions that differ: {'bfcabf302f5ca28e': 1, 'bfedfd838955127b': 1, 'bfd52b5666c53c50': 1, 'bfef4df245737c2b': 1, '3fef3727aef55dcc': 1, 'bfeece6188189236': 1, 'bfe002f6c8d4b5af': 1, 'bfed8e7ee38f142a': 1, '3fce73725b707701': 1, '3fe6b63faec0fb24': 1, 'bfe899118c1c88fd': 1, '3fec44e2b0871bd5': 1, 'bfd5f1f907dbd78f': 1, '3fee3c90f29c4e5a': 1, 'bfdd214d972073fe': 1, 'bfef4a7640f40632': 1, 'bfb6577ccfbcdef2': 1, '3fe494cb7327b918': 1, 'bfe864e354146b96': 1, '3fea3de20fb12d3e': 1, '3fd59c1345f96960': 1, 'bfe39247ba08aca3': 1, '3fe09a51afa45c4d': 1, 'bfc6c1c5e17b0b90': 1, '3fc0d920d1f924ee': 1, 'bfe48c9f4a621043': 1, 'bfec2613b33ec158': 1, '3fec4f2241cafb7c': 1, '3fee0e791ed71179': 1, '3fc295022d5db637': 1, 'bfe5e2e9ce5ec8d9': 1, 'bfddeb4ef5da4782': 1, '3fec0a00c999fdfa': 1, '3fef3cd55abe89a5': 1, '3fe8374a0e1fde51': 1, 'bfef18c28c77834d': 1, '3f5998c9e0d6d754': 1, 'bfe84bc134326f07': 1, 'bfb62df7246c3225': 1, '3fef2efc6ea80ddc': 1, '3feff766bedd800a': 1, 'bfebaf831939a08b': 1, 'bfe38b0f09aaec5e': 1, '3fc64b9dbb0aae17': 1, '3fc565d05f828357': 1, 'bf96be247b8c3bf8': 1, 'bfeae0812597e4bb': 1, '3fe5e6672bbf76ea': 1, 'bfef9e90637fbba7': 1, 'bfed058425bcd6c2': 1, '3fddd140d27eec03': 1, '3fecb98c4b0d864c': 1, 'bfeff968f49f1e88': 1, '3f9e9f836f6380ae': 1, '3fcd52a01cc1b651': 1, '3f638d80bcd4021e': 1, 'bfe869da0d7f66d4': 1, '3fec01f4ef30c752': 1, 'bfefdf68c557d704': 1, '3fd867bec3162b27': 1}
```
That dictionary has 60 entries, one per differing line — the notebook's tally keys on the first
token of each line (`a.split()[0]`, which in this dump is the `sin` bit pattern, not a function
name), so it reports *which lines* differ rather than *which functions*. The per-function result
is the one in §5.0: only the pow column differs, 60 times.

`bs_pow_diff.ipynb` (cell 4):
```
samples: 200008   mismatches: 56  (0.0280%)
ULP distribution: {1: 56}
```
(`200008` because the notebook counts the 8 `P` constant lines as samples; the 56 mismatches are all `E` samples.)

`integrators_test.ipynb` (cell 3, after `integrators_test.exe whfast 2 500`):
```
integrator whfast order 2 steps 500
t 4013ffffffffffa1
dt 3f847ae147ae147b
steps_done 500
0 3f357d2bdbae1411 3f694f30d803150e 3e0f7ff82f5dd424 bf414ab36cf6a274 3f1ea2d921c213f4 be1434b6b2dddaf8
1 3ff45b0339a179bb bfe2d9ef0e68e546 beb2f1ea9fc72c63 3fe0e23dfb5562f1 3fd8866309cf14b6 3ea0725ceb7444f1
2 3fe18658ff5bffbb bff2822af7e258f6 3f9189105e041a12 3feba7a1c87a2c6f bfb9434f6e55e272 3f8018d772e04126
```
(this notebook prints the Rust dump only; the C comparison for whfast is the matrix row above.)

Other compare cells: `shearing_sheet_test.ipynb` — `Rust SHA-256: 75BDAAB7109F125192F56AEB0CCDCAC554AF88A165FE11075EC5A871178521F0`,
`C    SHA-256: 75BDAAB7109F125192F56AEB0CCDCAC554AF88A165FE11075EC5A871178521F0`,
`BIT-IDENTICAL: C and Rust agree on every bit`; `movetocom_var.ipynb`, `tides_spin_pseudo.ipynb`,
`tides_spin_kozai.ipynb`, `tides_spin_migration.ipynb` — each ends `BIT-IDENTICAL: C and Rust agree on every bit`;
`server_test.ipynb` — `/simulation returned 3448 bytes`, `header: b'REBOUND Binary File. Version: 5.'`,
`server exited cleanly`; `archive_test.ipynb` — `archive_rust_ias15.bin: 15865 bytes`,
`archive_rust_whfast-usafe.bin: 5174 bytes` plus the whfast-usafe dump; `frequency_test.ipynb` — decoded
frequencies `[0.3, 0.55, 0.11]` for FMFT/FMFT2 and `[0.299996, 0.550001, 0.11]` for MFT.
`derivatives_test.ipynb` and `addfmt_test.ipynb` end with an *empty* last cell: their generated
`head` cell is guarded by `OUTFILE`, which the generator sets to `None`, so it prints nothing by design.

## 7. Things to know / did not reproduce literally

1. **REBOUNDx commit id** cannot be given: the reference tree is an unpacked archive, not a clone (§2). Version 5.1.0 is confirmed from `version.txt`, `core.c` and the changelog.
2. **Line endings.** `derivatives_c.txt` is written in text mode by the C harness and therefore has CRLF on Windows (17,028 bytes vs 16,898 in the committed LF copy and in the Rust file). It is left as the harness produced it; content is byte-identical after CRLF→LF, and the notebooks' `compare()` normalises CRLF. `git status` shows it modified for this reason only (`git diff --ignore-cr-at-eol` is empty).
3. **Text-only differences in the two stdout probes** (`movetocom_var_test`, `kepler_rectilinear` with `vy=0`) are C `printf("%e")` exponent style (`e+00`) versus Rust's `{:e}` (`e0`); every raw-bit field matches.
4. **`libm_diff.ipynb`'s "functions that differ" line** is a per-line rather than per-function tally (see §6); the measurement it summarises (60 differing lines, all pow) is correct. Not changed here — this session was measurement only.
5. **`ref_c_ias15.txt` / `ref_rust_ias15.txt`** in the repository hold the *archive_test ias15 write* dump (t = `403c0717ebf55e62`, 368 bytes, SHA-256 `E0B628A0299B90B1F1E4EE3712293E0CA94E1F9B193FEC2CE9B182EA37BB3CF4`), not the `integrators_test.exe` default-run dump that §19 step 6 appears to describe. This run reproduced that committed content bit-exactly (from `archive_test.exe ias15 write` on both sides), and those files were left holding it; the default-run comparison is recorded in §5.2 instead. `archive_state_c.txt` was likewise left holding this run's whfast-usafe C write dump (identical to the committed content); `archive_state_rust.txt` was last written by `archive_test.ipynb` (whfast-usafe write, 375 bytes, identical to the committed content).
6. **`.bin` sizes** of the C-written archives vary between runs (15,799 vs 15,832 bytes for `archive_c_ias15.bin`) because the incremental blobs carry wall-clock fields; the restored states do not vary. All `.bin` files are gitignored.
7. **Working-tree state.** When this session started, `.gitignore` carried 16 uncommitted added lines ignoring the extensionless macOS harness binaries, and those 16 binaries (`rebound_rust/porttest/{addfmt_test,archive_test,bs_pow_diff,derivatives_test,frequency_test,integrators_test,kepler_rectilinear_c,libm_diff,movetocom_var_c,movetocom_var_test,problem_test}`, `reboundx_rust/porttest/{rebx_binary_read_c,rebx_binary_roundtrip_c,tides_spin_*_c}`) were staged as deleted (`D `) while still present on disk — left over from the earlier session. A concurrent job committed exactly those changes as `a5082b27` at 16:57:47 (the binaries remain on disk, now ignored). After that commit the footprint of this work in `git status` is: the 17 executed `rebound_rust/notebooks/*.ipynb` (modified — outputs written back by `run_notebooks.py`), `rebound_rust/porttest/derivatives_c.txt` (CRLF, item 2), this log and the twelve `win11_cref_*_20260909.log` files (untracked). This session ran no git write command.
8. **Timings** were taken while unrelated notebook jobs were running elsewhere in the repository; treat them as indicative. They match the guide within noise (shearing sheet C 2.2 s / Rust 1.7 s; kozai C 1.3 s / Rust 1.6 s).
9. **`-NoProfile`** was added to the documented `powershell -ExecutionPolicy Bypass -File .\run_integrator_matrix.ps1 500` invocation (transcript hygiene only, see §4).

## 8. Files now on disk (the dumps the notebooks read)

`rebound_rust\porttest` (SHA-256, size):
```
75bdaab7109f125192f56aeb0ccdcac554af88a165fe11075ec5a871178521f0  state_c_final.txt      208019   (shearing sheet, C)
75bdaab7109f125192f56aeb0ccdcac554af88a165fe11075ec5a871178521f0  state_rust_final.txt   208019   (rewritten by shearing_sheet_test.ipynb, identical)
82b4013994fff67315aa126caa3e993b45a66cd42028c1d00558d7f92eac86f1  state_c_init.txt       207859
82b4013994fff67315aa126caa3e993b45a66cd42028c1d00558d7f92eac86f1  state_rust_init.txt    207859
099663bc53fd50f1353a3e2cab18fb1c9bbaa65182ba74d28a6da1bffa9c486b  libm_c.txt           30600000
89d206add338ffadc6d88b1a0ace3625a33c74fd074d8cb962a56b3a6f63fc02  libm_rust.txt        30600000
e255acb0ce01bda2b6a948bf1411653df763947f93928a1eb6f7d7206d1b910a  bs_pow_c.txt          5311288
2243e3b3794eec91991db4427df3b68fd1e3ae1b38e2a6fb6cd78ac4c1ad5180  bs_pow_rust.txt       5311288
ea4c7f22361b7babeb646e4162c3bc6df8686968ba2338c328a65c0df586908f  derivatives_c.txt       17028   (CRLF)
55dd3c407e737aadec76af8a7b92179016d8480b8b773d7aa10fe70f6bc8122f  derivatives_rust.txt    16898
352c0162e64102adaa22cab6f43b7ae55211aa45f7fdeb6ace87ae5440bbc018  frequency_c.txt = frequency_rust.txt   546
7551686e6e484f1376fd36518d6732a7fe51094a7c40f3e50e40d7880e72846c  addfmt_c.txt = addfmt_rust.txt        1454
07dbbc89144dc335c0d487be94634a3dad1f3449a830dc8098c121177436e7d3  movetocom_var_c.txt = movetocom_var_rust.txt   436
cbd6dfe07ab3e0ac230702e9416e7ee7a343e1ef94975fe9bcb608ce6f419d2e  ref_c_whfast-usafe.txt = ref_rust_whfast-usafe.txt = archive_state_c.txt = archive_state_rust.txt   375
e0b628a0299b90b1f1e4ee3712293e0ca94e1f9b193fec2ce9b182ea37bb3cf4  ref_c_ias15.txt = ref_rust_ias15.txt   368   (archive ias15 write dump)
3379ea70d33a0351640302f27d9816c2e1f539b9f050c54e6da4f8cfb93a4e95  server_state_rust.txt     369   (rewritten by server_test.ipynb)
47368e55e6c4e933044b8a3c114220d963448a926e1dc13084b92091cb1403a2  served.bin               3448
matrix_c.txt / matrix_rust.txt 408 bytes (last matrix pair, trace-eta001)
archive_c_whfast-usafe.bin 5108, archive_rust_whfast-usafe.bin 5174, archive_c_ias15.bin 15832, archive_rust_ias15.bin 15865
```
`reboundx_rust\porttest`:
```
acc60530e284ca7ad447cecd250e41b1d7fbccf98c82f9b59f9995b28ff53da2  state_pseudo_c.txt = state_pseudo_rust.txt          471
3275d7abb5b5ae4d4d2759542f90e67cd6decd31dc9426b09b253db0e880a6c5  state_kozai_c.txt = state_kozai_rust.txt            590
b7c5e43c6e76b944664acb10ead05448371d0e81cd95cc0374c91be3b9e4079d  state_migration_c.txt = state_migration_rust.txt    663
004a290a2563eb607e92ee9f3c2456358dc84ab9038e7a603189ee665d5ed7ea  state_pseudo_c_long.txt = state_pseudo_rust_long.txt
1be14af9a36675e8c19d7c235108c2e8f96454568cb29025dac480275e85731b  state_kozai_c_long.txt = state_kozai_rust_long.txt
411b1fe18c8e5ce8f3a3223c94b53c9c9c248f77c133f7b995087ca8984538d2  state_migration_c_long.txt = state_migration_rust_long.txt
e3e84d8a973d78e8986b9958f3a732ab820ea6d042d1f3de2b7429a83d16a4c5  rebx_binary_roundtrip.bin = rebx_binary_roundtrip2.bin   6392
f2b6c521a9bae402cb5cc64cf6dad22c04feaf3d43ffc08ff78c0ba36b2551c6  rebx_c_reference.bin                                    6392
1c94d1ce8964217e7ae4dff34bb0e9e4691f10b0a8d595682c04aa2ac92fb59c  readback_from_rust.txt = readback_from_c.txt             682
```
Compiled artifacts left in place: `rebound_rust\porttest\{librebound.dll, librebound.lib, librebound_static.lib, 11 harness .exe}`,
`reboundx_rust\porttest\{librebound_static.lib, libreboundx.lib, msvc_shim\*.obj, 5 harness .exe}`,
`reboundx\src\{33 .obj, libreboundx.lib}`, `rebound\rebound\src\{31 .obj, librebound.dll/.lib/.exp, librebound_static.lib}`.
