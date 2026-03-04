@echo off
setlocal
REM Finalizar-Proceso.bat - Skill FinalizarProceso fase post_pr (punto de entrada "finalizar proceso")
REM Capsula: paths.skillCapsules.finalizar-proceso (scripts/skills/finalizar-proceso/)
REM Uso: Finalizar-Proceso.bat [rama]  o  Finalizar-Proceso.ps1 -BranchName "feat/xxx" [-NoDeleteRemote]

set "SCRIPT_DIR=%~dp0"
set "REPO_ROOT=%SCRIPT_DIR%..\..\"
cd /d "%REPO_ROOT%"

set "PS_SCRIPT=%SCRIPT_DIR%Finalizar-Proceso.ps1"
if "%~1"=="" (
    powershell -NoProfile -ExecutionPolicy Bypass -File "%PS_SCRIPT%"
) else (
    powershell -NoProfile -ExecutionPolicy Bypass -File "%PS_SCRIPT%" -BranchName "%~1"
)
endlocal
exit /b %ERRORLEVEL%
