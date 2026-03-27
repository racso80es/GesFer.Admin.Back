---
feature_name: refactor-automatic-task-kaizen-queue
created: 2026-03-27
base: objectives.md
status: planned
spec_version: 1.2
clarify_ref: clarify.md
plan_ref: plan.md
---

# Especificación — Cola Kaizen en automatic_task

## 1. Problema actual

En **Triage**, si no hay ficheros de tarea en la bandeja principal, el proceso indica **crear de inmediato** una nueva tarea Kaizen y ejecutarla. Eso ignora el caso en que ya existan **tareas Kaizen acumuladas** (p. ej. definidas en auditorías o sesiones previas) que deberían ejecutarse antes de inventar otra.

## 2. Comportamiento deseado

1. **Primero:** localizar tareas pendientes **no Kaizen** en la bandeja principal (`paths.tasksPath`, solo `.md` en el directorio raíz de tareas, sin subcarpetas).
2. **Si no hay ninguna:** buscar tareas **Kaizen pendientes** en la cola dedicada (ver §3).
3. **Si hay al menos una en cola Kaizen:** seleccionar **la más antigua** (criterio: fecha en nombre del fichero `Kaizen_YYYY_MM_DD*.md` o, si el fichero tiene frontmatter con `created` / `date`, el valor más antiguo).
4. **Ejecución:** la tarea elegida (normal o Kaizen de cola) sigue **el mismo procedimiento** que hoy: activación en `ACTIVE/`, proceso feature (u otro indicado en la tarea), finalización en `DONE/`, etc.
5. **Solo si** no hay tareas en la bandeja principal **ni** en la cola Kaizen: entonces **analizar el proyecto** y **registrar una nueva** tarea Kaizen en la cola (§3) y proceder con el ciclo habitual.

## 3. Modelo de carpetas (propuesta)

| Ubicación | Contenido |
| :--- | :--- |
| `paths.tasksPath` (raíz) | Tareas pendientes **generales** (solo `.md` sueltos; no incluye subcarpetas al listar “pendientes”). |
| `paths.tasksPath/KAIZEN/` | **Cola** de tareas Kaizen **ya especificadas** y pendientes de ejecución. Nombres recomendados: `Kaizen_YYYY_MM_DD_<slug>.md`. |

Las tareas en `ACTIVE/`, `CLARIFY/`, `DONE/` no se consideran disponibles para triaje.

## 4. Cambios en artefactos SSOT

- **`SddIA/process/automatic_task/spec.md`:** **completado** (v **1.1.0**) — triaje en tres pasos (raíz → `KAIZEN/` por antigüedad → nueva Kaizen), estructura de carpetas y rutas vía `paths.tasksPath`; registro evolution asociado en `SddIA/evolution/` (id `f8e2d4c1-7b3a-4f9e-8c6d-1a2b3c4d5e6f`).
- **Opcional (plan P3):** revisar normas o `AGENTS.md` si aún mencionan solo literales `docs/TASKS` o Kaizen únicamente en raíz; difusión vía acción **sddia-difusion** si se detecta contradicción.

## 5. Criterios de validación

- El texto del proceso describe un orden **claro**: backlog raíz → cola `KAIZEN/` por antigüedad → creación nueva Kaizen.
- La transición ACTIVE/DONE es **única** para tareas normales y Kaizen de cola.
- Rutas descritas en términos de Cúmulo (`paths.tasksPath`) donde aplique.

## 6. Fuera de alcance (v1)

- Migración automática de ficheros `Kaizen_*.md` históricos que pudieran estar en la raíz de `paths.tasksPath` (se puede hacer a mano o en otra tarea).
- Herramienta automatizada que ordene por fecha; el ejecutor humano/IA aplica el criterio de antigüedad según §2.3.
