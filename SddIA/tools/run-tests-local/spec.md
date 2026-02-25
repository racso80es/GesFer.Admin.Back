# Especificación: run-tests-local

**toolId:** `run-tests-local`  
**Definición (SddIA):** Este directorio (paths.toolsDefinitionPath/run-tests-local/).  
**Implementación (scripts):** Ruta canónica en Cúmulo → **implementation_path_ref:** `paths.toolCapsules.run-tests-local` (consultar `SddIA/agents/cumulo.json`).

## Objetivo

Ejecutar tests (unitarios, integración, E2E) en condiciones de validación local sin invocar comandos de sistema directamente desde el agente. Orquesta opcionalmente prepare-full-env e invoke-mysql-seeds, compila la solución y ejecuta dotnet test según el alcance (unit | integration | e2e | all). Salida JSON y feedback conforme a tools-contract.

## Entradas

| Parámetro | Tipo | Descripción |
|-----------|------|-------------|
| SkipPrepare | switch | No invocar prepare-full-env. |
| SkipSeeds | switch | No invocar invoke-mysql-seeds. |
| TestScope | string | unit, integration, e2e, all (por defecto all). |
| OnlyTests | switch | Solo ejecutar tests (no prepare, no seeds). |
| E2EBaseUrl | string | URL base API para E2E (por defecto http://localhost:5010). |
| OutputPath | string | Fichero donde escribir el resultado JSON. |
| OutputJson | switch | Emitir resultado JSON por stdout. |

## Salida

Cumple `SddIA/tools/tools-contract.json`: toolId, exitCode, success, timestamp, message, feedback[], data (tests_summary, duration_ms).

## Fases (feedback)

init → prepare (opcional) → seeds (opcional) → build → tests → done (o error).

## Implementación

La implementación (manifest, config, .bat, .ps1) reside en la carpeta indicada por Cúmulo en **implementation_path_ref**.
