# Especificación: postman-mcp-validation

**toolId:** `postman-mcp-validation`  
**Definición (SddIA):** Este directorio (paths.toolsDefinitionPath/postman-mcp-validation/).  
**Implementación (scripts):** Ruta canónica en Cúmulo → **implementation_path_ref:** `paths.toolCapsules.postman-mcp-validation` (consultar `SddIA/agents/cumulo.paths.json`).

## Objetivo

Herramienta de **seguridad externa** que realiza **validaciones sobre los endpoints** del proyecto (GesFer.Admin.Back) ejecutando la colección Postman existente mediante Newman (CLI). Salida JSON y feedback conforme a tools-contract. Diseño **MCP-ready** para futura interacción con Postman MCP (Model Context Protocol).

## Entradas

| Parámetro | Tipo | Descripción |
|-----------|------|-------------|
| CollectionPath | string | Ruta al JSON de la colección Postman. Por defecto: docs/postman/GesFer.Admin.Back.API.postman_collection.json (relativa al repo). |
| BaseUrl | string | URL base de la API (p. ej. http://localhost:5010). Sustituye la variable {{baseUrl}} de la colección. |
| InternalSecret | string | Secreto interno para endpoints que usan X-Internal-Secret. Sustituye {{internalSecret}}. |
| EnvironmentPath | string | (Opcional) Ruta a fichero de entorno Postman (.json). Si no se indica, se usan variables inline. |
| OutputPath | string | (Opcional) Fichero donde escribir el resultado JSON. |
| OutputJson | switch | (Opcional) Emitir resultado JSON por stdout. |

## Salida

Cumple `SddIA/tools/tools-contract.json`: toolId, exitCode, success, timestamp, message, feedback[], data (run_summary con executed/passed/failed, assertions si aplica, duration_ms).

## Fases (feedback)

init → newman (ejecución de la colección) → done (o error).

## Implementación

La implementación (manifest, config, .bat, .ps1) reside en la carpeta indicada por Cúmulo en **implementation_path_ref**. La cápsula invocará Newman sobre la colección con las variables indicadas y transformará la salida de Newman al formato del contrato.

## Diseño MCP-ready (futura integración con Postman MCP)

Para una fase posterior en la que esta validación se exponga como capacidad invocable por agentes vía MCP:

- **Nombre de capacidad sugerido (MCP Tool):** `run_endpoint_validation` o `postman_validate_endpoints`.
- **Argumentos sugeridos:** `collection_path` (string), `base_url` (string), `internal_secret` (string, opcional), `environment_path` (string, opcional).
- **Formato de resultado:** El mismo que la salida JSON de esta tool (toolId, exitCode, success, timestamp, message, feedback, data con run_summary y duration_ms). Así un servidor MCP puede delegar en la ejecución de esta herramienta y devolver el mismo contrato a los clientes MCP.

Referencia: Postman MCP (Model Context Protocol) — Tools, Resources, Prompts; transporte STDIO o HTTP.
