# Pull Request: SddIA procesos, validate y documentación

## Descripción

Cierre de la tarea de definición de procesos de tarea en SddIA, refinamiento de la acción validate (validación de cambios git obligatoria) y establecimiento de Cúmulo como fuente de rutas. Documentación de la feature en **docs/features/sddia-process-validate-docs/**.

## Documentación de la tarea (SSOT)

- **Objetivo y alcance:** [docs/features/sddia-process-validate-docs/objectives.md](../../features/sddia-process-validate-docs/objectives.md)
- **Evolution Log (detalle):** [docs/evolution/EVOLUTION_LOG.md](../../evolution/EVOLUTION_LOG.md#2026-02-17--sddia-procesos-validate-y-documentación)
- **Validación (modo pr_analysis):** [docs/audits/validacion-main-20260217.json](../../audits/validacion-main-20260217.json)

## Cambios principales

- **SddIA/process:** Procesos feature y bug-fix; feature.md y bug-fix-specialist.json; interfaz .md/.json (Cúmulo).
- **AGENTS.md:** Inicio de tarea (procesos), Cúmulo como fuente de paths, interfaz de procesos, rol BUG-FIX.
- **SddIA/actions:** execution, validate (siempre con git_changes), finalize; spec/clarify/agents referencian Cúmulo.
- Eliminación de referencias a knowledge-architect.
- Evolution Logs y docs/audits actualizados.

## Crear el PR

Abrir PR desde: https://github.com/racso80es/GesFer.Admin.Back/pull/new/feat/sddia-process-validate-docs  
Base: `main` ← Head: `feat/sddia-process-validate-docs`. Pegar esta descripción en el cuerpo del PR.
