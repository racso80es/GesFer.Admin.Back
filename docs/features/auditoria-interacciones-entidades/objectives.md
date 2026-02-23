# Objetivos: Auditoría de interacciones entre entidades

**Proceso:** Nuevo proceso de auditoría (integrado con contrato de Token).  
**Ruta (Cúmulo):** paths.featurePath/auditoria-interacciones-entidades/

## Objetivo

Auditar las **interacciones entre entidades** del ecosistema SddIA (skills, tools, actions, process) de forma que se disponga de información sobre, por ejemplo:
- **Uso de una herramienta:** cuántas veces se utilizó y por quién (qué agente o identidad).
- **Uso de una skill:** invocaciones por agente o sesión.
- **Trazabilidad:** qué entidad ejecutó qué y cuándo.

Para ello se incorpora al **agente auditor** en los elementos del sistema, aplicando los cambios al **contrato de Token** (paths.tokensPath, tokens-contract.json) y al Karma2Token (spec.json), de modo que todo token lleve los datos necesarios para que el auditor consulte y agregue interacciones.

## Criterios de éxito

- El contrato de Token exige o recomienda campos de **auditoría de interacciones** (entity_type, entity_id, invoked_by, timestamp) y una referencia al registro que consume el auditor.
- El **auditor** (agente) figura como consumidor formal del contrato de Token y puede agregar/consultar datos de interacciones (p. ej. desde paths.auditsPath o path definido en el contrato).
- Karma2Token incluye en su definición los campos necesarios para registrar “quién invocó qué entidad y cuándo”.

## Entidades afectadas

- **Contrato de Token:** SddIA/tokens/tokens-contract.json.
- **Karma2Token:** SddIA/tokens/karma2-token/spec.json (y opcional spec.md).
- **Agente auditor:** SddIA/agents/auditor/ (process-interaction, front, back); integración como consumidor del token y de los logs de interacción.

## Ley aplicada

- **Entidades de dominio:** Las que integran el contrato Token respetan estructura y sincronidad (SddIA/norms/entidades-dominio-ecosistema-sddia.md).
- **Comandos:** Solo vía skill/herramienta/acción/proceso.
