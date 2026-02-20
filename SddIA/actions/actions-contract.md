# Contrato: Acciones del ciclo (actions)

**Alcance:** paths.actionsPath (SddIA/actions/). Toda acción del ciclo debe cumplir este contrato.

## Definición por acción

Cada acción tiene una **carpeta** en paths.actionsPath con identificador `<action-id>` (kebab-case). Dentro de la carpeta:

| Artefacto | Propósito |
|-----------|-----------|
| **spec.md** | Especificación legible: propósito, entradas, salidas, flujo de ejecución. es-ES. |
| **spec.json** | Metadatos machine-readable: action_id, name, purpose, inputs, outputs, flow_steps, contract_ref, related_processes. |

## Restricciones

- action_id en kebab-case (spec, clarify, planning, implementation, execution, validate, finalize, sddia-difusion).
- Rutas solo vía Cúmulo.

## Consumidores

paths.processPath, SddIA/agents, SddIA/norms (interaction-triggers), .cursor/rules.
