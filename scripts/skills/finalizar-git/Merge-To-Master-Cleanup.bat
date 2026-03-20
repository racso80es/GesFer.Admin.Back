@echo off
setlocal
REM Post-PR: launcher humano merge_to_master_cleanup. Cápsula: paths.skillCapsules.finalizar-git

set "SCRIPT_DIR=%~dp0"
set "REPO_ROOT=%SCRIPT_DIR%..\..\..\"
cd /d "%REPO_ROOT%"

set "RUST_EXE=%SCRIPT_DIR%merge_to_master_cleanup.exe"
if exist "%RUST_EXE%" (
    "%RUST_EXE%" %*
    endlocal
    exit /b %ERRORLEVEL%
)

echo ERROR: merge_to_master_cleanup.exe no encontrado. Ejecute scripts/skills-rs/install.ps1
endlocal
exit /b 1
