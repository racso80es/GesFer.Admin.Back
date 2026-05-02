---
constraints:
- Rama fix/<nombre_fix>, nunca master.
- Documentación obligatoria en paths.fixPath/<nombre_fix>.
- 'Alcance mínimo: causa raíz; no refactor en la misma rama.'
contract_ref: paths.processPath/process-contract.md
paths:
  bugPath_ref: paths.bugPath (Cúmulo)
  fixPath_ref: paths.fixPath (Cúmulo)
persist_ref: paths.fixPath/<nombre_fix>
phases:
- description: Ejecutar git-workspace-recon para validar entorno limpio. Tras confirmar, crear o fijar rama fix/<nombre_fix> desde master usando git-branch-manager (nunca trabajar en master).
  id: '0'
  name: Preparar entorno
- description: objectives.md en carpeta de la tarea (paths.fixPath).
  id: '1'
  name: Documentación con objetivos
- description: Acción spec; salida spec.md (frontmatter YAML + Markdown).
  id: '2'
  name: Especificación
- description: Acción clarify si aplica; clarify.md (frontmatter YAML + Markdown).
  id: '3'
  name: Clarificación
- description: Acción implementation; implementation.md (frontmatter YAML + Markdown).
  id: '4'
  name: Implementación (doc)
- description: Aplicar el fix al código. Consolidar hitos ejecutando git-save-snapshot (commits atómicos). Ante fallo estructural del entorno o corrupción de contexto, usar git-tactical-retreat como protocolo de emergencia.
  id: '5'
  name: Ejecución
- description: Acción validate; validacion.md. Opcionalmente re-ejecutar git-workspace-recon para coherencia de mutaciones frente al plan.
  id: '6'
  name: Validar
- description: Cierre. Ejecutar git-sync-remote para publicar la rama; seguidamente git-create-pr enlazando en el cuerpo del PR los extractos relevantes de objectives.md y validacion.md. Actualizar Evolution Logs.
  id: '7'
  name: Finalizar
principles_ref: paths.principlesPath
process_id: bug-fix
process_interface_compliance: Solicita/genera en carpeta de la tarea artefactos .md con frontmatter YAML (objectives.md, spec.md, clarify.md, validacion.md). Sin .json separados. Patrón: SddIA/norms/features-documentation-pattern.md.
related_actions:
- spec
- clarify
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
skills:
- documentation
- filesystem-ops
- dotnet-development
spec_version: 2.0.0
triggers:
- Reporte de bug
- Fallo en CI o tests
- Solicitud de fix con bug-id o título
---

# Proceso: Bug Fix

Este documento define el **proceso de tarea** para la corrección de un bug. Está ubicado en paths.processPath/bug-fix/ (Cúmulo). La ruta de persistencia se obtiene de **Cúmulo** (paths.fixPath/<nombre_fix>).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): solicita/genera en la carpeta de la tarea (Cúmulo) artefactos **`.md`** con frontmatter YAML (objectives.md, spec.md, clarify.md, validacion.md). Sin .json separados. Patrón: SddIA/norms/features-documentation-pattern.md.

## Propósito

El proceso **bug-fix** orquesta el ciclo de vida del bug: triaje, documentación, reproducción, alcance mínimo del fix y trazabilidad en la carpeta de persistencia del fix (paths.fixPath/<nombre_fix>).

## Alcance

- **Rama:** fix/<nombre_fix> (nunca master).
- **Documentación:** Carpeta paths.fixPath/<nombre_fix>/ con objectives.md, spec.md, clarify.md si aplica, implementation.md, validacion.md (todos con frontmatter YAML + Markdown). Sin .json separados.
- **Arsenal táctico Git (S+):** git-workspace-recon, git-branch-manager, git-save-snapshot, git-sync-remote, git-tactical-retreat, git-create-pr (paths.skillCapsules; envelope JSON según SddIA/norms/capsule-json-io.md). Complemento: documentation, filesystem-ops, dotnet-development.
- **Restricciones:** Alcance mínimo (solo causa raíz); no refactorizar ni ampliar funcionalidad en la misma rama.

## Fases (resumen)

| Fase | Nombre | Descripción |
| :--- | :--- | :--- |
| **0** | Preparar entorno | **git-workspace-recon**; aislar contexto con **git-branch-manager** en fix/&lt;nombre_fix&gt;. |
| **1**–**4** | Documentación e implementación (doc) | objectives, spec, clarify, implementation según related_actions. |
| **5** | Ejecución | Código + **git-save-snapshot**; emergencia: **git-tactical-retreat**. |
| **6** | Validar | validate → validacion.md. |
| **7** | Finalizar | **git-sync-remote** → **git-create-pr** (cuerpo del PR con objetivos y validación). |

## Integración

El agente Bug Fix Specialist (definición en paths.processPath/bug-fix/) orquesta el ciclo. En la descripción del PR y en Evolution Logs, la referencia canónica es **paths.fixPath/<nombre_fix>/** (Cúmulo). SSOT para ese fix.
