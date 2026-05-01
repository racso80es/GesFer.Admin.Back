---
feature_name: refactor-actions-contract-and-finalize
branch: feat/refactor-actions-contract-and-finalize
base_branch: main
global: pass
blocking: false
checks:
  - name: actions-contract-invariantes
    result: pass
    message: Contrato actions incluye invariante de ejecución (sin OS/scripts directos) y action_id finalize-process en listado.
  - name: finalize-process-spec
    result: pass
    message: SddIA/actions/finalize-process/spec.md presente; SddIA/actions/finalize/spec.md eliminado.
  - name: referencias-procesos-normas
    result: pass
    message: Procesos, interaction-triggers, git-via-skills, pr-acceptance-protocol, templates y .cursor/rules alineados a finalize-process.
  - name: difusion-features-contract
    result: pass
    message: docs/features/features-contract.md y norma features-documentation-pattern usan finalize-process / finalize-process.md.
  - name: helper-tekton-capsule
    result: pass
    message: scripts/skills/run-capsule-from-tekton-request.ps1 añadido para envelope v2 desde .tekton_request.json.
git_changes:
  files_added:
    - SddIA/actions/finalize-process/spec.md
    - scripts/skills/run-capsule-from-tekton-request.ps1
  files_modified:
    - SddIA/actions/actions-contract.md
    - SddIA/process/feature/spec.md
    - SddIA/norms/interaction-triggers.md
    - docs/features/features-contract.md
    - .cursor/rules/action-suggestions.mdc
  files_deleted:
    - SddIA/actions/finalize/spec.md
---

# Validación — refactor-actions-contract-and-finalize

## Resumen

Mutación documental y estructural aplicada: contrato de acciones blindado, acción **finalize-process** como cierre canónico, referencias actualizadas en procesos SddIA, normas, plantillas y difusión Cursor. Sin ejecución directa prescrita en las specs de acción para `.ps1` / `.bat` / git CLI.

## Evidencia

- Búsqueda residual de `paths.actionsPath/finalize/` en SddIA activo eliminada (salvo notas históricas puntuales en análisis legacy si aplica).
- **validate** y **finalize-process** referenciadas de forma coherente en `SddIA/actions/validate/spec.md`.
