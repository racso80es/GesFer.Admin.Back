@echo off
setlocal
if not defined GESFER_REPO_ROOT (
    for %%I in ("%~dp0..\..\..") do set "GESFER_REPO_ROOT=%%~fI"
)
set "SCRIPT_DIR=%~dp0"
set "BIN_EXE=%SCRIPT_DIR%run_test_e2e_local.exe"
if exist "%BIN_EXE%" (
    "%BIN_EXE%" --output-json %*
    endlocal
    exit /b %ERRORLEVEL%
)
echo ERROR: run_test_e2e_local.exe no encontrado. Ejecute scripts\tools-rs\install.ps1
endlocal
exit /b 1
