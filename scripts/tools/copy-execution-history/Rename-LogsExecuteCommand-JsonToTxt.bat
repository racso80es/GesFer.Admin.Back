@echo off
setlocal EnableExtensions

set "DIR=C:\Proyectos\LogsExecuteCommand"
if not exist "%DIR%" (
  echo ERROR: No existe "%DIR%".
  exit /b 1
)

pushd "%DIR%" || exit /b 1

REM Renombra dinamicamente todos los .json a .txt (mismo nombre base)
for %%F in (*.json) do (
  ren "%%F" "%%~nF.txt"
)

popd
endlocal
exit /b 0

