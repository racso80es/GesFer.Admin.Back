# Contrato: Procesos de tarea (process)

**Alcance:** paths.processPath (SddIA/process/). Todo proceso de tarea debe cumplir este contrato.

## Definición por proceso

Cada proceso tiene una **carpeta** en paths.processPath con identificador `<process-id>` (kebab-case). Dentro de la carpeta:

| Artefacto | Propósito |
|-----------|-----------|
| **spec.md** | Especificación legible: propósito, fases, alcance, ruta de persistencia (persist_ref). Idioma: es-ES. |
| **spec.json** | Metadatos machine-readable: process_id, name, description, phases, persist_ref (paths.featurePath o paths.fixPath), related_actions (paths.actionsPath), contract_ref. |

## Interfaz de proceso (Cúmulo)

La **interfaz de proceso** (qué genera un proceso en la carpeta de la tarea) está definida en Cúmulo (cumulo.json → process_interface): al menos un `.md` y un `.json` en la carpeta de la tarea (paths.featurePath/<nombre_feature>/ o paths.fixPath/<nombre_fix>/). Este contrato define la **forma de la definición** del proceso en SddIA, no sustituye la process_interface.

## Restricciones

- process_id en kebab-case.
- Rutas solo vía Cúmulo; no literales en la definición.
- Un proceso sin spec.md y spec.json en paths.processPath/<process-id>/ no se considera completo.

## Consumidores

paths.actionsPath, SddIA/agents, SddIA/norms (interaction-triggers), .cursor/rules.
