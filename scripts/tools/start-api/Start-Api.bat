@echo off
setlocal
if not defined GESFER_REPO_ROOT (
    for %%I in ("%~dp0..\..\..") do set "GESFER_REPO_ROOT=%%~fI"
)
set "SCRIPT_DIR=%~dp0"
set "BIN_EXE=%SCRIPT_DIR%start_api.exe"
if exist "%BIN_EXE%" (
    "%BIN_EXE%" %*
    endlocal
    exit /b %ERRORLEVEL%
)
echo ERROR: start_api.exe no encontrado. Ejecute scripts/tools-rs/install.ps1
endlocal
exit /b 1
