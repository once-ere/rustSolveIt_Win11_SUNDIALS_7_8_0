@echo off
REM ===========================================================================
REM build_c_examples.cmd
REM
REM Builds SUNDIALS 7.8.0 and its serial C examples with Microsoft Visual
REM Studio 18 Professional, and records everything needed to audit the build.
REM
REM   tools\build_c_examples.cmd [<path-to-sundials-7.8.0>]
REM
REM Writes into c-results\provenance\ :
REM   00-environment.txt        host, tool paths, tool versions, INCLUDE/LIB
REM   01-configure-cmd.txt      the literal cmake configure command line
REM   02-configure-out.txt      everything cmake printed
REM   03-CMakeCache.txt         every cache variable cmake resolved
REM   04-compile_commands.json  exact cl.exe line per translation unit
REM   05-build-cmd.txt          the literal cmake --build command line
REM   06-build-out.txt          ninja -v: every compile and link as executed
REM
REM The upstream C tree is read-only; the build tree is logs\c-build.
REM
REM NOTE ON STYLE: no parenthesised redirect blocks are used below. INCLUDE and
REM LIB contain "Program Files (x86)", and a ")" inside such a block ends it
REM early, which silently corrupts the script.
REM
REM SPDX-License-Identifier: BSD-3-Clause
REM ===========================================================================

setlocal
set "VSROOT=C:\Program Files\Microsoft Visual Studio\18\Professional"
set "SRC=%~1"
if "%SRC%"=="" set "SRC=C:\Users\youruser\Developer\sundials-7.8.0"
set "HERE=%~dp0.."
set "BUILD=%HERE%\logs\c-build"
set "PROV=%HERE%\c-results\provenance"
set "P=%PROV%\00-environment.txt"

if not exist "%PROV%" mkdir "%PROV%"
if exist "%BUILD%" rmdir /s /q "%BUILD%"
mkdir "%BUILD%"

call "%VSROOT%\VC\Auxiliary\Build\vcvars64.bat" >nul
if errorlevel 1 exit /b 1

echo == how this file was produced ==> "%P%"
echo tools\build_c_examples.cmd, after calling>> "%P%"
echo "%VSROOT%\VC\Auxiliary\Build\vcvars64.bat">> "%P%"
echo.>> "%P%"
echo == host ==>> "%P%"
ver >> "%P%"
powershell -NoProfile -Command "(Get-CimInstance Win32_OperatingSystem).Caption" >> "%P%"
powershell -NoProfile -Command "(Get-CimInstance Win32_Processor).Name" >> "%P%"
echo.>> "%P%"
echo == build started (UTC) ==>> "%P%"
powershell -NoProfile -Command "(Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ssZ')" >> "%P%"
echo.>> "%P%"
echo == cl.exe path and banner ==>> "%P%"
where cl >> "%P%"
cl 2>> "%P%" >nul
echo.>> "%P%"
echo == link.exe path and banner ==>> "%P%"
where link >> "%P%"
link 2>> "%P%" >nul
echo.>> "%P%"
echo == cmake ==>> "%P%"
where cmake >> "%P%"
cmake --version >> "%P%"
echo.>> "%P%"
echo == ninja ==>> "%P%"
where ninja >> "%P%"
ninja --version >> "%P%"
echo.>> "%P%"
echo == toolchain selected by vcvars64 ==>> "%P%"
echo VSCMD_ARG_TGT_ARCH=%VSCMD_ARG_TGT_ARCH%>> "%P%"
echo VCToolsVersion=%VCToolsVersion%>> "%P%"
echo WindowsSDKVersion=%WindowsSDKVersion%>> "%P%"
echo UCRTVersion=%UCRTVersion%>> "%P%"
echo.>> "%P%"
echo == INCLUDE ==>> "%P%"
echo %INCLUDE%>> "%P%"
echo.>> "%P%"
echo == LIB ==>> "%P%"
echo %LIB%>> "%P%"
echo.>> "%P%"
echo == source tree ==>> "%P%"
echo %SRC%>> "%P%"

set "C1=%PROV%\01-configure-cmd.txt"
echo cmake -G Ninja -S "%SRC%" -B "%BUILD%" ^^> "%C1%"
echo   -DCMAKE_BUILD_TYPE=Release ^^>> "%C1%"
echo   -DCMAKE_C_COMPILER=cl ^^>> "%C1%"
echo   -DCMAKE_EXPORT_COMPILE_COMMANDS=ON ^^>> "%C1%"
echo   -DBUILD_SHARED_LIBS=OFF ^^>> "%C1%"
echo   -DBUILD_STATIC_LIBS=ON ^^>> "%C1%"
echo   -DEXAMPLES_ENABLE_C=ON ^^>> "%C1%"
echo   -DEXAMPLES_ENABLE_CXX=OFF ^^>> "%C1%"
echo   -DEXAMPLES_INSTALL=OFF ^^>> "%C1%"
echo   -DBUILD_TESTING=OFF ^^>> "%C1%"
echo   -DSUNDIALS_INDEX_SIZE=64 ^^>> "%C1%"
echo   -DSUNDIALS_PRECISION=double ^^>> "%C1%"
echo   -DENABLE_LAPACK=OFF -DENABLE_KLU=OFF -DENABLE_SUPERLUMT=OFF ^^>> "%C1%"
echo   -DENABLE_SUPERLUDIST=OFF -DENABLE_MPI=OFF -DENABLE_OPENMP=ON ^^>> "%C1%"
echo   -DENABLE_PTHREAD=OFF -DENABLE_HYPRE=OFF -DENABLE_PETSC=OFF ^^>> "%C1%"
echo   -DENABLE_TRILINOS=OFF -DENABLE_CUDA=OFF -DENABLE_HIP=OFF ^^>> "%C1%"
echo   -DENABLE_SYCL=OFF -DENABLE_RAJA=OFF -DENABLE_KOKKOS=OFF ^^>> "%C1%"
echo   -DENABLE_GINKGO=OFF -DENABLE_XBRAID=OFF -DENABLE_CALIPER=OFF ^^>> "%C1%"
echo   -DENABLE_ADIAK=OFF -DBUILD_FORTRAN_MODULE_INTERFACE=OFF>> "%C1%"

cmake -G Ninja -S "%SRC%" -B "%BUILD%" ^
  -DCMAKE_BUILD_TYPE=Release ^
  -DCMAKE_C_COMPILER=cl ^
  -DCMAKE_EXPORT_COMPILE_COMMANDS=ON ^
  -DBUILD_SHARED_LIBS=OFF ^
  -DBUILD_STATIC_LIBS=ON ^
  -DEXAMPLES_ENABLE_C=ON ^
  -DEXAMPLES_ENABLE_CXX=OFF ^
  -DEXAMPLES_INSTALL=OFF ^
  -DBUILD_TESTING=OFF ^
  -DSUNDIALS_INDEX_SIZE=64 ^
  -DSUNDIALS_PRECISION=double ^
  -DENABLE_LAPACK=OFF -DENABLE_KLU=OFF -DENABLE_SUPERLUMT=OFF ^
  -DENABLE_SUPERLUDIST=OFF -DENABLE_MPI=OFF -DENABLE_OPENMP=ON ^
  -DENABLE_PTHREAD=OFF -DENABLE_HYPRE=OFF -DENABLE_PETSC=OFF ^
  -DENABLE_TRILINOS=OFF -DENABLE_CUDA=OFF -DENABLE_HIP=OFF ^
  -DENABLE_SYCL=OFF -DENABLE_RAJA=OFF -DENABLE_KOKKOS=OFF ^
  -DENABLE_GINKGO=OFF -DENABLE_XBRAID=OFF -DENABLE_CALIPER=OFF ^
  -DENABLE_ADIAK=OFF -DBUILD_FORTRAN_MODULE_INTERFACE=OFF ^
  > "%PROV%\02-configure-out.txt" 2>&1
if errorlevel 1 goto :cfgfail

copy /y "%BUILD%\CMakeCache.txt" "%PROV%\03-CMakeCache.txt" >nul
copy /y "%BUILD%\compile_commands.json" "%PROV%\04-compile_commands.json" >nul

echo cmake --build "%BUILD%" --parallel -- -v > "%PROV%\05-build-cmd.txt"
cmake --build "%BUILD%" --parallel -- -v > "%PROV%\06-build-out.txt" 2>&1
if errorlevel 1 goto :bldfail

echo.>> "%P%"
echo == build finished (UTC) ==>> "%P%"
powershell -NoProfile -Command "(Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ssZ')" >> "%P%"
echo OK - provenance in c-results\provenance\
endlocal
exit /b 0

:cfgfail
echo CONFIGURE FAILED - see c-results\provenance\02-configure-out.txt
type "%PROV%\02-configure-out.txt"
endlocal
exit /b 1

:bldfail
echo BUILD FAILED - see c-results\provenance\06-build-out.txt
endlocal
exit /b 1
