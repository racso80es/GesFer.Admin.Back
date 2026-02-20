# Plan: Kaizen process/actions restructuración

**Feature:** kaizen-process-actions-restructure  
**Ruta (Cúmulo):** paths.featurePath/kaizen-process-actions-restructure  
**Referencias:** spec.md, spec.json, clarify.md

---

## Orden de ejecución (respetar dependencias)

### Fase 0 — Contrato process

| Id | Tarea | Salida | Dependencias |
|----|-------|--------|--------------|
| 0.1 | Redactar process-contract.json en SddIA/process/ (definition_artefacts, process_interface_compliance, constraints, consumers). | SddIA/process/process-contract.json | spec, clarify |
| 0.2 | Redactar process-contract.md (versión legible del contrato). | SddIA/process/process-contract.md | 0.1 |

### Fase 1 — Migrar procesos a carpetas

| Id | Tarea | Salida | Dependencias |
|----|-------|--------|--------------|
| 1.1 | Crear SddIA/process/feature/; migrar feature.md → feature/spec.md; generar feature/spec.json. | feature/spec.md, feature/spec.json | 0.2 |
| 1.2 | Crear SddIA/process/refactorization/; migrar refactorization.md → refactorization/spec.md; generar refactorization/spec.json. | refactorization/spec.md, refactorization/spec.json | 0.2 |
| 1.3 | Crear SddIA/process/create-tool/; unir create-tool.md + create-tool.json → create-tool/spec.md y create-tool/spec.json. | create-tool/spec.md, create-tool/spec.json | 0.2 |
| 1.4 | Crear SddIA/process/bug-fix/; derivar bug-fix/spec.json desde bug-fix-specialist.json; redactar bug-fix/spec.md (especificación legible). | bug-fix/spec.md, bug-fix/spec.json | 0.2 |
| 1.5 | Eliminar de la raíz de SddIA/process/: feature.md, refactorization.md, create-tool.md, create-tool.json, bug-fix-specialist.json. | — | 1.1–1.4 |

### Fase 2 — README y referencias process

| Id | Tarea | Salida | Dependencias |
|----|-------|--------|--------------|
| 2.1 | Actualizar SddIA/process/README.md: listar procesos como paths.processPath/<process-id>/, referenciar process-contract. | README.md | 1.5 |
| 2.2 | Actualizar SddIA/norms/interaction-triggers.md (#Process): rutas paths.processPath/<process-id>/ o <process-id>/spec. | interaction-triggers.md | 1.5 |
| 2.3 | Actualizar .cursor/rules/process-suggestions.mdc: referencias a carpetas (paths.processPath/feature/, etc.). | process-suggestions.mdc | 1.5 |
| 2.4 | Buscar y actualizar resto de referencias a paths.processPath/feature.md, bug-fix-specialist.json, etc. (AGENTS.md, skills, create-tool). | Varios | 1.5 |

### Fase 3 — Contrato actions

| Id | Tarea | Salida | Dependencias |
|----|-------|--------|--------------|
| 3.1 | Redactar actions-contract.json en SddIA/actions/ (definition_artefacts, action_id, inputs/outputs, constraints, consumers). | SddIA/actions/actions-contract.json | spec, clarify |
| 3.2 | Redactar actions-contract.md (versión legible). | SddIA/actions/actions-contract.md | 3.1 |

### Fase 4 — Migrar acciones a carpetas

| Id | Tarea | Salida | Dependencias |
|----|-------|--------|--------------|
| 4.1 | Por cada acción (spec, clarify, planning, implementation, execution, validate, finalize, sddia-difusion): crear carpeta <action-id>/; mover <action>.md → <action>/spec.md; generar <action>/spec.json. | 8 carpetas con spec.md + spec.json cada una | 3.2 |
| 4.2 | Eliminar de la raíz de SddIA/actions/: spec.md, clarify.md, planning.md, implementation.md, execution.md, validate.md, finalize.md, sddia-difusion.md. | — | 4.1 |

### Fase 5 — README actions y referencias

| Id | Tarea | Salida | Dependencias |
|----|-------|--------|--------------|
| 5.1 | Crear o actualizar SddIA/actions/README.md: tabla de acciones con paths.actionsPath/<action-id>/. | SddIA/actions/README.md | 4.2 |
| 5.2 | Actualizar SddIA/norms/interaction-triggers.md (#Action): rutas paths.actionsPath/<action-id>/. | interaction-triggers.md | 4.2 |
| 5.3 | Actualizar .cursor/rules/action-suggestions.mdc: referencias a carpetas. | action-suggestions.mdc | 4.2 |
| 5.4 | Buscar y actualizar referencias a paths.actionsPath/<action>.md en process, AGENTS, finalize, etc. | Varios | 4.2 |

### Fase 6 — Validación

| Id | Tarea | Salida | Dependencias |
|----|-------|--------|--------------|
| 6.1 | Verificar listados #Process y #Action (interaction-triggers y .cursor); comprobar que no queden rutas a .md sueltos. | validacion.json (opcional) | 2.4, 5.4 |

---

## Resumen por fases

- **Fase 0:** Contratos process (2 artefactos).
- **Fase 1:** 4 carpetas process, 8 artefactos (spec.md + spec.json) + eliminación 5 ficheros raíz.
- **Fase 2:** README process + interaction-triggers + .cursor + referencias cruzadas.
- **Fase 3:** Contratos actions (2 artefactos).
- **Fase 4:** 8 carpetas actions, 16 artefactos + eliminación 8 .md raíz.
- **Fase 5:** README actions + interaction-triggers + .cursor + referencias.
- **Fase 6:** Validación final.
