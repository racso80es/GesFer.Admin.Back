---
feature_name: refactor-automatic-task-kaizen-queue
created: 2026-03-27
process: feature
---

# Objetivos: Refactor — cola Kaizen en proceso automatic_task

**Rama:** feat/refactor-automatic-task-kaizen-queue  
**Ruta (Cúmulo):** paths.featurePath/refactor-automatic-task-kaizen-queue  
**Proceso:** paths.processPath/feature/

## Objetivo

Ajustar el proceso **automatic_task** para que, cuando no existan tareas pendientes “normales”, se priorice la **cola de tareas Kaizen ya definidas** (ejecución por antigüedad, mismo ciclo ACTIVE → DONE). Solo si esa cola está vacía se debe **analizar el proyecto y proponer una nueva tarea Kaizen**.

## Alcance

- Actualizar `SddIA/process/automatic_task/spec.md` (triaje y estructura de carpetas).
- Definir ubicación y convención de la cola Kaizen pendiente (subcarpeta bajo la ruta de tareas del Cúmulo).
- Alinear referencias a rutas con **paths.tasksPath** (sin literales innecesarios donde el contrato lo permita).

## Ley aplicada

- **L6_CONSULTATION:** Rutas vía Cúmulo (`paths.featurePath`, `paths.processPath`, `paths.tasksPath`).
- **SSOT:** Un solo procedimiento para transiciones ACTIVE/DONE; Kaizen reutiliza el mismo flujo que una tarea normal.

## Fases del proceso (recordatorio)

| Fase | Nombre | Estado |
| :--- | :--- | :--- |
| 0 | Preparar entorno (rama feat, skill iniciar-rama) | Pendiente |
| 1 | Objetivos | Hecho |
| 2 | Spec | Hecho (borrador en esta carpeta) |
| 3 | Clarify | Hecho (`clarify.md`) |
| 4 | Planning | Hecho (`plan.md`) |
| 5–8 | Implementation … Finalize | Pendiente |
