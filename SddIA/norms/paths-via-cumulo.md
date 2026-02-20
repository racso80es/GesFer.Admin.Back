# Norma: Rutas vía Cúmulo

**Fuente:** SddIA/norms. Consulte Cúmulo (paths) para toda ruta de fichero o carpeta.

## Principio

En SddIA **no se escriben rutas de ficheros literales** (ej. `docs/features/`, `scripts/skills/`, `docs/audits/`). Toda ruta se obtiene del **agente Cúmulo** (`SddIA/agents/cumulo.json` → `paths`). El ejecutor (p. ej. Tekton) o cualquier agente que necesite una ruta debe **consultar Cúmulo** para resolver paths.featurePath, paths.auditsPath, paths.skillCapsules[skill-id], etc.

## Claves de paths en Cúmulo (referencia)

- **Persistencia de tareas:** paths.featurePath, paths.fixPath, paths.logPath.
- **Evolution y auditoría:** paths.evolutionPath, paths.evolutionLogFile, paths.auditsPath, paths.accessLogFile.
- **Técnico y operativo:** paths.architecturePath, paths.infrastructurePath, paths.debtPath, paths.tasksPath.
- **SddIA:** paths.actionsPath, paths.processPath, paths.normsPath.
- **Skills y tools:** paths.skillsDefinitionPath, paths.skillCapsules[skill-id], paths.skillsRustPath; paths.toolsDefinitionPath, paths.toolCapsules[tool-id], paths.toolsRustPath.

## Aplicación

- **Actions, process, skills, agents:** Referenciar rutas solo como paths.\<clave\> o paths.\<clave\>[\<id\>]. No usar cadenas literales docs/... ni scripts/... en la documentación de comportamiento.
- **AGENTS.md y constitution:** Indicar que la única fuente de rutas es Cúmulo; no ejemplos con rutas literales salvo en Cúmulo mismo (que es la fuente de verdad).
