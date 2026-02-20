# Proceso: Feature

Este documento define el **proceso de tarea** para desarrollar una funcionalidad. Está ubicado en paths.processPath/feature.md (Cúmulo). Las acciones que orquesta están en paths.actionsPath (Cúmulo). La ruta de persistencia se obtiene de **Cúmulo** (paths.featurePath/<nombre_feature>).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): solicita/genera en la carpeta de la tarea (Cúmulo) al menos un **`.md`** (objectives.md, spec.md, clarify.md, plan, etc.) y al menos un **`.json`** (spec.json, clarify.json, implementation.json, validacion.json, etc.).

## Propósito

El proceso **feature** define el procedimiento formal de ciclo completo para desarrollar una funcionalidad o tarea: desde la creación de la rama hasta el cierre y la apertura del Pull Request. Orquesta las acciones **spec**, **clarify**, **planning**, **implementation**, **execution**, **validate** y **finalize** en secuencia, fija la ubicación de la documentación de la tarea y garantiza trazabilidad en los logs de evolución.

Proporciona un flujo repetible y auditado, alineado con las Leyes Universales (soberanía documental en AGENTE_CUMULO, no commits en `master`).

## Alcance del procedimiento

Ruta de la tarea: Cúmulo (paths.featurePath/<nombre_feature>).

| Fase | Nombre | Descripción |
| :--- | :--- | :--- |
| **0** | Preparar entorno | Crear rama feat/<nombre_feature> (o `fix/` si aplica) desde `master` actualizado. No trabajar en `master`. **Skill:** iniciar-rama — invocar según contrato (paths.skillsDefinitionPath/iniciar-rama/). Tekton invoca la implementación (paths.skillCapsules[\"iniciar-rama\"]). Parámetros: BranchType feat, BranchName <nombre_feature>. |
| **1** | Documentación con objetivos | Documentar objetivo, alcance y ley aplicada. La documentación de la tarea se ubica en la carpeta de la tarea (Cúmulo)/objectives.md. |
| **2** | Especificación | Ejecutar o generar SPEC (acción **spec**). Entrada: requerimiento o borrador, carpeta de la tarea (Cúmulo)/objectives.md; salida: especificación técnica en paths.actionsPath (spec.md) y copia/canon en carpeta de la tarea (Cúmulo)/spec.md y spec.json |
| **3** | Clarificación | Ejecutar o generar clarificaciones (acción **clarify**). Especificación técnica: paths.actionsPath/clarify.md. Entrada: carpeta de la tarea (Cúmulo)/objectives.md, spec.json; salida: carpeta de la tarea (Cúmulo)/clarify.md, clarify.json |
| **4** | Planificación | Ejecutar o generar plan (acción **plan**). Entrada: Especificación, Clarificación. Salida: carpeta de la tarea (Cúmulo)/plan (y/o clarify según convención). |
| **5** | Implementación | Generar documento de implementación. Especificación técnica: paths.actionsPath/implementation.md. Entrada: carpeta de la tarea (Cúmulo)/objectives.md, spec.json, clarify.json; salida: carpeta de la tarea (Cúmulo)/implementation.md, implementation.json |
| **6** | Ejecución | Aplicar el plan al código (Tekton Developer). Especificación técnica: paths.actionsPath/execution.md. Entrada: carpeta de la tarea (Cúmulo)/implementation.json; salida: carpeta de la tarea (Cúmulo)/execution.json |
| **7** | Validar | Ejecutar validación pre-PR. Especificación técnica: paths.actionsPath/validate.md. Entrada: carpeta de la tarea (Cúmulo); salida: carpeta de la tarea (Cúmulo)/validacion.json |
| **8** | Finalizar | Cierre y PR. Especificación técnica: paths.actionsPath/finalize.md. Entrada: carpeta de la tarea (Cúmulo); salida: Evolution Logs y Pull Request. |

## Implementación

Este proceso se implementa como **procedimiento** que combina:

*   **Scripts y agentes** para spec, clarify y plan (cuando se requiera trazabilidad con token de auditor), según las acciones definidas en paths.actionsPath (Cúmulo).
*   **Ubicación obligatoria de la documentación de la tarea:** paths.featurePath/<nombre_feature>/ (Cúmulo).

### Contenido mínimo de la carpeta de la tarea (Cúmulo: paths.featurePath/<nombre_feature>/)

| Documento | Contenido |
| :--- | :--- |
| **objectives.md** (u OBJETIVO.md) | Objetivo, alcance, ley aplicada, resumen del proceso (fases 0–8), cierre y PR, referencias. |
| **spec.md** / SPEC-* | Especificación técnica (puede generarse con `--spec` y copiarse/adaptarse aquí). |
| **clarify.md** / SPEC-*_CLARIFICATIONS.md | Clarificaciones y decisiones (puede generarse con `--clarify` y copiarse aquí). |
| **PLAN-*** | Plan de implementación / task roadmap (puede generarse con `--plan` y copiarse aquí). |

### Actualización de Evolution Logs

Al cierre de la feature (fase 8):

*   **docs/EVOLUTION_LOG.md o paths.evolutionPath + paths.evolutionLogFile:** Añadir una línea con formato `[YYYY-MM-DD] [feat/<nombre>] [Descripción breve del resultado.] [Estado].`
*   **paths.evolutionPath + paths.evolutionLogFile:** Añadir una sección con fecha y título de la feature, resumen y referencia a la carpeta de la tarea (Cúmulo)/objectives.md.

## Integración con Agentes

*   **Arquitecto / Spec Architect:** Puede iniciar el procedimiento y asegurar que la fase 1 (documentación con objetivos) y la ubicación paths.featurePath/<nombre_feature>/ (Cúmulo) se respeten.
*   **Clarifier:** Responsable de la fase 3 (clarificación) y de persistir decisiones en el SPEC y en la carpeta de la feature.
*   **Tekton Developer:** Ejecuta las fases 4 (plan), 5 (implementación), 6 (ejecución), 7 (validación) y 8 (cierre/PR); aplica la SPEC como marco legal.
*   **Cúmulo:** Valida que la documentación de la tarea esté en paths.featurePath/<nombre_feature>/ como SSOT para esa feature.

## Dependencias con otras acciones

*   El proceso **feature** invoca o utiliza los resultados de las acciones **spec**, **clarify**, **plan**, **implementation**, **execution**, **validate** y **finalize** en paths.actionsPath (Cúmulo).
*   La **documentación de la tarea** (objetivo, spec, clarifications, plan, validacion) debe residir en **paths.featurePath/<nombre_feature>/** (Cúmulo) para aprobación y revisión humana.

## Estándares de Calidad

*   **Grado S+:** Trazabilidad desde el objetivo hasta el PR: rama → paths.featurePath → spec/clarify/plan → implementación → execution → validación → Evolution Logs → PR.
*   **Ley GIT:** Ningún commit en `master`; todo el trabajo en rama `feat/` o `fix/` con documentación en paths.featurePath/<nombre_feature>/ (Cúmulo).
*   **Single Source of Truth:** Para cada feature, la documentación canónica de la tarea es paths.featurePath/<nombre_feature>/ (Cúmulo); la referencia en PR y en Evolution Log es esa ruta.

## Alcance para Fix (bug)

El mismo patrón de persistencia se aplica a correcciones de bugs mediante el proceso **bug-fix**. La ubicación de la documentación se obtiene del agente Cúmulo (paths.fixPath/<nombre_fix>; en este repositorio fixPath = ./docs/bugs/).

| Documento en carpeta de la tarea (Cúmulo)/ | Contenido |
| :--- | :--- |
| **objectives.md** | Objetivo del fix, pasos de reproducción, causa raíz (o hipótesis), ley aplicada. |
| **spec.md / spec.json** | Especificación técnica del fix (generada con `--spec --context` y la ruta de Cúmulo). |
| **clarify.md / clarify.json** | Clarificaciones y decisiones (si aplica). |
| **implementation.md / implementation.json** | Touchpoints y plan de implementación del fix. |
| **validacion.json** | Resultado de la validación pre-PR. |

- Rama: `fix/<nombre_fix>` (nunca `master`).
- En la descripción del PR y en Evolution Logs, la referencia canónica es **paths.fixPath/<nombre_fix>/** (Cúmulo) (SSOT para ese fix).
- El agente **Bug Fix Specialist** (paths.processPath/bug-fix-specialist.json) orquesta el ciclo del fix y asegura que toda la documentación viva en la carpeta de la tarea (Cúmulo).

## Referencia de ejecución

Procedimiento aplicado en la rama **feat/e2e-product-back-mocked** (2026-02-10). Documentación de la tarea: paths.featurePath/<nombre_feature>/. Acciones relacionadas: paths.actionsPath (spec.md, `clarify.md`, `planning.md`, `execution.md`, `validate.md`, `finalize.md`.
