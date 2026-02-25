# Objetivos: Herramienta postman-mcp-validation (create-tool)

**Tarea:** create-tool-postman-mcp-validation  
**Proceso:** paths.processPath/create-tool/  
**Rutas Cúmulo:** paths.featurePath, paths.toolsDefinitionPath, paths.toolCapsules, paths.toolsIndexPath.

---

## Objetivo principal

Incorporar una **herramienta de seguridad externa** `postman-mcp-validation` al ecosistema SddIA que realice **validaciones sobre los endpoints** del proyecto (GesFer.Admin.Back) mediante la colección Postman existente, con diseño preparado para **interacción con Postman MCP**. La ejecución se realiza siempre a través de esta herramienta, cumpliendo el contrato de herramientas y las normas commands-via-skills-or-tools, sin que el agente invoque comandos de sistema directamente.

## Objetivos específicos

1. **Definición en SddIA**  
   - Crear paths.toolsDefinitionPath/postman-mcp-validation/ con spec.md y spec.json.  
   - Incluir implementation_path_ref a la cápsula, inputs (CollectionPath, BaseUrl, InternalSecret, EnvironmentPath opcional, OutputPath, OutputJson), descripción del resultado (resumen de ejecución Newman, assertions, fallos) y fases de feedback alineadas al contrato.  
   - Documentar en spec el **diseño MCP-ready**: nombre de capacidad, argumentos y formato de resultado para una futura exposición vía Postman MCP o servidor MCP propio.

2. **Cápsula (implementación)**  
   - Crear paths.toolCapsules.postman-mcp-validation (según Cúmulo) con: manifest.json, postman-mcp-validation-config.json, documentación .md, launcher .bat y script .ps1 de fallback que:
     - Resuelva la ruta de la colección (por defecto docs/postman/GesFer.Admin.Back.API.postman_collection.json o la indicada en config).
     - Invoque Newman (o equivalente) con variables baseUrl, internalSecret (y adminToken si aplica) para ejecutar la colección.
     - Capture la salida (reporter JSON de Newman) y la transforme al formato de salida del tools-contract (toolId, exitCode, success, timestamp, message, feedback, data con resumen de ejecución/assertions/errores, duration_ms).
   - Salida JSON según tools-contract; en caso de fallo de conexión o tests fallidos, success false y feedback con nivel error.

3. **Índice y Cúmulo**  
   - Registrar postman-mcp-validation en paths.toolsIndexPath (scripts/tools/index.json).  
   - Actualizar paths.toolCapsules en SddIA/agents/cumulo.paths.json con la entrada postman-mcp-validation.

4. **Validación con el caso de uso**  
   - Poner a prueba la herramienta: ejecutar la tool con la colección del proyecto (API en marcha o documentar prerequisito) y verificar que el resultado JSON y el exitCode reflejen el resultado real de la ejecución Newman (éxito/fallos), sin que el agente ejecute newman ni comandos de red directamente.

5. **Documentación de la tarea**  
   - Mantener en paths.featurePath/create-tool-postman-mcp-validation/ los artefactos del proceso: analysis.md, objectives.md, objectives.json y, en fases siguientes, spec.md, spec.json, implementation.md, implementation.json, validacion.json según create-tool.

## Criterios de éxito

- La herramienta postman-mcp-validation existe en paths.toolsDefinitionPath y en paths.toolCapsules.  
- Invocar la herramienta ejecuta la colección Postman (vía Newman o wrapper) y produce un JSON de resultado conforme al contrato.  
- El agente (o cualquier consumidor) puede usar postman-mcp-validation como punto de ejecución de validaciones de endpoints, sin invocar newman ni comandos de sistema directamente.  
- La especificación deja documentado el diseño MCP-ready para futura interacción con Postman MCP.  
- El caso de uso "validación de endpoints con colección del proyecto" se verifica ejecutando la tool y comprobando salida y códigos de salida.

## Alcance fuera de esta tarea

- Implementación en Rust (paths.toolsRustPath) puede quedar para una fase posterior; el fallback .ps1 en la cápsula (invocando Newman) es suficiente para cumplir el contrato y el caso de uso.  
- Implementación real de un servidor MCP propio o cliente hacia el MCP de Postman (exponer "validar endpoints" como Tool MCP) queda como evolución posterior; en esta tarea solo se documenta el diseño (nombre, argumentos, resultado) para esa integración.

---

*Objetivos alineados con SddIA/process/create-tool y SddIA/tools/tools-contract.json.*
