---
feature_name: refactor-actions-contract-and-finalize
created: "2026-05-01"
phases:
  - id: P0
    name: Contrato y acción
    tasks:
      - Editar actions-contract.md (invariantes + lista action_id con finalize-process).
      - Crear finalize-process/spec.md; eliminar o vaciar finalize/ tras copiar contenido ajustado.
  - id: P1
    name: Procesos y normas
    tasks:
      - SddIA/process/**/spec.md y README.md — related_actions, tablas, fase 8.
      - SddIA/norms/interaction-triggers.md, git-via-skills-or-process.md, pr-acceptance-protocol.md.
      - SddIA/templates/**, SddIA/skills/git-operations/spec.md, actions/README.md, sddia-difusion si aplica.
  - id: P2
    name: Difusión Cursor y AGENTS
    tasks:
      - .cursor/rules/action-suggestions.mdc, features-documentation-pattern.mdc, sddia-ssot.mdc, process-suggestions.mdc si lista acciones.
      - AGENTS.md, AGENTS.norms.md — ciclo y fase 8.
  - id: P3
    name: Validación y cierre
    tasks:
      - Búsqueda residual de finalize en rutas canónicas; generar validacion.md.
      - Si diff toca SddIA/ fuera de evolution: sddia_evolution_register + git-save-snapshot.
      - git-sync-remote, git-create-pr con objectives + validacion en cuerpo del PR.
tasks:
  - id: T1
    description: Snapshot atómico tras contrato + nueva acción
    phase: P0
  - id: T2
    description: Snapshot tras procesos/normas
    phase: P1
  - id: T3
    description: Snapshot tras .cursor y AGENTS
    phase: P2
  - id: T4
    description: Evolution + snapshot + sync + PR
    phase: P3
---

# Plan de implementación

## Orden sugerido

1. **P0** — Cambios en `SddIA/actions/` primero (contrato + carpeta de acción); minimiza referencias rotas intermedias.
2. **P1** — Procesos y normas dependen del nuevo `action_id`.
3. **P2** — Difusión para que Cursor y AGENTS alineen disparadores.
4. **P3** — Validación, evolution, PR.

## Criterio de hecho (DoD)

- No queda `action_id: finalize` en spec canónica bajo `SddIA/actions/`.
- `actions-contract.md` incluye párrafo normativo: acciones = solo orquestación de skills/tools registradas; prohibido invocar OS/scripts directamente desde la definición o el flujo documentado de la acción.
- Proceso feature v2.0.0 referencia `finalize-process` en `related_actions` y en la descripción de fase 8.

## Dependencias

- Skills Git S+ ya desplegadas en cápsulas (compilación local ya realizada en esta sesión para recon/branch).
