# Implementación: Kaizen process/actions restructuración

**Feature:** kaizen-process-actions-restructure  
**Plan:** plan.md | **SPEC:** spec.md  
**Ruta (Cúmulo):** paths.featurePath/kaizen-process-actions-restructure

---

## Ítems de implementación

Cada ítem: **Id** | **Acción** | **Ruta** | **Ubicación** | **Propuesta** | **Dependencias**

### Fase 0 — Contrato process

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| 0.1 | Crear | SddIA/process/process-contract.json | (nuevo) | JSON con contract_version, scope (paths.processPath), description, definition_artefacts (spec.md, spec.json en paths.processPath/<process-id>/), process_interface_compliance (ref a Cúmulo), constraints (process_id kebab-case, rutas vía Cúmulo), consumers (paths.actionsPath, agents, norms). Modelo: skills-contract.json. | — |
| 0.2 | Crear | SddIA/process/process-contract.md | (nuevo) | Versión legible del contrato: propósito, artefactos por proceso, cumplimiento process_interface, restricciones. | 0.1 |

### Fase 1 — Migrar procesos

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| 1.1a | Crear | SddIA/process/feature/spec.md | (nuevo) | Copiar contenido íntegro de SddIA/process/feature.md. Ajustar encabezado si indica ruta (paths.processPath/feature/). | 0.2 |
| 1.1b | Crear | SddIA/process/feature/spec.json | (nuevo) | Generar JSON: process_id "feature", name, description, phases (tabla fases 0–8), persist_ref (paths.featurePath), related_actions (paths.actionsPath), contract_ref (process-contract.json). | 0.2 |
| 1.2a | Crear | SddIA/process/refactorization/spec.md | (nuevo) | Copiar contenido de SddIA/process/refactorization.md. | 0.2 |
| 1.2b | Crear | SddIA/process/refactorization/spec.json | (nuevo) | process_id "refactorization", name, description, phases, persist_ref (paths.featurePath/refactorization-*), contract_ref. | 0.2 |
| 1.3a | Crear | SddIA/process/create-tool/spec.md | (nuevo) | Unir contenido de create-tool.md y create-tool.json en spec.md (narrativa) y spec.json (datos). Migrar create-tool.md → spec.md; create-tool.json → spec.json (ajustar campos a process_id, contract_ref). | 0.2 |
| 1.3b | Crear | SddIA/process/create-tool/spec.json | (nuevo) | Basado en create-tool.json actual; añadir contract_ref, path en paths.processPath/create-tool/. | 0.2 |
| 1.4a | Crear | SddIA/process/bug-fix/spec.json | (nuevo) | Migrar bug-fix-specialist.json; renombrar agent_id a process_id "bug-fix"; añadir contract_ref. | 0.2 |
| 1.4b | Crear | SddIA/process/bug-fix/spec.md | (nuevo) | Redactar especificación legible a partir del JSON (objetivo, fases, persist_ref paths.fixPath, skills, constraints). | 0.2 |
| 1.5 | Eliminar | SddIA/process/feature.md | — | Eliminar fichero. | 1.1a,1.1b |
| 1.6 | Eliminar | SddIA/process/refactorization.md | — | Eliminar. | 1.2a,1.2b |
| 1.7 | Eliminar | SddIA/process/create-tool.md, create-tool.json | — | Eliminar ambos. | 1.3a,1.3b |
| 1.8 | Eliminar | SddIA/process/bug-fix-specialist.json | — | Eliminar. | 1.4a,1.4b |

### Fase 2 — Referencias process

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| 2.1 | Modificar | SddIA/process/README.md | Tabla procesos, sección Uso | Sustituir enlaces [feature.md] por [feature/spec.md] o feature/; añadir párrafo que referencie process-contract.json y process-contract.md. | 1.5–1.8 |
| 2.2 | Modificar | SddIA/norms/interaction-triggers.md | #Process, listado, fuentes | paths.processPath/<process-id>/ (feature, bug-fix, refactorization, create-tool); detalle paths.processPath/<id>/spec.md. | 1.5 |
| 2.3 | Modificar | .cursor/rules/process-suggestions.mdc | Listado, fuentes | paths.processPath/feature/, paths.processPath/bug-fix/, etc. | 1.5 |
| 2.4 | Modificar | AGENTS.md, SddIA/process/*/spec, SddIA/skills/iniciar-rama/spec | Cualquier path a feature.md o bug-fix-specialist.json | Sustituir por paths.processPath/feature/, paths.processPath/bug-fix/. | 1.5 |

### Fase 3 — Contrato actions

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| 3.1 | Crear | SddIA/actions/actions-contract.json | (nuevo) | contract_version, scope (paths.actionsPath), description, definition_artefacts (spec.md, spec.json en paths.actionsPath/<action-id>/), constraints (action_id kebab-case, rutas Cúmulo), consumers (paths.processPath, agents, norms). | — |
| 3.2 | Crear | SddIA/actions/actions-contract.md | (nuevo) | Versión legible del contrato. | 3.1 |

### Fase 4 — Migrar acciones

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| 4.1–4.8 | Crear | SddIA/actions/<action>/spec.md + spec.json | Por cada action in [spec, clarify, planning, implementation, execution, validate, finalize, sddia-difusion] | Copiar <action>.md → <action>/spec.md; generar <action>/spec.json (action_id, name, purpose, inputs, outputs, flow_steps, contract_ref). | 3.2 |
| 4.9 | Eliminar | SddIA/actions/spec.md, clarify.md, planning.md, implementation.md, execution.md, validate.md, finalize.md, sddia-difusion.md | — | Eliminar los 8 .md de la raíz de actions. | 4.1–4.8 |

### Fase 5 — Referencias actions

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| 5.1 | Crear | SddIA/actions/README.md | (nuevo) | Tabla: action_id, descripción, path (paths.actionsPath/<id>/). Referencia actions-contract. | 4.9 |
| 5.2 | Modificar | SddIA/norms/interaction-triggers.md | #Action | paths.actionsPath/<action-id>/; detalle paths.actionsPath/<id>/spec.md. | 4.9 |
| 5.3 | Modificar | .cursor/rules/action-suggestions.mdc | Fuentes, listado | paths.actionsPath/spec/, paths.actionsPath/finalize/, etc. | 4.9 |
| 5.4 | Modificar | Referencias en process, finalize, AGENTS | paths.actionsPath/finalize.md etc. | Sustituir por paths.actionsPath/finalize/ o finalize/spec.md. | 4.9 |

---

## Orden sugerido de aplicación

1. Fase 0 (0.1 → 0.2).  
2. Fase 1 (1.1a,b → 1.2a,b → 1.3a,b → 1.4a,b → 1.5–1.8).  
3. Fase 2 (2.1–2.4).  
4. Fase 3 (3.1 → 3.2).  
5. Fase 4 (4.1–4.8 por acción, luego 4.9).  
6. Fase 5 (5.1–5.4).

## Resumen por archivo

- **SddIA/process/:** process-contract.json, process-contract.md (crear); feature/, refactorization/, create-tool/, bug-fix/ (crear con spec.md+spec.json); feature.md, refactorization.md, create-tool.md, create-tool.json, bug-fix-specialist.json (eliminar); README.md (modificar).
- **SddIA/actions/:** actions-contract.json, actions-contract.md (crear); spec/, clarify/, planning/, implementation/, execution/, validate/, finalize/, sddia-difusion/ (crear con spec.md+spec.json); 8× .md raíz (eliminar); README.md (crear).
- **SddIA/norms/interaction-triggers.md:** modificar #Process y #Action.
- **.cursor/rules/:** process-suggestions.mdc, action-suggestions.mdc: modificar.
- **Otros:** AGENTS.md y referencias cruzadas en process/skills: actualizar rutas.
