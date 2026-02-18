# Objetivo: SddIA procesos, validate y documentación

## Objetivo

Definir procesos de tarea (feature, bug-fix) en SddIA, refinar la acción validate con validación de cambios git obligatoria, establecer Cúmulo como fuente de rutas e interfaz .md/.json para procesos, y cerrar la tarea con Evolution Logs y documentación de referencia.

## Alcance

- Documentación del proyecto aislado: Objetivos.md, README.md.
- SddIA/process: feature.md, bug-fix-specialist.json, README; interfaz de procesos en Cúmulo.
- AGENTS.md: Inicio de tarea (procesos), Cúmulo como fuente de paths, interfaz procesos, rol BUG-FIX.
- Eliminación de referencias a knowledge-architect; uso de SddIA/agents/cumulo.json.
- Acciones execution, validate, finalize en SddIA/actions; validate siempre con git_changes.
- Validación ejecutada: docs/audits/validacion-main-20260217.json (modo pr_analysis; build fail por dependencias Shared).

## Ley aplicada

- **Ley GIT:** Trabajo en rama feat/sddia-process-validate-docs; no commit en master. Cierre vía PR.
- **SSOT:** Documentación de la tarea en docs/features/sddia-process-validate-docs/.

## Referencias

- Validación (modo sin documentación): `docs/audits/validacion-main-20260217.json`
- Procesos: `SddIA/process/README.md`
- Acción validate: `SddIA/actions/validate.md`
