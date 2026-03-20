@echo off
setlocal
REM Launcher humano: postman_mcp_validation.exe. Agente: JSON stdin/stdout cuando exista el binario.

set "SCRIPT_DIR=%~dp0"
set "REPO_ROOT=%SCRIPT_DIR%..\..\.."
cd /d "%REPO_ROOT%"

set "RUST_EXE=%SCRIPT_DIR%postman_mcp_validation.exe"
if exist "%RUST_EXE%" (
    set "GESFER_REPO_ROOT=%REPO_ROOT%"
    "%RUST_EXE%" %*
    endlocal
    exit /b %ERRORLEVEL%
)

echo ERROR: postman_mcp_validation.exe no encontrado. Implementación Rust pendiente o ejecute install del tool.
endlocal
exit /b 1
