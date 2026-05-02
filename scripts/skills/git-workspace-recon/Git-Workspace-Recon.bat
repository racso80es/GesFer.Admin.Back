@echo off
setlocal enabledelayedexpansion

REM Launcher humano opcional. Para agentes/IA, invocar git_workspace_recon.exe con JSON (capsule-json-io v2).

set "SCRIPT_DIR=%~dp0"
set "EXE=%SCRIPT_DIR%git_workspace_recon.exe"

if not exist "%EXE%" (
  echo [git-workspace-recon] Ejecutable no encontrado: "%EXE%"
  echo Compila y copia desde scripts\skills-rs\install.ps1
  exit /b 1
)

REM Pasa argumentos tal cual (modo CLI).
"%EXE%" %*
exit /b %ERRORLEVEL%

