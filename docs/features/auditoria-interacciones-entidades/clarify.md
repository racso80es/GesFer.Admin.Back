# Clarificación: Auditoría de interacciones entre entidades

**Feature:** auditoria-interacciones-entidades  
**Ruta (Cúmulo):** paths.featurePath/auditoria-interacciones-entidades/  
**Fase:** Especificación + Clarificación

## Alcance cerrado

- **Entidades de modelo:** Definición general adoptada. Son entidades de modelo las que implementan el contrato de Token (paths.tokensPath); todas han de implementarlo. Instancias: skill, tool, action, process. No se depende de una enumeración fija sino del contrato.
- **Contrato de Token:** interaction_audit con purpose, required_fields, entity_type_definition (tipo de entidad de modelo), entity_type_enum (valores ej.), interaction_log_ref, auditor_consumer. Constraints: tokens con auditoría incluyen interaction_audit; toda entidad de modelo ha de implementar contrato de Token.
- **Salida pre-commit:** Antes de cada commit, generar en paths.auditsPath un fichero documental con la información de interacciones contabilizada, en formato JSON y MD (mismo nombre base, ej. INTERACCIONES_YYYY-MM-DD_HHmm).
- **Auditor:** Responsabilidad de registro de interacciones y generación de ficheros documentales (auditor.json unificado y process-interaction.json) alineada con la definición de entidades de modelo.

## Gaps resueltos

| Gap / Ambigüedad | Resolución |
|------------------|------------|
| Enumeración fija (skills, tools, actions, process) vs definición extensible | Sustituida por **entidades de modelo** (implementan contrato de Token). La enumeración queda como valores de ejemplo en entity_type. |
| Dónde y en qué formato se persiste el resumen pre-commit | paths.auditsPath; formato JSON y MD (mismo nombre base). |
| Quién genera el fichero pre-commit | SddIA / agente auditor (unificado y process-interaction); instrucciones explícitas en ambos. |

## Pendiente / Abierto

- **Implementación técnica del pre-commit:** El cómo (hook pre-commit, invocación desde invoke-command, o script dedicado) que genere los ficheros INTERACCIONES_*.json y .md queda para fase de planificación/implementación.
- **Estructura exacta del JSON/MD:** Campos mínimos (entity_type, entity_id, invoked_by, timestamp, agregaciones); formato de tabla en MD. Puede fijarse en planning o en un anexo del spec.

## Referencias

- SPEC: spec.md, spec.json
- Contrato: SddIA/tokens/tokens-contract.json (interaction_audit)
- Agentes: SddIA/agents/auditor/auditor.json, process-interaction.json
