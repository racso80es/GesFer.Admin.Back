@echo off
setlocal
REM Launcher humano: run_tests_local.exe en la cápsula. Agente: JSON stdin/stdout.

set "SCRIPT_DIR=%~dp0"
set "REPO_ROOT=%SCRIPT_DIR%..\..\.."
cd /d "%REPO_ROOT%"

set "RUST_EXE=%SCRIPT_DIR%run_tests_local.exe"
if exist "%RUST_EXE%" (
    set "GESFER_REPO_ROOT=%REPO_ROOT%"
    "%RUST_EXE%" %*
    endlocal
    exit /b %ERRORLEVEL%
)

echo ERROR: run_tests_local.exe no encontrado. Ejecute scripts/tools-rs/install.ps1 si el binario está en el proyecto.
endlocal
exit /b 1
