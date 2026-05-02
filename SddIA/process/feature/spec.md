---
contract_ref: paths.processPath/process-contract.md
persist_ref: paths.featurePath/<nombre_feature>
phases:
- description: Ejecutar git-workspace-recon para validar entorno limpio. Tras confirmar, crear rama feat/<nombre> o fix/<nombre> desde master usando git-branch-manager.
  id: '0'
  name: Preparar entorno
- description: objectives.md en carpeta de la tarea.
  id: '1'
  name: Documentación con objetivos
- description: Acción spec; salida spec.md (frontmatter YAML + Markdown).
  id: '2'
  name: Especificación
- description: Acción clarify; clarify.md (frontmatter YAML + Markdown).
  id: '3'
  name: Clarificación
- description: Acción planning; plan.md (frontmatter YAML + Markdown).
  id: '4'
  name: Planificación
- description: Acción implementation; implementation.md (frontmatter YAML + Markdown).
  id: '5'
  name: Implementación (doc)
- description: Aplicar el plan al código. Consolidar hitos ejecutando git-save-snapshot (commits atómicos). Si el entorno se corrompe severamente, usar git-tactical-retreat.
  id: '6'
  name: Ejecución
- description: Ejecutar validación pre-PR. Invocar git-workspace-recon para verificar la coherencia de los archivos mutados contra el plan. Generar validacion.md. Incluye comprobar que, si el diff incluye mutaciones bajo SddIA/ (fuera de SddIA/evolution/ según norma), exista registro evolution antes del cierre (ver fase 8 — Evaluación de Impacto SDDIA).
  id: '7'
  name: Validar
- description: 'Cierre del ciclo. Evaluación de Impacto SDDIA: antes de ejecutar git-sync-remote, analizar los archivos modificados. Si existe CUALQUIER mutación dentro del directorio SddIA/, es obligatorio ejecutar sddia_evolution_register y realizar un git-save-snapshot adicional para consolidar el registro de evolución antes de abrir el Pull Request. Después: git-sync-remote para subida segura al Leviatán, seguido de git-create-pr inyectando el resumen de objectives.md y validacion.md en el cuerpo del Pull Request. Actualizar Evolution Logs (paths.evolutionPath) según contrato de cierre.'
  id: '8'
  name: Finalizar
principles_ref: paths.principlesPath
process_id: feature
related_actions:
- spec
- clarify
- planning
- implementation
- execution
- validate
- finalize-process
related_skills:
- git-workspace-recon
- git-branch-manager
- git-save-snapshot
- git-sync-remote
- git-tactical-retreat
- git-create-pr
- git-close-cycle
- sddia-evolution-register
spec_version: 2.0.0
---

# Proceso: Feature

Este documento define el **proceso de tarea** para desarrollar una funcionalidad. Está ubicado en paths.processPath/feature/ (Cúmulo). Las acciones que orquesta están en paths.actionsPath (Cúmulo). La ruta de persistencia se obtiene de **Cúmulo** (paths.featurePath/<nombre_feature>).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): solicita/genera en la carpeta de la tarea (Cúmulo) artefactos **`.md`** con frontmatter YAML + cuerpo Markdown (objectives.md, spec.md, clarify.md, plan.md, implementation.md, execution.md, validacion.md). **No se usan ficheros `.json` separados.** Patrón: SddIA/norms/features-documentation-pattern.md; contrato: paths.featurePath/features-contract.md.

## Propósito

El proceso **feature** define el procedimiento formal de ciclo completo para desarrollar una funcionalidad o tarea: desde la creación de la rama hasta el cierre y la apertura del Pull Request. Orquesta las acciones **spec**, **clarify**, **planning**, **implementation**, **execution**, **validate** y **finalize-process** en secuencia, fija la ubicación de la documentación de la tarea y garantiza trazabilidad en los logs de evolución. Integra el **Arsenal Táctico Git** (grado S+): reconocimiento de workspace, gestión de rama, snapshots, sincronización remota, retirada táctica y apertura de PR vía skills dedicadas.

Proporciona un flujo repetible y auditado, alineado con las Leyes Universales (soberanía documental en AGENTE_CUMULO, no commits en `master`).

## Alcance del procedimiento

Ruta de la tarea: Cúmulo (paths.featurePath/<nombre_feature>).

| Fase | Nombre | Descripción |
| :--- | :--- | :--- |
| **0** | Preparar entorno | Ejecutar **git-workspace-recon** para validar entorno limpio. Tras confirmar, crear rama `feat/<nombre_feature>` o `fix/<nombre>` desde `master` usando **git-branch-manager**. No trabajar en `master`. Invocación: paths.skillCapsules y contrato capsule-json-io (SddIA/norms/capsule-json-io.md). |
| **1** | Documentación con objetivos | Documentar objetivo, alcance y ley aplicada. La documentación de la tarea se ubica en la carpeta de la tarea (Cúmulo)/objectives.md. |
| **2** | Especificación | Ejecutar o generar SPEC (acción **spec**). Entrada: requerimiento o borrador, carpeta de la tarea (Cúmulo)/objectives.md; salida: especificación técnica en carpeta de la tarea (Cúmulo)/spec.md (frontmatter YAML + Markdown). |
| **3** | Clarificación | Ejecutar o generar clarificaciones (acción **clarify**). Entrada: carpeta de la tarea (Cúmulo)/objectives.md, spec.md; salida: carpeta de la tarea (Cúmulo)/clarify.md (frontmatter YAML + Markdown). |
| **4** | Planificación | Ejecutar o generar plan (acción **plan**). Entrada: Especificación, Clarificación. Salida: carpeta de la tarea (Cúmulo)/plan.md (frontmatter YAML + Markdown). |
| **5** | Implementación | Generar documento de implementación. Entrada: carpeta de la tarea (Cúmulo)/objectives.md, spec.md, clarify.md; salida: carpeta de la tarea (Cúmulo)/implementation.md (frontmatter YAML + Markdown). |
| **6** | Ejecución | Aplicar el plan al código (Tekton Developer). Consolidar hitos ejecutando **git-save-snapshot** (commits atómicos). Si el entorno se corrompe severamente, usar **git-tactical-retreat**. Entrada: carpeta de la tarea (Cúmulo)/implementation.md; salida: carpeta de la tarea (Cúmulo)/execution.md (frontmatter YAML + Markdown). |
| **7** | Validar | Ejecutar validación pre-PR. Invocar **git-workspace-recon** para verificar la coherencia de los archivos mutados contra el plan. Generar **validacion.md**. Comprobar que, si el conjunto de cambios incluye mutaciones bajo **SddIA/** (excl. **SddIA/evolution/** según norma de validación CI), el cierre no proceda sin registro evolution aplicado (detalle en fase 8). Entrada: carpeta de la tarea (Cúmulo); salida: carpeta de la tarea (Cúmulo)/validacion.md (frontmatter YAML + Markdown). |
| **8** | Cierre (**finalize-process**) | **Evaluación de Impacto SDDIA:** antes de **git-sync-remote**, analizar archivos modificados. Si existe **cualquier** mutación dentro de **SddIA/**, es obligatorio ejecutar **sddia_evolution_register** y un **git-save-snapshot** adicional que consolide ese registro **antes** de **git-create-pr**. Cierre del ciclo: **git-sync-remote** (subida segura), **git-create-pr** (objectives + validacion en el cuerpo del PR). Actualizar Evolution Logs de producto (paths.evolutionPath) según acción **finalize-process**. Entrada: carpeta de la tarea (Cúmulo); salida: Evolution Logs y Pull Request. |

## Implementación

Este proceso se implementa como **procedimiento** que combina:

*   **Skills y agentes** para spec, clarify y plan (cuando se requiera trazabilidad con token de auditor), según las acciones definidas en paths.actionsPath (Cúmulo).
*   **Ubicación obligatoria de la documentación de la tarea:** paths.featurePath/<nombre_feature>/ (Cúmulo).

### Contenido mínimo de la carpeta de la tarea (Cúmulo: paths.featurePath/<nombre_feature>/)

| Documento | Contenido |
| :--- | :--- |
| **objectives.md** | Objetivo, alcance, ley aplicada (frontmatter YAML + Markdown). |
| **spec.md** | Especificación técnica (frontmatter YAML + Markdown). |
| **clarify.md** | Clarificaciones y decisiones (frontmatter YAML + Markdown). |
| **plan.md** | Plan de implementación / task roadmap (frontmatter YAML + Markdown). |
| **implementation.md** | Touchpoints y ítems de implementación (frontmatter YAML + Markdown). |
| **execution.md** | Registro de ejecución (frontmatter YAML + Markdown). |
| **validacion.md** | Informe de validación pre-PR (frontmatter YAML + Markdown). |

### Actualización de Evolution Logs

Al cierre de la feature (fase 8):

*   **paths.evolutionPath + paths.evolutionLogFile:** Añadir una línea con formato `[YYYY-MM-DD] [feat/<nombre>] [Descripción breve del resultado.] [Estado].`
*   **paths.evolutionPath + paths.evolutionLogFile:** Añadir una sección con fecha y título de la feature, resumen y referencia a la carpeta de la tarea (Cúmulo)/objectives.md.

## Integración con Agentes

*   **Arquitecto / Spec Architect:** Puede iniciar el procedimiento y asegurar que la fase 1 y la ubicación paths.featurePath/<nombre_feature>/ (Cúmulo) se respeten.
*   **Clarifier:** Responsable de la fase 3 (clarificación) y de persistir decisiones en el SPEC y en la carpeta de la feature.
*   **Tekton Developer:** Ejecuta las fases 4 (plan), 5 (implementación), 6 (ejecución), 7 (validación) y 8 (cierre/PR); aplica la SPEC como marco legal.
*   **Cúmulo:** Valida que la documentación de la tarea esté en paths.featurePath/<nombre_feature>/ como SSOT para esa feature.

## Dependencias con otras acciones

*   El proceso **feature** invoca o utiliza los resultados de las acciones **spec**, **clarify**, **plan**, **implementation**, **execution**, **validate** y **finalize-process** en paths.actionsPath (Cúmulo).
*   La **documentación de la tarea** (objetivo, spec, clarifications, plan, validacion) debe residir en **paths.featurePath/<nombre_feature>/** (Cúmulo) para aprobación y revisión humana.

## Estándares de Calidad

*   **Grado S+:** Trazabilidad desde el objetivo hasta el PR: rama → paths.featurePath → spec/clarify/plan → implementación → execution → validación → Evolution Logs → PR. **Arsenal Táctico Git:** git-workspace-recon → git-branch-manager → git-save-snapshot → (git-tactical-retreat si aplica) → git-workspace-recon (pre-PR) → **sddia_evolution_register (si mutó SddIA/)** + git-save-snapshot del registro → git-sync-remote → git-create-pr.
*   **Ley GIT:** Ningún commit en `master`; todo el trabajo en rama `feat/` o `fix/` con documentación en paths.featurePath/<nombre_feature>/ (Cúmulo).
*   **Single Source of Truth:** Para cada feature, la documentación canónica de la tarea es paths.featurePath/<nombre_feature>/ (Cúmulo); la referencia en PR y en Evolution Log es esa ruta.

## Alcance para Fix (bug)

El mismo patrón de persistencia se aplica a correcciones de bugs mediante el proceso **bug-fix**. La ubicación de la documentación se obtiene del agente Cúmulo (paths.fixPath/<nombre_fix>). Ver paths.processPath/bug-fix/.

## Referencia de ejecución

Procedimiento aplicado en la rama **feat/e2e-product-back-mocked** (2026-02-10). Documentación de la tarea: paths.featurePath/<nombre_feature>/. Acciones relacionadas: paths.actionsPath (spec/, clarify/, planning/, execution/, validate/, finalize-process/).
