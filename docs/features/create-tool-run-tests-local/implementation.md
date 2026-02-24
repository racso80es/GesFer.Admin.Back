# Implementación: run-tests-local (create-tool)

**Tarea:** create-tool-run-tests-local  
**Rutas Cúmulo:** paths.toolCapsules.run-tests-local, paths.toolsDefinitionPath/run-tests-local.

## Entregables

| Artefacto | Ubicación |
|-----------|-----------|
| Definición | SddIA/tools/run-tests-local/ (spec.md, spec.json) |
| Cápsula | scripts/tools/run-tests-local/ |
| Índice | scripts/tools/index.json (entrada run-tests-local) |
| Cúmulo | SddIA/agents/cumulo.paths.json (toolCapsules.run-tests-local) |

## Cápsula (paths.toolCapsules.run-tests-local)

- **manifest.json** — toolId, version, contract_ref, components (launcher_bat, launcher_ps1, config, doc, bin).
- **run-tests-local-config.json** — solutionPath, projects (unit, integration, e2e), e2eFilter, e2eEnv, toolsRef.
- **run-tests-local.md** — Documentación de uso (es-ES).
- **Run-Tests-Local.bat** — Launcher: bin/run_tests_local.exe si existe; si no, pwsh/powershell -File Run-Tests-Local.ps1.
- **Run-Tests-Local.ps1** — Orquestación: fases init → prepare (opcional) → seeds (opcional) → build → tests; salida JSON (toolId, exitCode, success, timestamp, message, feedback[], data, duration_ms); parámetros SkipPrepare, SkipSeeds, TestScope (unit|integration|e2e|all), OnlyTests, E2EBaseUrl, OutputPath, OutputJson.
- **bin/.gitkeep** — Reservado para binario Rust (fase posterior).

## Invocación de otras herramientas

- prepare-full-env: scripts/tools/prepare-full-env/Prepare-FullEnv.bat (cuando no SkipPrepare).
- invoke-mysql-seeds: scripts/tools/invoke-mysql-seeds/Invoke-MySqlSeeds.bat (cuando no SkipSeeds).

## Validación

Ejecutar la herramienta con el caso de uso (por ejemplo -OnlyTests -TestScope e2e o -TestScope unit) y comprobar que el JSON de salida cumple tools-contract y que exitCode refleja el resultado de los tests. El agente no debe invocar dotnet test directamente; solo la tool.
