# Procesos de tarea (SddIA/process)

Al iniciar una tarea se debe elegir un **proceso**. Cada proceso define el ciclo completo (rama, documentación, especificación, implementación, validación y cierre) y la ubicación de persistencia según el agente **Cúmulo** (paths; ver paths.processPath).

Todo proceso cumple el **contrato** en [process-contract.json](./process-contract.json) y [process-contract.md](./process-contract.md): definición en carpeta `<process-id>/` con spec.md y spec.json.

## Procesos disponibles

| Proceso      | Descripción                                                                 | Definición                                      | Agente principal        |
| :---         | :---                                                                        | :---                                             | :---                    |
| **feature**  | Desarrollo de una funcionalidad: rama `feat/<nombre_feature>`, documentación en paths.featurePath/<nombre_feature>/ (Cúmulo). | [feature/](./feature/) (spec.md, spec.json)     | Arquitecto, Tekton      |
| **bug-fix**  | Corrección de un bug: rama `fix/<nombre_fix>`, documentación en paths.fixPath/<nombre_fix>/ (Cúmulo). Alcance mínimo. | [bug-fix/](./bug-fix/)                           | Bug Fix Specialist      |
| **refactorization** | Refactorización: rama `feat/refactorization-<nombre_refactor>`, documentación en paths.featurePath/refactorization-&lt;nombre_refactor&gt;/ (Cúmulo). | [refactorization/](./refactorization/)           | Arquitecto, Tekton      |
| **create-tool** | Creación de herramienta: rama `feat/create-tool-<tool-id>`, cápsula en paths.toolCapsules. | [create-tool/](./create-tool/)                   | Tekton, Arquitecto      |
| **create-pattern** | Creación de patrón de diseño: carpeta en paths.patternsPath con spec.md y spec.json. | [create-pattern.json](./create-pattern.json)     | Arquitecto              |
| **create-principle** | Creación de principio técnico: carpeta en paths.principlesPath con spec.md y spec.json. Principios con blocking_for_pr bloquean PR si falla validate. | [create-principle.json](./create-principle.json) | Cúmulo, Arquitecto      |

## Uso

1. **Feature:** paths.processPath/feature/. Ruta: Cúmulo (paths.featurePath/<nombre_feature>).
2. **Bug-fix:** paths.processPath/bug-fix/. Ruta: Cúmulo (paths.fixPath/<nombre_fix>).
3. **Refactorization:** paths.processPath/refactorization/. Ruta: Cúmulo (paths.featurePath/refactorization-&lt;nombre_refactor&gt;).
4. **Create-tool:** paths.processPath/create-tool/. Ruta: Cúmulo (paths.featurePath/create-tool-&lt;tool-id&gt;). Entregable: cápsula en paths.toolsPath/&lt;tool-id&gt;/.

Las **acciones** (spec, clarify, plan, implementation, execution, validate, finalize) siguen en paths.actionsPath (Cúmulo) y son invocadas por los procesos.

## Interfaz de procesos (norma para agentes)

Todo proceso debe cumplir la **interfaz** definida en Cúmulo (Cúmulo → process_interface): solicitar o generar en la carpeta de la tarea (Cúmulo) artefactos con las extensiones:

| Extensión | Uso |
| :--- | :--- |
| **`.md`** | Al menos un fichero `{nombre}.md` por tarea (objectives.md, spec.md, clarify.md, plan, etc.). |
| **`.json`** | Al menos un fichero `{nombre}.json` por tarea (spec.json, clarify.json, implementation.json, validacion.json, etc.). |

Los agentes de proceso y los procedimientos (paths.processPath/<process-id>/) documentan qué artefactos requieren o producen en la carpeta de la tarea.
