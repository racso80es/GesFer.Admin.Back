@echo off
setlocal
REM Invoke-MySqlSeeds.bat - Migraciones EF y seeds Admin (MySQL).
REM Ejecutar desde la raiz del repositorio.

set "SCRIPT_DIR=%~dp0"
set "PS_SCRIPT=%SCRIPT_DIR%Invoke-MySqlSeeds.ps1"
cd /d "%SCRIPT_DIR%..\.."
where pwsh >nul 2>&1
if %ERRORLEVEL% equ 0 (
    pwsh -NoProfile -ExecutionPolicy Bypass -File "%PS_SCRIPT%" %*
) else (
    powershell -NoProfile -ExecutionPolicy Bypass -File "%PS_SCRIPT%" %*
)
endlocal
