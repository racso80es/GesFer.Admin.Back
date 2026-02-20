# Proceso: Refactorización

Este documento define el **proceso de tarea** para una refactorización. Está ubicado en paths.processPath/refactorization.md (Cúmulo). Las acciones que orquesta están en paths.actionsPath (Cúmulo). La ruta de persistencia se obtiene de **Cúmulo** (paths.featurePath/refactorization-<nombre_refactor>; mismo espacio que features hasta que se defina paths.refactorPath).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): solicita/genera en la carpeta de la tarea (Cúmulo) al menos un **`.md`** (objectives.md, spec.md, clarify.md, plan, etc.) y al menos un **`.json`** (spec.json, clarify.json, implementation.json, validacion.json, etc.).

## Propósito

El proceso **refactorization** define el procedimiento formal de ciclo completo para una refactorización: mismo flujo que feature (rama, documentación, spec, implementación, validación, cierre), adaptado al contexto de refactor (cambios estructurales o de dominio sin añadir funcionalidad nueva). Orquesta las acciones **spec**, **clarify**, **planning**, **implementation**, **execution**, **validate** y **finalize** en secuencia y garantiza trazabilidad en los logs de evolución.

Proporciona un flujo repetible y auditado, alineado con las Leyes Universales. Desde el punto de vista de SddIA, el dominio no referencia **scripts** sino **skills** o **herramientas** (paths.skillCapsules, paths.toolCapsules, definiciones en paths.skillsDefinitionPath / paths.toolsDefinitionPath).

## Alcance del procedimiento

Ruta de la tarea: Cúmulo (paths.featurePath/refactorization-<nombre_refactor>).

| Fase | Nombre | Descripción |
| :--- | :--- | :--- |
| **0** | Preparar entorno | Crear rama feat/refactorization-<nombre_refactor> desde `master` actualizado. No trabajar en `master`. **Skill:** iniciar-rama (paths.skillCapsules[\"iniciar-rama\"]). Parámetros: BranchType feat, BranchName refactorization-<nombre_refactor>. |
| **1** | Documentación con objetivos | Documentar objetivo, alcance, análisis de situación actual y ley aplicada. La documentación de la tarea se ubica en la carpeta de la tarea (Cúmulo)/objectives.md. |
| **2** | Especificación | Ejecutar o generar SPEC (acción **spec**). Entrada: objectives.md; salida: carpeta de la tarea (Cúmulo)/spec.md y spec.json. |
| **3** | Clarificación | Ejecutar acción **clarify**. Entrada: spec; salida: carpeta de la tarea (Cúmulo)/clarify.md, clarify.json. |
| **4** | Planificación | Ejecutar acción **planning**. Salida: carpeta de la tarea (Cúmulo)/plan. |
| **5** | Implementación (doc) | Generar documento de implementación (acción **implementation**). Salida: carpeta de la tarea (Cúmulo)/implementation.md, implementation.json. |
| **6** | Ejecución | Aplicar el plan (Tekton). Acción **execution**. Salida: carpeta de la tarea (Cúmulo)/execution.json. |
| **7** | Validar | Ejecutar validación pre-PR (acción **validate**). Salida: carpeta de la tarea (Cúmulo)/validacion.json. |
| **8** | Finalizar | Cierre y PR (acción **finalize**). Salida: Evolution Logs y Pull Request. |

## Contenido mínimo de la carpeta de la tarea (Cúmulo)

| Documento | Contenido |
| :--- | :--- |
| **objectives.md** | Objetivo de la refactorización, alcance, análisis de situación actual, ley aplicada, resumen de fases. |
| **spec.md** / **spec.json** | Especificación técnica de la refactorización. |
| **clarify.md** / **clarify.json** | Clarificaciones y decisiones (si aplica). |
| **implementation.md** / **implementation.json** | Touchpoints y plan de implementación. |
| **validacion.json** | Resultado de la validación pre-PR. |

## Actualización de Evolution Logs

Al cierre (fase 8): paths.evolutionPath + paths.evolutionLogFile — añadir sección con fecha, título de la refactorización, resumen y referencia a la carpeta de la tarea (Cúmulo)/objectives.md.

## Integración con Agentes

- **Arquitecto:** Inicia el procedimiento y asegura la fase 1 y la ubicación (Cúmulo).
- **Tekton Developer:** Ejecuta fases 4–8; aplica la SPEC como marco legal.
- **Cúmulo:** Valida que la documentación esté en la carpeta de la tarea (Cúmulo) como SSOT.

## Dependencias

El proceso **refactorization** utiliza las mismas acciones que **feature** en paths.actionsPath (Cúmulo). La documentación canónica de la tarea reside en **paths.featurePath/refactorization-<nombre_refactor>/** (Cúmulo).

## Estándares de Calidad

- **Grado S+:** Trazabilidad desde el objetivo hasta el PR.
- **Ley GIT:** Ningún commit en `master`; todo en rama `feat/refactorization-<nombre_refactor>` con documentación en la carpeta de la tarea (Cúmulo).
- **Single Source of Truth:** Para cada refactorización, la documentación canónica es la carpeta de la tarea (Cúmulo); la referencia en PR y en Evolution Log es esa ruta.

---
*Proceso reflejo de feature, adaptado al contexto de refactorización. Referencia: paths.processPath/feature.md.*
