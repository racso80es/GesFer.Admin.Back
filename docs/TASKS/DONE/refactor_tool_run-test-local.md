---
task_id: refactor_tool_run-test-local
toolId: run-tests-local
tipo: refactor-futuro
prioridad: media
estado: pendiente
fecha: 2026-04-23
---

## Objetivo

Refactorizar la tool `run-tests-local` para cumplir el estándar SddIA:

- **Interfaz obligatoria**: ejecutable Rust `run_tests_local.exe` en la **raíz** de la cápsula `scripts/tools/run-tests-local/`.
- **Contrato**: envelope v2 (`SddIA/norms/capsule-json-io.md`) y salida `CapsuleResponse` con campo **`result`** (no `data`).
- **Sin `.ps1` como interfaz**: cualquier `.ps1` debe eliminarse o quedar solo como artefacto transitorio no invocable por agente (ideal: eliminarlo).

## Contexto actual (a revisar al iniciar)

- `SddIA/tools/run-tests-local/spec.md` ya indica migración pendiente a Rust.
- Existe `scripts/tools/run-tests-local/Run-Tests-Local.ps1` y wrapper `.bat`.
- `scripts/tools/index.json` define política: **IA/MCP → `.exe`**.

## Acciones necesarias (pasos)

### 1) Implementar binario Rust

- Crear `scripts/tools-rs/src/bin/run_tests_local.rs`.
- Añadirlo al crate `gesfer-tools` (si no existe ya en `scripts/tools-rs/Cargo.toml`).
- Implementar el flujo mínimo:
  - Opcionalmente invocar `prepare-full-env` según flags (o realizar “docker up + wait mysql” de forma equivalente).
  - Opcionalmente invocar `invoke-mysql-seeds`.
  - Ejecutar `dotnet test` según parámetros (scope unit/integration/e2e si aplica).
  - Emitir `feedback[]` por fases (ej. `init`, `prepare`, `seeds`, `build`, `tests`, `done/error`).
  - Salida `CapsuleResponse::tool("run-tests-local", ...)` con `result` estructurado.

### 2) Alinear la cápsula (`scripts/tools/run-tests-local/`)

- Asegurar que `scripts/tools-rs/install.ps1` copia `run_tests_local.exe` a:
  - `scripts/tools/run-tests-local/run_tests_local.exe`
- Ajustar `scripts/tools/run-tests-local/Run-Tests-Local.bat` para **fallar** si el exe no existe (sin fallback a `.ps1`).
- Actualizar `scripts/tools/run-tests-local/run-tests-local.md` para eliminar referencias a ejecución por `.ps1`.

### 3) Alinear la spec SddIA y documentación

- `SddIA/tools/run-tests-local/spec.md`
  - Confirmar “Ubicación objetivo” en **raíz de cápsula**.
  - Asegurar “Fuente Rust” apunta a `scripts/tools-rs/src/bin/run_tests_local.rs`.
- Revisar `SddIA/tools/tools-contract.md` y `SddIA/norms/capsule-json-io.md` para coherencia.

## Criterios de aceptación

- Existe `scripts/tools/run-tests-local/run_tests_local.exe` tras compilar (`scripts/tools-rs/install.ps1`).
- La tool funciona vía:
  - **CLI**: `run_tests_local.exe --output-json ...`
  - **Envelope** (MCP/IA): `GESFER_CAPSULE_REQUEST` o stdin JSON según norma.
- No hay dependencias funcionales en `.ps1` (ni wrappers que lo usen como fallback).
- La salida cumple v2: `meta`, `success`, `exitCode`, `message`, `feedback`, `result`, `duration_ms`.

