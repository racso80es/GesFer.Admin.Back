---
feature_name: prepare-full-env-drop-start-api
created: 2026-04-28
process: feature
scope:
  tool_id: prepare-full-env
  change_type: removal
---

# Objetivos: prepare-full-env-drop-start-api

## Objetivo principal

Retirar de la tool `prepare-full-env` cualquier opción y referencia a **arranque de API** (antiguo `StartApi` / `--start-api`), dejando esa responsabilidad fuera del alcance de la herramienta.

## Alcance

- **Entra**
  - Contrato y documentación de la tool `prepare-full-env` (definición SddIA + cápsula).
  - Implementación Rust (`gesfer-tools`) y compilación/instalación del ejecutable en la cápsula.
- **No entra**
  - Modificar auditorías históricas en `docs/audits/` (son testigos inmutables).

## Criterios de aceptación

- `prepare-full-env` **no expone** `StartApi` ni `--start-api` (spec + docs cápsula + launcher).
- El flujo de fases/feedback **no incluye** `api`.
- La compilación de `gesfer-tools` produce `prepare_full_env.exe` y se copia a `scripts/tools/prepare-full-env/`.

