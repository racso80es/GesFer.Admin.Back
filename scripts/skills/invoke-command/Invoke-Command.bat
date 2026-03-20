@echo off
setlocal
REM Launcher humano: invoke-command (Rust en la misma carpeta). La IA debe invocar invoke_command.exe con JSON en stdin.
REM Cápsula: paths.skillCapsules.invoke-command

set "SCRIPT_DIR=%~dp0"
set "REPO_ROOT=%SCRIPT_DIR%..\..\..\"
cd /d "%REPO_ROOT%"

set "RUST_EXE=%SCRIPT_DIR%invoke_command.exe"
if exist "%RUST_EXE%" (
    "%RUST_EXE%" %*
    endlocal
    exit /b %ERRORLEVEL%
)

echo ERROR: invoke_command.exe no encontrado. Ejecute scripts/skills-rs/install.ps1
endlocal
exit /b 1
