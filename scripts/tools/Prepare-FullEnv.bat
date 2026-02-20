@echo off
setlocal
REM Prepare-FullEnv.bat - Prepara entorno: Docker + API/clientes (contrato tools, Rust por defecto)
REM Ejecutar desde la raiz del repositorio.

set "SCRIPT_DIR=%~dp0"
set "REPO_ROOT=%SCRIPT_DIR%..\.."
cd /d "%REPO_ROOT%"

set "RUST_EXE=%SCRIPT_DIR%prepare_full_env.exe"
if exist "%RUST_EXE%" (
    "%RUST_EXE%" %*
    endlocal
    exit /b %ERRORLEVEL%
)

set "PS_SCRIPT=%SCRIPT_DIR%Prepare-FullEnv.ps1"
where pwsh >nul 2>&1
if %ERRORLEVEL% equ 0 (
    pwsh -NoProfile -ExecutionPolicy Bypass -File "%PS_SCRIPT%" %*
) else (
    powershell -NoProfile -ExecutionPolicy Bypass -File "%PS_SCRIPT%" %*
)
endlocal
