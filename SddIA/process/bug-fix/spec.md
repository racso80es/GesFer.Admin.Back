# Proceso: Bug Fix

Este documento define el **proceso de tarea** para la corrección de un bug. Está ubicado en paths.processPath/bug-fix/ (Cúmulo). La ruta de persistencia se obtiene de **Cúmulo** (paths.fixPath/<nombre_fix>).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): solicita/genera en la carpeta de la tarea (Cúmulo) al menos un **`.md`** (objectives.md, spec.md, clarify.md) y al menos un **`.json`** (spec.json, clarify.json, validacion.json).

## Propósito

El proceso **bug-fix** orquesta el ciclo de vida del bug: triaje, documentación, reproducción, alcance mínimo del fix y trazabilidad en la carpeta de persistencia del fix (paths.fixPath/<nombre_fix>).

## Alcance

- **Rama:** fix/<nombre_fix> (nunca master).
- **Documentación:** Carpeta paths.fixPath/<nombre_fix>/ con objectives.md (objetivo, pasos de reproducción, causa raíz o hipótesis), spec.md/spec.json, clarify.md/clarify.json si aplica, implementation, validacion.json.
- **Skills:** iniciar-rama, documentation, filesystem-ops, dotnet-development.
- **Restricciones:** Alcance mínimo (solo causa raíz); no refactorizar ni ampliar funcionalidad en la misma rama.

## Integración

El agente Bug Fix Specialist (definición en paths.processPath/bug-fix/) orquesta el ciclo. En la descripción del PR y en Evolution Logs, la referencia canónica es **paths.fixPath/<nombre_fix>/** (Cúmulo). SSOT para ese fix.
