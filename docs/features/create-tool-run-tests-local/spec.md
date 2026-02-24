# Especificación: run-tests-local (create-tool)

**toolId:** `run-tests-local`  
**Tarea:** create-tool-run-tests-local (paths.featurePath/create-tool-run-tests-local/).  
**Definición (SddIA):** paths.toolsDefinitionPath/run-tests-local/  
**Implementación:** paths.toolCapsules.run-tests-local (Cúmulo).

## Objetivo

Herramienta que ejecuta tests (unitarios, integración, E2E) en condiciones de validación local sin que el agente invoque comandos de sistema directamente. Orquesta opcionalmente prepare-full-env e invoke-mysql-seeds, compila la solución y ejecuta dotnet test según el alcance indicado (unit | integration | e2e | all). Produce salida JSON y feedback conforme a tools-contract.

## Entradas

| Parámetro | Tipo | Descripción |
|-----------|------|-------------|
| SkipPrepare | switch | No invocar prepare-full-env (Docker/MySQL ya levantados). |
| SkipSeeds | switch | No invocar invoke-mysql-seeds (BD ya migrada y con seeds). |
| TestScope | string | Alcance: `unit`, `integration`, `e2e`, `all`. Por defecto `all`. |
| OnlyTests | switch | Solo ejecutar tests (no prepare, no seeds, no arranque API); equivalente a SkipPrepare + SkipSeeds cuando TestScope incluye e2e. |
| E2EBaseUrl | string | URL base de la API para E2E (por defecto http://localhost:5010). |
| OutputPath | string | Fichero donde escribir el resultado JSON (contrato). |
| OutputJson | switch | Emitir el resultado JSON por stdout. |

## Salida

Cumple SddIA/tools/tools-contract.json: objeto JSON con toolId, exitCode, success, timestamp, message, feedback[], data (resumen por proyecto/scope: passed, failed, total; opcionalmente results por categoría), duration_ms.

## Fases (feedback)

init → prepare (opcional) → seeds (opcional) → build → tests (unit | integration | e2e según scope) → done (o error).

## Dependencias

- paths.toolCapsules.prepare-full-env, paths.toolCapsules.invoke-mysql-seeds (invocación desde la cápsula).
- Solución y proyectos de test: UnitTests, IntegrationTests, E2ETests; rutas desde config o Cúmulo.
- E2E: variables E2E_BASE_URL, E2E_INTERNAL_SECRET cuando TestScope incluye e2e.

## Implementación

La cápsula en paths.toolCapsules.run-tests-local contendrá manifest.json, run-tests-local-config.json, documentación .md, launcher .bat y script .ps1 que orqueste las invocaciones y dotnet test. Contrato: tools-contract.json.
