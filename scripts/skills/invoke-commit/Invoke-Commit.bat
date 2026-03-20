@echo off
setlocal
REM Launcher humano invoke_commit. Cápsula: paths.skillCapsules.invoke-commit

set "SCRIPT_DIR=%~dp0"
set "REPO_ROOT=%SCRIPT_DIR%..\..\..\"
cd /d "%REPO_ROOT%"

set "RUST_EXE=%SCRIPT_DIR%invoke_commit.exe"
if exist "%RUST_EXE%" (
    "%RUST_EXE%" %*
    endlocal
    exit /b %ERRORLEVEL%
)

echo ERROR: invoke_commit.exe no encontrado. Ejecute scripts/skills-rs/install.ps1
endlocal
exit /b 1
