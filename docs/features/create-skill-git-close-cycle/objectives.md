---
feature_name: create-skill-git-close-cycle
created: "2026-05-01"
process: feature
---

## Objetivo

Introducir la skill **git-close-cycle** para normalizar el cierre local del ciclo tras integración en remoto (actualizar troncal, podar refs, eliminar la rama de trabajo), y enlazar su invocación en el contrato de la acción **finalize-process** cuando el cierre se entiende como **tarea finalizada** en contexto post-fusión.

## Alcance

- Definición en `paths.skillsDefinitionPath/git-close-cycle/` y cápsula en `paths.skillCapsules.git-close-cycle` (Cúmulo).
- Implementación Rust `git_close_cycle` en `paths.skillsRustPath` y registro en `paths.skillsIndexPath`.
- Actualización de `SddIA/actions/finalize-process/spec.md`: paso orquestado final **git-close-cycle** con `target_branch` = rama de trabajo de la tarea.
- Difusión: `interaction-triggers.md`, `skill-suggestions.mdc`, `SddIA/skills/README.md`, proceso **feature** (`related_skills`), `install.ps1` de skills.

## Ley aplicada

- **Ley COMANDOS** / **Git vía skills:** solo cápsulas; invocación agente con envelope capsule-json-io v2 (o wrapper `.tekton_request.json` + `run-capsule-from-tekton-request.ps1`).

## Rama

- `feat/create-skill-git-close-cycle`
