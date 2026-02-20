# Proceso: Creación de herramientas (create-tool)

Este documento define el **proceso de tarea** para crear una nueva herramienta (tool) en el proyecto. Está ubicado en paths.processPath/create-tool/ (Cúmulo). Las rutas de herramientas se obtienen de **Cúmulo** (paths.toolsPath, paths.toolCapsules, paths.toolsIndexPath).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): la tarea de creación genera en la carpeta de la tarea (Cúmulo) al menos un **`.md`** (objectives.md, spec.md, implementation.md) y al menos un **`.json`** (spec.json, implementation.json, validacion.json). El **resultado ejecutable** es la cápsula en **paths.toolCapsules[<tool-id>]** con todos los artefactos requeridos por el contrato de herramientas.

## Propósito

El proceso **create-tool** define el procedimiento para incorporar una nueva herramienta al ecosistema de paths.toolsPath (Cúmulo): desde la definición del objetivo hasta la cápsula lista, el índice actualizado y Cúmulo sincronizado. Garantiza que cada herramienta cumpla SddIA/tools/tools-contract.json y que las rutas queden registradas en Cúmulo y en scripts/tools/index.json.

## Alcance del procedimiento

- **Documentación de la tarea:** Cúmulo (paths.featurePath/create-tool-<tool-id>/).
- **Definición (SddIA):** paths.toolsDefinitionPath/<tool-id>/ con spec.md y spec.json (implementation_path_ref obligatorio).
- **Cápsula (implementación):** paths.toolCapsules[<tool-id>].

Fases: 0 Preparar entorno | 1 Objetivos y especificación | 1b Definición en SddIA | 2–6 Cápsula, manifest, scripts, índice, Cúmulo | 7 Opcional Rust | 8 Validación | 9 Cierre.

## Restricciones

- toolId en kebab-case. Rama feat/create-tool-<tool-id>. Windows 11, PowerShell 7+. Contrato tools (salida JSON, feedback) obligatorio.

## Referencias

- Contrato: SddIA/tools/tools-contract.json, tools-contract.md.
- Cúmulo: paths.toolsDefinitionPath, paths.toolsPath, paths.toolCapsules, paths.toolsIndexPath.
- Proceso machine-readable: paths.processPath/create-tool/spec.json.
