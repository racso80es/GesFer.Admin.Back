---
constraints:
- action_id en kebab-case.
- Rutas canónicas solo desde Cúmulo (paths.actionsPath, paths.processPath, paths.featurePath, paths.fixPath, paths.principlesPath, paths.patternsPath).
- 'Acciones de diseño/implementación: validar principios según principles-contract.'
- 'Planning, implementation, execution: aplicar patrones según patterns-in-planning-implementation-execution.md.'
- Cumplimiento obligatorio de Karma2Token para trazabilidad y seguridad.
- 'Invariante de ejecución: una acción no invoca el sistema operativo, intérpretes ni scripts (.ps1, .bat, .sh, etc.) directamente; solo orquesta skills (paths.skillCapsules) y tools (paths.toolCapsules) registradas según sus contratos y capsule-json-io.'
consumers:
- paths.processPath
- SddIA/agents/*.json
- SddIA/norms/interaction-triggers.md
- .cursor/rules
contract_version: 1.0.0
definition_artefacts:
- ext: .md
  format: frontmatter_yaml
  naming: spec.md
  path: paths.actionsPath/<action-id>/
  purpose: 'Especificación: frontmatter YAML (metadatos) + cuerpo Markdown. Campos: action_id, name, purpose, inputs, outputs, flow_steps, contract_ref. es-ES.'
description: 'Contrato que toda acción del ciclo debe cumplir: definición en paths.actionsPath/<action-id>/ con archivo .md con frontmatter YAML. Entrada/salida y flujo documentados.'
patterns_validation: 'Las acciones planning, implementation y execution deben consultar paths.patternsPath para referenciar patrones en el PLAN, en ítems IMPL (pattern_id) y en ejecución. Norma: SddIA/norms/patterns-in-planning-implementation-execution.md.'
principles_validation: 'Las acciones que afecten diseño o implementación (spec, planning, implementation, execution) deben validar coherencia con paths.principlesPath (principles-contract.json). En spec.json puede declararse principles_applicable (array de principle_id) o principles_ref: paths.principlesPath.'
scope: paths.actionsPath (SddIA/actions/)
security_model:
  description: Toda acción debe ser invocada bajo el contexto de un Karma2Token válido.
  required_token: Karma2Token
  token_ref: SddIA/tokens/karma2-token/spec.json
---

# Contrato: Acciones del ciclo (actions)

**Alcance:** paths.actionsPath (SddIA/actions/). Toda acción del ciclo debe cumplir este contrato.

## Definición por acción

Cada acción tiene una **carpeta** en paths.actionsPath con identificador `<action-id>` (kebab-case). Dentro de la carpeta:

| Artefacto | Propósito |
|-----------|-----------|
| **spec.md** | Especificación legible: frontmatter YAML (action_id, inputs, outputs, flow_steps, etc.) + cuerpo Markdown. es-ES. Sin spec.json separado. |

## Restricciones

- action_id en kebab-case (spec, clarify, planning, implementation, execution, validate, finalize-process, sddia-difusion).
- Salida de acciones en carpeta de tarea (paths.featurePath, paths.fixPath): un .md por acción con frontmatter YAML + Markdown. Sin .json separados. Patrón: SddIA/norms/features-documentation-pattern.md.
- Rutas solo vía Cúmulo.

## Invariantes de ejecución (innegociable)

1. **Sin ejecución directa del SO:** Ninguna acción puede, en su definición operativa o en el flujo documentado en `spec.md`, prescribir la invocación directa de comandos del sistema operacional, intérpretes (`pwsh`, `cmd`, `bash`, …) ni ficheros script (`.ps1`, `.bat`, `.sh`, …) como mecanismo de implementación.
2. **Solo orquestación registrada:** El único mecanismo permitido es **orquestar** entidades ya registradas: **skills** (`paths.skillCapsules`, definición en `paths.skillsDefinitionPath`) y **tools** (`paths.toolCapsules`, definición en `paths.toolsDefinitionPath`), con envelope **capsule-json-io** cuando la cápsula lo exija.
3. **Separación de responsabilidades:** Quien ejecuta binarios encapsulados es la **skill** o la **tool**; la **acción** describe *qué* encadenar y bajo *qué* precondiciones, no *cómo* lanzar procesos ajenos al contrato.

## Consumidores

paths.processPath, SddIA/agents, SddIA/norms (interaction-triggers), .cursor/rules.
