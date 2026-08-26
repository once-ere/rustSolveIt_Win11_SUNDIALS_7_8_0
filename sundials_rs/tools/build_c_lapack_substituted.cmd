@echo off
REM ===========================================================================
REM build_c_lapack_substituted.cmd
REM
REM Builds the four *L examples against the native dense/band solvers.
REM
REM cvAdvDiff_bndL, cvRoberts_dnsL, cvsAdvDiff_bndL and cvsRoberts_dnsL are the
REM only in-scope serial examples that call a LAPACK linear solver. No LAPACK is
REM installed here, so the main build skips them. The Rust port instead ports
REM them onto the native solvers, substituting exactly four tokens:
REM
REM   sunlinsol/sunlinsol_lapackdense.h -> sunlinsol/sunlinsol_dense.h
REM   sunlinsol/sunlinsol_lapackband.h  -> sunlinsol/sunlinsol_band.h
REM   SUNLinSol_LapackDense             -> SUNLinSol_Dense
REM   SUNLinSol_LapackBand              -> SUNLinSol_Band
REM
REM To compare like with like, this script applies the SAME four substitutions
REM to the C sources and builds them against the C library the main build
REM produced. Substituted sources go to logs\c-build\lapack-sub\; the upstream
REM tree is never written to.
REM
REM Run tools\build_c_examples.cmd first.
REM
REM Records into c-results\provenance\ :
REM   10-lapacksub-cmd.txt   the literal cl.exe command line used, and every
REM                          line that differs between original and substituted
REM   11-lapacksub-out.txt   compiler and linker output
REM
REM SPDX-License-Identifier: BSD-3-Clause
REM ===========================================================================

setlocal enabledelayedexpansion
set "VSROOT=C:\Program Files\Microsoft Visual Studio\18\Professional"
set "HERE=%~dp0.."
set "SRC=%~1"
if "%SRC%"=="" set "SRC=C:\Users\youruser\Developer\sundials-7.8.0"
set "BUILD=%HERE%\logs\c-build"
set "SUB=%BUILD%\lapack-sub"
set "PROV=%HERE%\c-results\provenance"
set "CMDLOG=%PROV%\10-lapacksub-cmd.txt"
set "OUTLOG=%PROV%\11-lapacksub-out.txt"

call "%VSROOT%\VC\Auxiliary\Build\vcvars64.bat" >nul
if errorlevel 1 exit /b 1
if not exist "%SUB%" mkdir "%SUB%"
if not exist "%PROV%" mkdir "%PROV%"

echo == token substitution applied to each source ==> "%CMDLOG%"
echo sunlinsol/sunlinsol_lapackdense.h -^> sunlinsol/sunlinsol_dense.h>> "%CMDLOG%"
echo sunlinsol/sunlinsol_lapackband.h  -^> sunlinsol/sunlinsol_band.h>> "%CMDLOG%"
echo SUNLinSol_LapackDense             -^> SUNLinSol_Dense>> "%CMDLOG%"
echo SUNLinSol_LapackBand              -^> SUNLinSol_Band>> "%CMDLOG%"
echo.>> "%CMDLOG%"
echo == compiler ==>> "%CMDLOG%"
where cl >> "%CMDLOG%"

echo (start) > "%OUTLOG%"

for %%P in (cvode\serial\cvRoberts_dnsL cvode\serial\cvAdvDiff_bndL cvodes\serial\cvsRoberts_dnsL cvodes\serial\cvsAdvDiff_bndL) do (
  set "NAME=%%~nP"
  echo === !NAME! ===
  echo === !NAME! ===>> "%OUTLOG%"

  powershell -NoProfile -Command "(Get-Content '%SRC%\examples\%%P.c' -Raw) -replace 'sunlinsol/sunlinsol_lapackdense.h','sunlinsol/sunlinsol_dense.h' -replace 'sunlinsol/sunlinsol_lapackband.h','sunlinsol/sunlinsol_band.h' -replace 'SUNLinSol_LapackDense','SUNLinSol_Dense' -replace 'SUNLinSol_LapackBand','SUNLinSol_Band' | Set-Content '%SUB%\!NAME!.c' -NoNewline"

  echo.>> "%CMDLOG%"
  echo == !NAME!: every line that differs from the upstream source ==>> "%CMDLOG%"
  powershell -NoProfile -Command "Compare-Object (Get-Content '%SRC%\examples\%%P.c') (Get-Content '%SUB%\!NAME!.c') | ForEach-Object { $_.SideIndicator + ' ' + $_.InputObject }" >> "%CMDLOG%"

  echo.>> "%CMDLOG%"
  echo == !NAME!: compile and link command line ==>> "%CMDLOG%"
  echo cl /nologo /O2 /Ob2 /DNDEBUG /MD /DWIN32 /D_WINDOWS ^^>> "%CMDLOG%"
  echo    /DSUNDIALS_STATIC_DEFINE /D_CRT_SECURE_NO_WARNINGS ^^>> "%CMDLOG%"
  echo    /I"%SRC%\include" /I"%BUILD%\include" ^^>> "%CMDLOG%"
  echo    "%SUB%\!NAME!.c" ^^>> "%CMDLOG%"
  echo    /Fo"%SUB%\!NAME!.obj" /Fe"%BUILD%\bin\!NAME!.exe" ^^>> "%CMDLOG%"
  echo    /link /LIBPATH:"%BUILD%\bin" ^^>> "%CMDLOG%"
  echo    sundials_cvode_static.lib sundials_cvodes_static.lib ^^>> "%CMDLOG%"
  echo    sundials_core_static.lib sundials_nvecserial_static.lib ^^>> "%CMDLOG%"
  echo    sundials_sunmatrixdense_static.lib sundials_sunmatrixband_static.lib ^^>> "%CMDLOG%"
  echo    sundials_sunlinsoldense_static.lib sundials_sunlinsolband_static.lib ^^>> "%CMDLOG%"
  echo    sundials_sunnonlinsolnewton_static.lib ^^>> "%CMDLOG%"
  echo    sundials_sunnonlinsolfixedpoint_static.lib>> "%CMDLOG%"

  cl /nologo /O2 /Ob2 /DNDEBUG /MD /DWIN32 /D_WINDOWS ^
     /DSUNDIALS_STATIC_DEFINE /D_CRT_SECURE_NO_WARNINGS ^
     /I"%SRC%\include" /I"%BUILD%\include" ^
     "%SUB%\!NAME!.c" ^
     /Fo"%SUB%\!NAME!.obj" /Fe"%BUILD%\bin\!NAME!.exe" ^
     /link /LIBPATH:"%BUILD%\bin" ^
     sundials_cvode_static.lib sundials_cvodes_static.lib ^
     sundials_core_static.lib sundials_nvecserial_static.lib ^
     sundials_sunmatrixdense_static.lib sundials_sunmatrixband_static.lib ^
     sundials_sunlinsoldense_static.lib sundials_sunlinsolband_static.lib ^
     sundials_sunnonlinsolnewton_static.lib ^
     sundials_sunnonlinsolfixedpoint_static.lib >> "%OUTLOG%" 2>&1
  if errorlevel 1 echo   FAILED !NAME!
)
echo OK - see c-results\provenance\10-lapacksub-cmd.txt
endlocal
