@echo off
setlocal EnableExtensions EnableDelayedExpansion

REM Copia una lista fija de execution_history.json a:
REM   C:\Proyectos\LogsExecuteCommand
REM renombrando a execution_history_###.json en orden secuencial.

set "DEST_DIR=C:\Proyectos\LogsExecuteCommand"
if not exist "%DEST_DIR%" (
  mkdir "%DEST_DIR%" >nul 2>&1
)

echo.
echo ==============================================
echo  Copy execution_history.json -^> LogsExecuteCommand
echo ==============================================
echo Destino: %DEST_DIR%
echo.

copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\prepare-full-env-drop-start-api\execution_history.json" "%DEST_DIR%\execution_history_001.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-8891640448748590830-c164f9de\execution_history.json" "%DEST_DIR%\execution_history_002.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-13882653442105097217-2a197b92\execution_history.json" "%DEST_DIR%\execution_history_003.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-3628181105714097681-9421699e\execution_history.json" "%DEST_DIR%\execution_history_004.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-16808167432995166160-bfa9dccb\execution_history.json" "%DEST_DIR%\execution_history_005.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\fix-hardcoded-jwt-secret-14196445710699879921\execution_history.json" "%DEST_DIR%\execution_history_006.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\fix-analyze-sddia-evolution-log-bug\execution_history.json" "%DEST_DIR%\execution_history_007.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Front\docs\diagnostics\main\execution_history.json" "%DEST_DIR%\execution_history_008.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Front\docs\diagnostics\fix\code-health-logger-15121670877633628854\execution_history.json" "%DEST_DIR%\execution_history_009.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\main\execution_history.json" "%DEST_DIR%\execution_history_010.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Front\scripts\skills-rs\docs\diagnostics\feat\test-cn-utility-1747190362919799202\execution_history.json" "%DEST_DIR%\execution_history_011.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Front\scripts\skills-rs\docs\diagnostics\main\execution_history.json" "%DEST_DIR%\execution_history_012.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-16027421683555186529-5b3382b1\execution_history.json" "%DEST_DIR%\execution_history_013.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\fix-cors-vulnerability-16027421683555186529\execution_history.json" "%DEST_DIR%\execution_history_014.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-12192513507434870129-9ef22780\execution_history.json" "%DEST_DIR%\execution_history_015.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-4422035750677707130-66526454\execution_history.json" "%DEST_DIR%\execution_history_016.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\feat-kaizen-remove-redundant-deletedat-user-2590960773944543816\execution_history.json" "%DEST_DIR%\execution_history_017.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\feat-audit-2026-04-20\execution_history.json" "%DEST_DIR%\execution_history_018.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\main\execution_history.json" "%DEST_DIR%\execution_history_019.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-7135794475779878235-04d0b836\execution_history.json" "%DEST_DIR%\execution_history_020.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-9123299970416965568-64d270c3\execution_history.json" "%DEST_DIR%\execution_history_021.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\feat-kaizen-audit-2026-04-18\execution_history.json" "%DEST_DIR%\execution_history_022.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\fix-infra-guardian-audit\execution_history.json" "%DEST_DIR%\execution_history_023.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-10448854381099017399-1918b7d0\execution_history.json" "%DEST_DIR%\execution_history_024.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-14999861344116021577-34e0ce36\execution_history.json" "%DEST_DIR%\execution_history_025.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-15341839860020147526-3e89b612\execution_history.json" "%DEST_DIR%\execution_history_026.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-2467478743147894237-eb3eaece\execution_history.json" "%DEST_DIR%\execution_history_027.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\performance-seeders-6347809113493479054\execution_history.json" "%DEST_DIR%\execution_history_028.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Front\docs\diagnostics\feat\task-actualizar-diseny-6820873171999042361\execution_history.json" "%DEST_DIR%\execution_history_029.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Front\docs\diagnostics\main\execution_history.json" "%DEST_DIR%\execution_history_030.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-999254972349308395-b2a84911\execution_history.json" "%DEST_DIR%\execution_history_031.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-6417098275882778343-5a924168\execution_history.json" "%DEST_DIR%\execution_history_032.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-8056690554976357309-714f2766\execution_history.json" "%DEST_DIR%\execution_history_033.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-1665558639986914355-927c7866\execution_history.json" "%DEST_DIR%\execution_history_034.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-6359688638528264566-85bd2e93\execution_history.json" "%DEST_DIR%\execution_history_035.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-13954389342489676636-bdebf064\execution_history.json" "%DEST_DIR%\execution_history_036.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-15117954001741022126-14cfb7b2\execution_history.json" "%DEST_DIR%\execution_history_037.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-11340766248755957736-9e59b129\execution_history.json" "%DEST_DIR%\execution_history_038.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\jules-12875293586801950215-ee68946e\execution_history.json" "%DEST_DIR%\execution_history_039.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\docs\diagnostics\feat-kaizen-2026-04-13-remove-redundant-deletedat-supplier\execution_history.json" "%DEST_DIR%\execution_history_040.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Front\scripts\skills-rs\docs\diagnostics\feat\kaizen-npm-audit-17913973750748420498\execution_history.json" "%DEST_DIR%\execution_history_041.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Front\docs\diagnostics\feat\kaizen-npm-audit-17913973750748420498\execution_history.json" "%DEST_DIR%\execution_history_042.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Front\docs\diagnostics\feat\sddia-evolution-sync-norma\execution_history.json" "%DEST_DIR%\execution_history_043.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Front\docs\diagnostics\feature\home-publica-1904859605311719791\execution_history.json" "%DEST_DIR%\execution_history_044.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Front\docs\diagnostics\feat\kaizen-enforce-import-rule-7169013432768436544\execution_history.json" "%DEST_DIR%\execution_history_045.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Front\docs\diagnostics\fix\kaizen-error-handling-9214232736602125064\execution_history.json" "%DEST_DIR%\execution_history_046.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Front\docs\diagnostics\feat\correccion-auditorias-2026-03-23-9043078618454227804\execution_history.json" "%DEST_DIR%\execution_history_047.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Front\docs\diagnostics\feat\sddia-evolution-sync-norma\execution_history.json" "%DEST_DIR%\execution_history_048.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\kaizen-2026-03-27-13652127576201859270\execution_history.json" "%DEST_DIR%\execution_history_049.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\refactor-automatic-task-kaizen-queue\execution_history.json" "%DEST_DIR%\execution_history_050.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\automatic-task-kaizen-queue-doc-cierre\execution_history.json" "%DEST_DIR%\execution_history_051.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat-sddia-evolution-sync-norma\execution_history.json" "%DEST_DIR%\execution_history_052.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\unificacion-geolocation-lectura-product\execution_history.json" "%DEST_DIR%\execution_history_053.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\fix\start-api-client-timeout\execution_history.json" "%DEST_DIR%\execution_history_054.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\eliminar-dependencias-serilog\execution_history.json" "%DEST_DIR%\execution_history_055.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\admin-login-audit-log\execution_history.json" "%DEST_DIR%\execution_history_056.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Front\scripts\skills\invoke-command\docs\diagnostics\feat\security-paths-cumulo\execution_history.json" "%DEST_DIR%\execution_history_057.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Front\scripts\docs\diagnostics\feat\refactorization-eliminar-ps1-skills-tools\execution_history.json" "%DEST_DIR%\execution_history_058.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Front\scripts\docs\diagnostics\main\execution_history.json" "%DEST_DIR%\execution_history_059.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Front\docs\diagnostics\fix\login-frontend-dashboard\execution_history.json" "%DEST_DIR%\execution_history_060.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Front\docs\diagnostics\feat\audit-execution-history-binarios\execution_history.json" "%DEST_DIR%\execution_history_061.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Front\docs\diagnostics\feat\audit-funcional-frontend-tool\execution_history.json" "%DEST_DIR%\execution_history_062.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Front\docs\diagnostics\feat\fix-execution-history-en-pr\execution_history.json" "%DEST_DIR%\execution_history_063.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Front\docs\diagnostics\feat\audit-inicial-admin-front\execution_history.json" "%DEST_DIR%\execution_history_064.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\refactorization-arquitectura-frontmatter\execution_history.json" "%DEST_DIR%\execution_history_065.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\scripts\docs\diagnostics\feat\refactorization-eliminar-ps1-skills-tools\execution_history.json" "%DEST_DIR%\execution_history_066.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Front\scripts\docs\diagnostics\feat\refactorization-eliminar-ps1-skills-tools\execution_history.json" "%DEST_DIR%\execution_history_067.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\scripts\docs\diagnostics\feat\refactorization-eliminar-ps1-skills-tools\execution_history.json" "%DEST_DIR%\execution_history_068.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\create-skill-invoke-commit\execution_history.json" "%DEST_DIR%\execution_history_069.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\refactorization-eliminar-ps1-skills-tools\execution_history.json" "%DEST_DIR%\execution_history_070.json" >nul
copy /Y "C:\Proyectos\Kalma2\scripts\skills\invoke-command\docs\diagnostics\feat\security-paths-cumulo\execution_history.json" "%DEST_DIR%\execution_history_071.json" >nul
copy /Y "C:\Proyectos\Kalma2\scripts\docs\diagnostics\main\execution_history.json" "%DEST_DIR%\execution_history_072.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\create-tool-start-api\execution_history.json" "%DEST_DIR%\execution_history_073.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\refactor-skill-finalizar-proceso\execution_history.json" "%DEST_DIR%\execution_history_074.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\scripts\docs\diagnostics\main\execution_history.json" "%DEST_DIR%\execution_history_075.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Front\scripts\docs\diagnostics\main\execution_history.json" "%DEST_DIR%\execution_history_076.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\scripts\docs\diagnostics\main\execution_history.json" "%DEST_DIR%\execution_history_077.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\auditor-unificado-kaizen\execution_history.json" "%DEST_DIR%\execution_history_078.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\estabilidad-bd-inicializacion\execution_history.json" "%DEST_DIR%\execution_history_079.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\rust-skills-tools-protocol\execution_history.json" "%DEST_DIR%\execution_history_080.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\correccion-auditorias-20260223\execution_history.json" "%DEST_DIR%\execution_history_081.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\rust-tools-skills-standard\execution_history.json" "%DEST_DIR%\execution_history_082.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\sddia-templates\execution_history.json" "%DEST_DIR%\execution_history_083.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\security-paths-cumulo\execution_history.json" "%DEST_DIR%\execution_history_084.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\scripts\skills\invoke-command\docs\diagnostics\feat\security-paths-cumulo\execution_history.json" "%DEST_DIR%\execution_history_085.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Front\scripts\skills\invoke-command\docs\diagnostics\feat\security-paths-cumulo\execution_history.json" "%DEST_DIR%\execution_history_086.json" >nul
copy /Y "C:\Proyectos\GesFer.Product.Back\scripts\skills\invoke-command\docs\diagnostics\feat\security-paths-cumulo\execution_history.json" "%DEST_DIR%\execution_history_087.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\norma-git-via-skills-validate-nomenclatura\execution_history.json" "%DEST_DIR%\execution_history_088.json" >nul
copy /Y "C:\Proyectos\GesFer.Admin.Back\docs\diagnostics\feat\tools-env-and-seeds\execution_history.json" "%DEST_DIR%\execution_history_089.json" >nul
copy /Y "C:\Proyectos\GesFer\docs\diagnostics\feat-spec-article-family-13284649229191957205\execution_history.json" "%DEST_DIR%\execution_history_090.json" >nul

echo Hecho.
endlocal
exit /b 0

