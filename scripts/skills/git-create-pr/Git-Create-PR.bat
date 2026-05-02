@echo off
setlocal

set "SCRIPT_DIR=%~dp0"
set "EXE=%SCRIPT_DIR%git_create_pr.exe"

if not exist "%EXE%" (
  echo [git-create-pr] Ejecutable no encontrado: "%EXE%"
  echo Compila y copia desde scripts\skills-rs\install.ps1
  exit /b 1
)

"%EXE%" %*
exit /b %ERRORLEVEL%

