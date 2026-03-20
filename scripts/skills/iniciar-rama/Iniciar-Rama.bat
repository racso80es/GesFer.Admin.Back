@echo off
setlocal
REM Launcher humano: iniciar-rama. Uso: Iniciar-Rama.bat feat mi-feature
REM Cápsula: paths.skillCapsules.iniciar-rama

set "SCRIPT_DIR=%~dp0"
set "REPO_ROOT=%SCRIPT_DIR%..\..\"
cd /d "%REPO_ROOT%"

set "RUST_EXE=%SCRIPT_DIR%iniciar_rama.exe"
if exist "%RUST_EXE%" (
    "%RUST_EXE%" %*
    endlocal
    exit /b %ERRORLEVEL%
)

echo ERROR: iniciar_rama.exe no encontrado. Ejecute scripts/skills-rs/install.ps1
endlocal
exit /b 1
