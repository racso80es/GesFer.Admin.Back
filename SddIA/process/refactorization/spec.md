---
contract_ref: paths.processPath/process-contract.md
persist_ref: paths.featurePath/refactorization-<nombre_refactor>
phases:
- description: Ejecutar git-workspace-recon para validar entorno limpio. Tras confirmar, crear rama feat/refactorization-<nombre_refactor> con git-branch-manager (nunca master).
  id: '0'
  name: Preparar entorno
- description: objectives.md.
  id: '1'
  name: Documentación con objetivos
- description: Acción spec.
  id: '2'
  name: Especificación
- description: Acción clarify.
  id: '3'
  name: Clarificación
- description: Acción planning.
  id: '4'
  name: Planificación
- description: Acción implementation.
  id: '5'
  name: Implementación (doc)
- description: Acción execution aplicada al código. Consolidar hitos con git-save-snapshot (commits atómicos). Ante fallo estructural, git-tactical-retreat.
  id: '6'
  name: Ejecución
- description: Acción validate. Opcionalmente git-workspace-recon para coherencia pre-PR.
  id: '7'
  name: Validar
- description: Cierre. git-sync-remote; git-create-pr enlazando objectives.md y validacion en el cuerpo del PR. Evolution Logs.
  id: '8'
  name: Finalizar
process_id: refactorization
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
spec_version: 2.0.0
---

# Proceso: Refactorización

Este documento define el **proceso de tarea** para una refactorización. Está ubicado en paths.processPath/refactorization/ (Cúmulo). Las acciones que orquesta están en paths.actionsPath (Cúmulo). La ruta de persistencia se obtiene de **Cúmulo** (paths.featurePath/refactorization-<nombre_refactor>; mismo espacio que features hasta que se defina paths.refactorPath).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): solicita/genera en la carpeta de la tarea (Cúmulo) al menos un **`.md`** (objectives.md, spec.md, clarify.md, plan, etc.) y al menos un **`.json`** (spec.json, clarify.json, implementation.json, validacion.json, etc.).

## Propósito

El proceso **refactorization** define el procedimiento formal de ciclo completo para una refactorización: mismo flujo que feature (rama, documentación, spec, implementación, validación, cierre), adaptado al contexto de refactor (cambios estructurales o de dominio sin añadir funcionalidad nueva). Orquesta las acciones **spec**, **clarify**, **planning**, **implementation**, **execution**, **validate** y **finalize-process** en secuencia y garantiza trazabilidad en los logs de evolución.

Proporciona un flujo repetible y auditado, alineado con las Leyes Universales. Desde el punto de vista de SddIA, el dominio no referencia **scripts** sino **skills** o **herramientas** (paths.skillCapsules, paths.toolCapsules, definiciones en paths.skillsDefinitionPath / paths.toolsDefinitionPath).

## Alcance del procedimiento

Ruta de la tarea: Cúmulo (paths.featurePath/refactorization-<nombre_refactor>).

| Fase | Nombre | Descripción |
| :--- | :--- | :--- |
| **0** | Preparar entorno | **git-workspace-recon**; aislar contexto con **git-branch-manager** en `feat/refactorization-<nombre_refactor>`. |
| **1** | Documentación con objetivos | objectives.md en carpeta de la tarea (Cúmulo). |
| **2** | Especificación | Acción spec. Salida: spec.md, spec.json. |
| **3** | Clarificación | Acción clarify. Salida: clarify.md, clarify.json. |
| **4** | Planificación | Acción planning. Salida: plan. |
| **5** | Implementación (doc) | Acción implementation. Salida: implementation.md, implementation.json. |
| **6** | Ejecución | Acción execution; **git-save-snapshot** por hitos; emergencia **git-tactical-retreat**. Salida: execution.json (si el ciclo lo exige). |
| **7** | Validar | Acción validate. Salida: validacion.json. |
| **8** | Finalizar | **git-sync-remote**; **git-create-pr** (cuerpo del PR con objetivos y validación). Evolution Logs. |

## Contenido mínimo de la carpeta de la tarea (Cúmulo)

| Documento | Contenido |
| :--- | :--- |
| **objectives.md** | Objetivo, alcance, análisis de situación actual, ley aplicada. |
| **spec.md** / **spec.json** | Especificación técnica de la refactorización. |
| **clarify.md** / **clarify.json** | Clarificaciones (si aplica). |
| **implementation.md** / **implementation.json** | Touchpoints y plan de implementación. |
| **validacion.json** | Resultado de la validación pre-PR. |

## Actualización de Evolution Logs

Al cierre (fase 8): paths.evolutionPath + paths.evolutionLogFile — añadir sección con fecha, título, resumen y referencia a la carpeta de la tarea (Cúmulo)/objectives.md.

## Integración con Agentes

- **Arquitecto:** Inicia el procedimiento y asegura la fase 1 y la ubicación (Cúmulo).
- **Tekton Developer:** Ejecuta fases 4–8; aplica la SPEC como marco legal.
- **Cúmulo:** Valida que la documentación esté en la carpeta de la tarea (Cúmulo) como SSOT.

## Dependencias

El proceso **refactorization** utiliza las mismas acciones que **feature** en paths.actionsPath (Cúmulo). Referencia: paths.processPath/feature/.

## Estándares de Calidad

- **Grado S+:** Trazabilidad desde el objetivo hasta el PR. Arsenal táctico: git-workspace-recon → git-branch-manager → git-save-snapshot → (git-tactical-retreat si aplica) → git-sync-remote → git-create-pr.
- **Ley GIT:** Ningún commit en `master`; documentación en la carpeta de la tarea (Cúmulo).
- **Single Source of Truth:** Documentación canónica en la carpeta de la tarea (Cúmulo).

---
*Proceso reflejo de feature, adaptado al contexto de refactorización. Referencia: paths.processPath/feature/.*
