# Spec de la tarea: create-tool-postman-mcp-validation

**Definición canónica:** paths.toolsDefinitionPath/postman-mcp-validation/ (SddIA/tools/postman-mcp-validation/spec.md, spec.json).

## Resumen

- **toolId:** postman-mcp-validation  
- **Objetivo:** Herramienta de seguridad externa que valida endpoints ejecutando la colección Postman (Newman). Salida según tools-contract; diseño MCP-ready.
- **Entradas:** CollectionPath, BaseUrl, InternalSecret, EnvironmentPath (opc.), OutputPath, OutputJson.
- **Fases feedback:** init → newman → done | error.
- **Implementación:** paths.toolCapsules.postman-mcp-validation (cápsula a crear en scripts/tools/postman-mcp-validation/).

Ver especificación completa en SddIA/tools/postman-mcp-validation/.
