# Análisis: Causa raíz y evaluación (Racso-Tormentosa v1.2)

**Proceso:** refactorization  
**Fases:** Análisis + Evaluación de situación.

## Causa raíz de la desincronización

- **Hecho:** Cambios en la lógica de una Skill (p. ej. invoke-command: nuevo --command-file, rutas) se documentaron en **spec.md** y en la cápsula (invoke-command.md), pero la **definición estructural (spec.json)** no se actualizó en el mismo acto.
- **Causa:** Las **Acciones** actuales del proceso (spec, implementation, execution) no exigen de forma explícita y comprobable que, al modificar un .md de definición (skill o process), se actualice el .json correspondiente en la misma transacción. No hay checklist ni validación automática post-edición que obligue a la propagación MD → JSON (ni JSON → MD cuando el contrato sea JSON-first).
- **Efecto:** El ecosistema queda con JSON desfasado hasta que una tarea posterior (p. ej. rust-skills-tools-protocol) corrige la desincronía de forma puntual.

## Evaluación de situación: ¿Nueva Acción de validación?

- **Determinación:** El proceso actual de SddIA **sí se beneficia** de una **acción de validación post-edición** (o de un criterio explícito dentro de la acción **validate** o **sddia-difusion**) que compruebe la coherencia MD ↔ JSON para entidades de dominio (skills, process, acciones).
- **Propuesta:** No crear una acción nueva independiente, sino:
  1. **Refuerzo en la acción validate:** Incluir en sus criterios (paths.actionsPath/validate/spec.md y spec.json) la comprobación de que, para cada skill/process tocado en la tarea, existan y sean coherentes spec.md y spec.json (campos clave: parameters, rules, routing cuando aplique).
  2. **Refuerzo en sddia-difusion:** Incluir en sus criterios de aceptación que las definiciones en paths.skillsDefinitionPath y paths.processPath tengan paridad MD/JSON en los listados y contratos que se difunden.
- **Marcado:** Inserción de // TODO: [REF-SddIA] en los touchpoints que implementen esta gobernanza (ver implementation.md).
