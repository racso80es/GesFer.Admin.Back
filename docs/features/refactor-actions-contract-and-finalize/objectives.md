---
feature_name: refactor-actions-contract-and-finalize
created: "2026-05-01"
process: feature
---

## Objetivo

Alinear el ecosistema SddIA con un **contrato de acciones** que prohíbe la ejecución directa de comandos del sistema operativo o scripts (`.ps1`, `.bat`, etc.) desde las acciones del ciclo. Las acciones solo **orquestan** skills y tools registradas (Cúmulo + contratos). En paralelo, **renombrar** la acción de cierre `finalize` a **`finalize-process`**, reorientar sus disparadores al vocabulario de cierre de proceso/tarea y eliminar cualquier remanente de ejecución directa, delegando el cierre Git en la suite **Git S+** (`git-sync-remote`, `git-create-pr`, `git-save-snapshot`, etc.).

## Alcance

- **Contrato:** `SddIA/actions/actions-contract.md` — nueva sección de invariantes (solo orquestación vía skills/tools autorizadas; prohibición explícita de OS/scripts).
- **Acción:** carpeta `SddIA/actions/finalize/` → `SddIA/actions/finalize-process/` con `action_id: finalize-process` y triggers/ texto orientados a «proceso finalizado» / «tarea finalizada».
- **Referencias:** actualizar `paths.processPath`, normas (`interaction-triggers`, `git-via-skills-or-process`, `pr-acceptance-protocol`, etc.), `SddIA/actions/README.md`, plantillas, skills de definición (`git-operations`), reglas `.cursor/rules` (p. ej. `action-suggestions.mdc`, `features-documentation-pattern.mdc`, `sddia-ssot.mdc`) y cualquier `spec.md` de proceso que liste `finalize` en `related_actions` o en tablas de fases.
- **AGENTS.md / AGENTS.norms.md:** sustituir menciones a la acción `finalize` por `finalize-process` donde describan el ciclo o la fase 8.

## Fuera de alcance (esta iteración)

- Cambiar el **nombre de carpeta** de artefactos en features (`finalize.md` opcional en la tarea puede seguir existiendo como documento de cierre; valorar en `clarify` si se renombra a `finalize-process.md` solo a nivel documental de tarea).
- Reescribir histórico de `docs/evolution/` salvo entradas nuevas que exijan esta feature.

## Ley aplicada

- **Ley COMANDOS** (`AGENTS.md`): comandos solo vía skill/tool/acción/proceso; esta feature refuerza que la **acción** no sea un atajo que invoque shell.
- **SSOT** (`docs/` y `SddIA/`): coherencia entre contrato, procesos y difusión Cursor.

## Proceso y trazabilidad

- **Proceso:** feature v2.0.0 (`SddIA/process/feature/spec.md`).
- **Rama:** `feat/refactor-actions-contract-and-finalize`.
- **Impacto SddIA:** obligatorio `sddia-evolution-register` + snapshot adicional antes de PR; cierre con `git-sync-remote` y `git-create-pr` inyectando resumen de validación.
