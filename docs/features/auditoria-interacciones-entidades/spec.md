# SPEC: Auditoría de interacciones entre entidades

**Feature:** auditoria-interacciones-entidades  
**Ruta (Cúmulo):** paths.featurePath/auditoria-interacciones-entidades/

## Contexto

**Entidades de modelo:** Se consideran entidades de modelo del ecosistema SddIA aquellas que **implementan el contrato de Token** (paths.tokensPath, tokens-contract.json). Todas han de cumplir dicho contrato para ser auditables. Son instancias de entidad de modelo, entre otras: skill, tool, action, process (cada una definida en su path canónico y consumidora del contrato de Token).

## Propósito

Incorporar al **agente auditor** en los elementos del sistema mediante cambios al **contrato de Token**, de modo que las interacciones entre **entidades de modelo** sean auditable: información sobre **veces utilizada una entidad** (p. ej. herramienta, skill, acción, proceso) **y por quién**.

**Responsabilidad documental:** SddIA ha de contabilizar esta información y, **antes de cada commit**, generar un **fichero documental** con ella en **formato JSON y en formato MD** (mismo nombre base, p. ej. `INTERACCIONES_YYYY-MM-DD_HHmm.json` y `INTERACCIONES_YYYY-MM-DD_HHmm.md`), en paths.auditsPath (Cúmulo).

## Cambios aplicados al contrato de Token

1. **tokens-contract.json**
   - **consumers:** Inclusión explícita de SddIA/agents/auditor/auditor.json (unificado) y SddIA/agents/auditor/process-interaction.json.
   - **interaction_audit:** Nuevo bloque con purpose, required_fields (entity_type, entity_id, invoked_by, timestamp), entity_type como identificación de entidad de modelo (valores según implementaciones que cumplen contrato de Token; ej. skill, tool, action, process), interaction_log_ref (paths.auditsPath), auditor_consumer (el auditor es consumidor formal y puede consultar/agregar interacciones).
   - **constraints:** Los tokens que soporten auditoría de interacciones deben incluir los campos interaction_audit según el contrato. Toda entidad de modelo ha de implementar contrato de Token.

2. **karma2-token/spec.json**
   - **token_definition.fields.interaction_audit:** entity_type (tipo de entidad de modelo), entity_id, invoked_by, timestamp (mapeados desde identity y context).
   - **description / purpose:** Mención a auditoría de interacciones entre entidades de modelo y al auditor.
   - **validation_rules:** Regla sobre presencia de campos de interacción y consumo por el auditor.

## Auditor como consumidor

El agente auditor (unificado y process-interaction) queda integrado como consumidor del contrato de Token y debe poder:
- Consultar registros de tokens en paths.auditsPath (o accessLogFile / subcarpeta interactions/).
- Agregar datos de interacciones: quién (invoked_by), qué entidad de modelo (entity_type, entity_id), cuándo (timestamp).
- **Antes de cada commit:** generar en paths.auditsPath un fichero documental con la información contabilizada, en **JSON y MD** (mismo nombre base).

**Fuente de datos para el generador:** Ver contrato de lectura en esta carpeta: [data-source-contract.md](./data-source-contract.md). Rutas de lectura: paths.auditsPath (o `docs/audits/`), y opcionalmente `docs/diagnostics/{branch}/execution_history.json` (mapeo definido en el contrato).

Ref: SddIA/tokens/tokens-contract.json → interaction_audit.auditor_consumer.
