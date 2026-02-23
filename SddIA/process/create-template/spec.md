# Proceso: Creación de plantillas (create-template)

Este documento define el **proceso de tarea** para crear una nueva plantilla (template) en el proyecto. Está ubicado en paths.processPath/create-template/ (Cúmulo). Las rutas de plantillas se obtienen de **Cúmulo** (paths.templatesPath).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): la tarea de creación genera en la carpeta de la tarea (Cúmulo) al menos un **`.md`** (objectives.md, spec.md) y al menos un **`.json`** (spec.json). El **resultado** es la carpeta en **paths.templatesPath/<template-id>/** con spec.md y spec.json, cumpliendo SddIA/templates/templates-contract.json.

## Propósito

El proceso **create-template** define el procedimiento para incorporar una nueva plantilla al ecosistema de paths.templatesPath (Cúmulo): desde la definición del objetivo hasta la plantilla lista con spec.md y spec.json. Garantiza que cada plantilla cumpla SddIA/templates/templates-contract.json y que quede registrada en Cúmulo (paths.templatesPath). Las plantillas procedimentan el uso de procesos mediante configuraciones predefinidas con un fin concreto.

## Alcance del procedimiento

- **Documentación de la tarea:** Cúmulo (paths.featurePath/create-template-<template-id>/).
- **Entregable:** paths.templatesPath/<template-id>/ con spec.md y spec.json (y opcionalmente config.json).

Fases: 0 Preparar entorno | 1 Objetivos y especificación de la plantilla | 2 Redactar spec.md y spec.json según templates-contract | 3 Validación | 4 Cierre.

## Restricciones

- template_id en kebab-case. Rama feat/create-template-<template-id>. Windows 11, PowerShell 7+. Contrato templates (SddIA/templates/templates-contract.json) obligatorio. process_ref debe existir en paths.processPath.

## Referencias

- Contrato: SddIA/templates/templates-contract.json, templates-contract.md.
- Cúmulo: paths.templatesPath, paths.featurePath, paths.processPath.
- Proceso machine-readable: paths.processPath/create-template/spec.json.
