@echo off
setlocal
REM Pre-PR: push + crear PR (push_and_create_pr.exe). Cápsula: paths.skillCapsules.finalizar-git

set "SCRIPT_DIR=%~dp0"
set "REPO_ROOT=%SCRIPT_DIR%..\..\..\"
cd /d "%REPO_ROOT%"

set "RUST_EXE=%SCRIPT_DIR%push_and_create_pr.exe"
if exist "%RUST_EXE%" (
    "%RUST_EXE%" %*
    endlocal
    exit /b %ERRORLEVEL%
)

echo ERROR: push_and_create_pr.exe no encontrado. Ejecute scripts/skills-rs/install.ps1
endlocal
exit /b 1
