# Clarificaciones: Refactorización Arquitectura Frontmatter

**Spec:** spec.md (SPEC-REF-2026-002)  
**Ruta:** paths.featurePath/refactorization-arquitectura-frontmatter  
**Estado:** Resuelto — decisiones del usuario aplicadas

---

## Resoluciones (consultar usuario)

| ID | Tema | Decisión |
|----|------|----------|
| C1 | Contratos duales | **En Fase 2.** Toda entidad SddIA ha de respetar la nueva estructura. Incluir contratos en Fase 2. |
| C2 | Orden Fase 1 vs Fase 2 | **A.** Fase 1 completa antes de Fase 2 (secuencial estricto). |
| C3 | Excepción karma2-token | **B.** Mantener excepción; tokens pueden ser JSON-only por contrato. |
| C4 | Validación frontmatter | **B.** Esquema YAML nativo (herramienta en el ecosistema). |
| C5 | Compatibilidad durante migración | **B.** Migración en único PR; no hay coexistencia. |
| C6 | Ubicación herramienta conversión | **B.** Script puntual en carpeta de la tarea o scripts/; uso puntual. |

---

## Requisito adicional (C7)

**Actualización de normas SddIA sobre el nuevo formato — ha de ser lo primero.**

La actualización de las normas SddIA (constitution.json, entidades-dominio-ecosistema-sddia.md, contratos, interaction-triggers.md, paths-via-cumulo.md, etc.) para definir y mandatar el nuevo formato frontmatter es el **primer paso** de la Fase 2, antes de cualquier migración de entidades.

Orden Fase 2:
1. **Actualizar normas SddIA** (constitution, entidades-dominio, contratos, interaction-triggers, paths-via-cumulo, patterns-in-planning-implementation-execution, agents-principles-contract, commands-via-skills-or-tools, git-via-skills-or-process, norms/README) sobre el nuevo formato.
2. Script Rust de conversión (puntual).
3. Migración en único PR (84 entidades + contratos).
4. Actualizar validate, reglas Cursor, AGENTS.md.

---

## Requisito adicional (C8)

**La documentación generada en un proceso ha de respetar el nuevo patrón.**

Los artefactos de documentación producidos por procesos (objectives, spec, clarify, plan, implementation, validacion, execution) — es decir, todo lo que vive en paths.featurePath/\<task\>/ y paths.fixPath/\<fix\>/ — deben seguir el formato frontmatter (un único .md con frontmatter YAML por artefacto, en lugar de pares .md + .json).

Esto implica:
- **objectives.md** con frontmatter (en lugar de objectives.md + objectives.json)
- **spec.md** con frontmatter (en lugar de spec.md + spec.json)
- **clarify.md** con frontmatter (en lugar de clarify.md + clarify.json)
- **plan.md** con frontmatter (en lugar de plan.md + plan.json)
- **implementation.md** con frontmatter (en lugar de implementation.md + implementation.json)
- **validacion.md** con frontmatter (en lugar de validacion.json solo, o validacion.md + validacion.json)

La interfaz de proceso (process_interface en Cúmulo) y las acciones (spec, clarify, planning, implementation) deben actualizarse para generar artefactos en formato frontmatter. Las tareas nuevas que se inicien tras la migración producirán documentación en el nuevo formato.

---

*Clarificaciones cerradas. Listo para fase de planificación.*
