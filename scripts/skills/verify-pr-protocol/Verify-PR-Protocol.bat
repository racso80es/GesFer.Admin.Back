@echo off
setlocal enabledelayedexpansion

REM Launcher humano opcional. Para agentes/IA, invocar verify_pr_protocol.exe con JSON (capsule-json-io v2).

set "SCRIPT_DIR=%~dp0"
set "EXE=%SCRIPT_DIR%verify_pr_protocol.exe"

if not exist "%EXE%" (
  echo [verify-pr-protocol] Ejecutable no encontrado: "%EXE%"
  echo Compila y copia desde scripts\skills-rs\install.ps1
  exit /b 1
)

"%EXE%" %*
exit /b %ERRORLEVEL%
