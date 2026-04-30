---
feature_name: create-process-create-skill
created: "2026-04-30"
process: feature
---

## Objetivo

Crear un **nuevo proceso** `create-skill` en `paths.processPath` con todo lo necesario para estandarizar la creación de **skills** (definición + cápsula ejecutable), alineado con el patrón de `SddIA/process/create-tool/spec.md`.

## Alcance

- **Proceso nuevo**: `SddIA/process/create-skill/` con `spec.md` (frontmatter YAML) y `spec.json` (machine-readable).
- **Difusión / listados**:
  - `SddIA/process/README.md` (índice de procesos).
  - `.cursor/rules/process-suggestions.mdc` (difusión para `#Process`).
  - `SddIA/norms/interaction-triggers.md` (listado canónico).

## Restricciones (SSOT)

- **Rutas**: solo vía Cúmulo (`SddIA/agents/cumulo.paths.json`).
- **Implementación de skills**: entrega **solo** como ejecutable Rust `.exe` (sin `.ps1/.bat` como implementación principal).
- **Ejecución**: envelope JSON stdin/stdout (`SddIA/norms/capsule-json-io.md`).

