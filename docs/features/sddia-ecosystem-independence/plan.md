# Plan: Implementación remanente — Rutas vía Cúmulo

**Feature:** sddia-ecosystem-independence (alcance sddia-paths-cumulo)  
**Rama:** feat/sddia-paths-cumulo  
**Persist:** docs/features/sddia-ecosystem-independence/

## Estado

- **Hecho:** Cúmulo (paths + instructions), finalize.md, feature.md, norms (interaction-triggers, paths-via-cumulo.md), AGENTS.md, clarify.json (decisiones resueltas).
- **Hecho (fases 1-8):** Cúmulo, actions, process, skills, agents, tools, norms, templates; guía remanente-rutas.md; implementation.json y validacion.json actualizados.

## Fases del plan

| Fase | Alcance | Entregable |
|------|---------|------------|
| **1** | Documentación para repetir | remanente-rutas.md (guía de sustitución y archivos pendientes). |
| **2** | Actions: validate, execution, clarify, spec, planning, implementation | Sin literales docs/... ni scripts/...; solo paths.* o referencia a skill/Cúmulo. |
| **3** | Process: bug-fix-specialist.json, create-tool (.md, .json), README.md | Idem. |
| **4** | Skills: skills-contract, finalizar-git, invoke-command, iniciar-rama, security-audit, documentation | Idem. |
| **5** | Agents: tekton, qa-judge, auditor/*, performance-engineer | Idem. |
| **6** | Tools: tools-contract, README | Idem. |
| **7** | Templates: spec-template.md | Idem. |
| **8** | Revisión final y validacion.json | validacion.json con checks de "rutas vía Cúmulo" y lista de archivos actualizados. |

## Criterio de aceptación

- Ningún archivo en SddIA (salvo cumulo.json) contiene rutas literales `docs/...` ni `scripts/...` para contextos ya cubiertos por Cúmulo (featurePath, fixPath, evolutionPath, auditsPath, actionsPath, processPath, skillCapsules, toolCapsules, etc.).
- La guía remanente-rutas.md permite a un agente o desarrollador repetir el proceso en futuros cambios.

## Referencias

- alcance-rutas.md — Análisis inicial.
- clarify.md / clarify.json — Decisiones (Todos paths, Opción B, norma sí).
- SddIA/norms/paths-via-cumulo.md — Norma de rutas.
