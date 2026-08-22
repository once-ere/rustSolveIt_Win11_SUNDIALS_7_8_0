@echo off
rem posim_notebook.cmd — Windows launcher for a dynamic notebook by bare name.
rem
rem   tools\posim_notebook.cmd kepler_orbit
rem   tools\posim_notebook.cmd routh_rectangle_diagonal
rem   tools\posim_notebook.cmd --list
rem
rem The Windows twin of tools/posim_notebook (the bash original, which still
rem works from Git Bash). Resolves <name> against dynamic_notebooks\<name>.posim
rem in this repository and hands it to posim's --notebook mode, which loads
rem every cell, opens the scene window, and leaves you at an interactive In[]
rem prompt. Press Start in the window (or type `scene start`) to run.
rem
rem Works from any working directory: the repository root is derived from this
rem script's own location, not from %CD%.
rem
rem Extra arguments are passed through to posim. Environment variables are
rem inherited as usual, so the headless form is:
rem
rem   set POSIM_NO_BROWSER=1
rem   tools\posim_notebook.cmd kepler_orbit
setlocal

set "TOOLS_DIR=%~dp0"
for %%I in ("%TOOLS_DIR%..") do set "ROOT=%%~fI"
set "NB_DIR=%ROOT%\dynamic_notebooks"

if "%~1"=="" goto usage
if "%~1"=="-h" goto usage_ok
if "%~1"=="--help" goto usage_ok
if "%~1"=="-l" goto list_ok
if "%~1"=="--list" goto list_ok

set "ARG=%~1"

rem --- resolve the notebook: a real path, a bare name, or name.posim
if exist "%ARG%" (
  set "NB=%ARG%"
) else (
  set "BARE=%ARG%"
  if /i "%ARG:~-6%"==".posim" set "BARE=%ARG:~0,-6%"
  call set "NB=%NB_DIR%\%%BARE%%.posim"
)

if not exist "%NB%" (
  echo posim_notebook: no such notebook: %ARG% 1>&2
  echo   looked for: %NB% 1>&2
  echo. 1>&2
  echo Available notebooks: 1>&2
  call :list 1>&2
  exit /b 1
)

rem --- run it. cargo is used (not a prebuilt binary) so a stale target\ cannot
rem     silently shadow current sources; --release matches the documented form.
cd /d "%ROOT%"
shift
set "EXTRA="
:collect
if "%~1"=="" goto run
set "EXTRA=%EXTRA% %1"
shift
goto collect
:run
cargo run -p posim --release -- --notebook "%NB%"%EXTRA%
exit /b %ERRORLEVEL%

:list
if exist "%NB_DIR%" (
  for %%F in ("%NB_DIR%\*.posim") do echo   %%~nF
)
exit /b 0

:list_ok
call :list
exit /b 0

:usage
call :usage_text
exit /b 2

:usage_ok
call :usage_text
exit /b 0

:usage_text
echo usage: posim_notebook ^<name^> [extra posim args...]
echo        posim_notebook --list
echo.
echo Launches dynamic_notebooks\^<name^>.posim in posim's interactive notebook
echo mode. ^<name^> may also be given as "^<name^>.posim" or as a path to any
echo .posim file.
echo.
echo Available notebooks:
call :list
exit /b 0
