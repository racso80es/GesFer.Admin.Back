@echo off
setlocal
REM Invoke-MySqlSeeds.bat - Migraciones EF y seeds Admin (contrato tools, Rust en bin/ si existe)
REM Capsula: scripts/tools/invoke-mysql-seeds/

set "SCRIPT_DIR=%~dp0"
set "REPO_ROOT=%SCRIPT_DIR%..\..\.."
cd /d "%REPO_ROOT%"

set "RUST_EXE=%SCRIPT_DIR%bin\invoke_mysql_seeds.exe"
if exist "%RUST_EXE%" (
    set "GESFER_REPO_ROOT=%REPO_ROOT%"
    "%RUST_EXE%" %*
    endlocal
    exit /b %ERRORLEVEL%
)

set "PS_SCRIPT=%SCRIPT_DIR%Invoke-MySqlSeeds.ps1"
where pwsh >nul 2>&1
if %ERRORLEVEL% equ 0 (
    pwsh -NoProfile -ExecutionPolicy Bypass -File "%PS_SCRIPT%" %*
) else (
    powershell -NoProfile -ExecutionPolicy Bypass -File "%PS_SCRIPT%" %*
)
endlocal
