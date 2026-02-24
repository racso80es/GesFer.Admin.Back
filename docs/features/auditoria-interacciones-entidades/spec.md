# SPEC: Auditoría de interacciones entre entidades

**Feature:** auditoria-interacciones-entidades  
**Ruta (Cúmulo):** paths.featurePath/auditoria-interacciones-entidades/

## Propósito

Incorporar al **agente auditor** en los elementos del sistema mediante cambios al **contrato de Token**, de modo que las interacciones entre entidades (skills, tools, actions, process) sean auditable: información sobre **veces utilizada una herramienta** (o skill/acción/proceso) **y por quién**.

## Cambios aplicados al contrato de Token

1. **tokens-contract.json**
   - **consumers:** Inclusión explícita de SddIA/agents/auditor/auditor.json (unificado) y SddIA/agents/auditor/process-interaction.json.
   - **interaction_audit:** Nuevo bloque con purpose, required_fields (entity_type, entity_id, invoked_by, timestamp), entity_type_enum (skill, tool, action, process), interaction_log_ref (paths.auditsPath), auditor_consumer (el auditor es consumidor formal y puede consultar/agregar interacciones).
   - **constraints:** Los tokens que soporten auditoría de interacciones deben incluir los campos interaction_audit según el contrato.

2. **karma2-token/spec.json**
   - **token_definition.fields.interaction_audit:** entity_type, entity_id, invoked_by, timestamp (mapeados desde identity y context).
   - **description / purpose:** Mención a auditoría de interacciones y al auditor.
   - **validation_rules:** Regla sobre presencia de campos de interacción y consumo por el auditor.

## Auditor como consumidor

El agente auditor (process-interaction, front, back) queda integrado como consumidor del contrato de Token y debe poder:
- Consultar registros de tokens en paths.auditsPath (o accessLogFile / subcarpeta interactions/).
- Agregar datos de interacciones: quién (invoked_by), qué entidad (entity_type, entity_id), cuándo (timestamp).

Ref: SddIA/tokens/tokens-contract.json → interaction_audit.auditor_consumer.
