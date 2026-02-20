# Procesos de tarea (SddIA/process)

Al iniciar una tarea se debe elegir un **proceso**. Cada proceso define el ciclo completo (rama, documentación, especificación, implementación, validación y cierre) y la ubicación de persistencia (`{persist}`) según el agente **Cúmulo** (`SddIA/agents/cumulo.json`).

## Procesos disponibles

| Proceso      | Descripción                                                                 | Definición                                      | Agente principal        |
| :---         | :---                                                                        | :---                                             | :---                    |
| **feature**  | Desarrollo de una funcionalidad: rama `feat/<nombre_feature>`, documentación en `docs/features/<nombre_feature>/`. | [feature.md](./feature.md)                       | Arquitecto, Tekton      |
| **bug-fix**  | Corrección de un bug: rama `fix/<nombre_fix>`, documentación en `docs/bugs/<nombre_fix>/`. Alcance mínimo, sin refactor en la misma rama. | [bug-fix-specialist.json](./bug-fix-specialist.json) | Bug Fix Specialist      |
| **create-tool** | Creación de una nueva herramienta: rama `feat/create-tool-<tool-id>`, cápsula en paths.toolCapsules[&lt;tool-id&gt;], índice y Cúmulo actualizados. | [create-tool.md](./create-tool.md), [create-tool.json](./create-tool.json) | Tekton, Arquitecto      |

## Uso

1. **Feature:** Seguir las fases descritas en `SddIA/process/feature.md`. {persist} = Cúmulo.featurePath/<nombre_feature>.
2. **Bug-fix:** Seguir las instrucciones del agente en `SddIA/process/bug-fix-specialist.json`. {persist} = Cúmulo.fixPath/<nombre_fix>.
3. **Create-tool:** Seguir las fases en `SddIA/process/create-tool.md` y artefactos en `create-tool.json`. {persist} = Cúmulo.featurePath/create-tool-&lt;tool-id&gt;. Entregable: cápsula en paths.toolsPath/&lt;tool-id&gt;/; actualizar index.json y Cúmulo paths.toolCapsules.

Las **acciones** (spec, clarify, plan, implementation, execution, validate, finalize) siguen en `SddIA/actions/` y son invocadas por los procesos.

## Interfaz de procesos (norma para agentes)

Todo proceso debe cumplir la **interfaz** definida en Cúmulo (`SddIA/agents/cumulo.json` → `process_interface`): solicitar o generar en `{persist}/` artefactos con las extensiones:

| Extensión | Uso |
| :--- | :--- |
| **`.md`** | Al menos un fichero `{nombre}.md` por tarea (objectives.md, spec.md, clarify.md, plan, etc.). |
| **`.json`** | Al menos un fichero `{nombre}.json` por tarea (spec.json, clarify.json, implementation.json, validacion.json, etc.). |

Los agentes de proceso (p. ej. Bug Fix Specialist) y los procedimientos (p. ej. feature.md) deben documentar qué `{}.md` y `{}.json` requieren o producen.
