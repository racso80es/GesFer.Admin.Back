---
feature_name: create-process-create-skill
created: "2026-04-30"
purpose: "Cerrar naming, alcance y restricciones del proceso create-skill antes de actualizar índices/normas."
decisions:
  - id: implementation_constraint
    decision: "strict_rust_only"
    detail: "Entrega ejecutable: `.exe` Rust. No `.ps1/.bat` como implementación principal."
  - id: feature_slug
    decision: "create-process-create-skill"
    detail: "Se confirma este slug para carpeta `docs/features/<feature_name>/` y rama `feat/<feature_name>`."
  - id: deliverables_scope
    decision: "full"
    detail: "El proceso create-skill debe asegurar: definición SddIA + cápsula ejecutable + actualización de skillsIndex + actualización de Cúmulo (skillCapsules) cuando aplique."
  - id: process_index_updates
    decision: "yes_update_norms"
    detail: "Actualizar también `SddIA/norms/interaction-triggers.md` además de README y difusión Cursor."
  - id: process_shape
    decision: "mirror_create_tool"
    detail: "Estructura y fases como `create-tool` (adaptado a skills)."
---

## Dudas y gaps identificados

### Q1: Naming de la feature / rama

**Contexto:** se requiere un `feature_name` único y kebab-case para `docs/features/<feature_name>/` y rama `feat/<feature_name>`.

**Decisión:** `create-process-create-skill`.

### Q2: Alcance mínimo del proceso `create-skill`

**Contexto:** al ejecutar el proceso, puede requerirse (a) definición SddIA, (b) cápsula ejecutable, (c) índice, (d) actualización de Cúmulo.

**Decisión:** alcance **completo** (definición SddIA + cápsula ejecutable + skillsIndex + actualización de Cúmulo cuando aplique).

## Decisiones confirmadas

- **Implementación**: estricta `.exe` Rust.
- **Forma del proceso**: espejo de `create-tool`.
- **Listado canónico**: se actualizará `SddIA/norms/interaction-triggers.md`.
 - **Naming**: `create-process-create-skill`.
 - **Entregables**: alcance completo (definición+cápsula+índice+Cúmulo).

