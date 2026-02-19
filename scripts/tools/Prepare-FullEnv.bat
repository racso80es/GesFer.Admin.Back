@echo off
setlocal
REM Prepare-FullEnv.bat - Prepara entorno: Docker + API/clientes segun scripts/tools/prepare-env.json
REM Ejecutar desde la raiz del repositorio o desde scripts/tools (el .ps1 resuelve la raiz).

set "SCRIPT_DIR=%~dp0"
set "PS_SCRIPT=%SCRIPT_DIR%Prepare-FullEnv.ps1"

REM Ir a la raiz del repo (dos niveles arriba de scripts/tools)
cd /d "%SCRIPT_DIR%..\.."

REM Invocar PowerShell 7+ si esta en path; si no, usar pwsh o powershell
where pwsh >nul 2>&1
if %ERRORLEVEL% equ 0 (
    pwsh -NoProfile -ExecutionPolicy Bypass -File "%PS_SCRIPT%" %*
) else (
    powershell -NoProfile -ExecutionPolicy Bypass -File "%PS_SCRIPT%" %*
)
endlocal
