# Análisis: Herramienta de validación de endpoints con Postman MCP (SddIA)

**Tarea:** create-tool-postman-mcp-validation  
**Fuente canónica de rutas:** Cúmulo (SddIA/agents/cumulo.paths.json).  
**Proceso de referencia:** paths.processPath/create-tool/ (spec.md, spec.json).

---

## 1. Contexto y motivación

- Se requiere una **herramienta de seguridad externa** que realice validaciones sobre el proyecto, en particular sobre los **endpoints** de la API (GesFer.Admin.Back).
- La validación debe ser **trazada y machine-readable**, sin que el agente invoque comandos de sistema directamente (norma SddIA/norms/commands-via-skills-or-tools.md).
- El proyecto dispone ya de una colección Postman en **docs/postman/GesFer.Admin.Back.API.postman_collection.json** (Health, Auth, Logs, etc.) con variables `baseUrl`, `internalSecret`, `adminToken`.
- Objetivo: disponer de una **tool** que permita ejecutar esas validaciones de forma estandarizada, con salida JSON según SddIA/tools/tools-contract.json y, en su caso, **interacción con Postman MCP** para que agentes IA puedan solicitar o consumir validaciones.

---

## 2. Tecnologías indicadas

### 2.1 Postman MCP (Model Context Protocol)

- **Qué es:** Integración de Postman con el [Model Context Protocol](https://modelcontextprotocol.io/introduction). Postman ofrece un **MCP server** que expone capacidades consumibles por agentes IA.
- **Capacidades del servidor MCP:**
  - **Tools:** acciones que el servidor puede ejecutar (p. ej. crear peticiones API, ejecutar colecciones).
  - **Resources:** datos que el servidor puede recuperar (p. ej. documentación de API).
  - **Prompts:** plantillas sugeridas para interactuar con el modelo.
- **Transporte:** STDIO o HTTP (streamable). En Postman se crean "MCP requests" que se conectan al servidor (comando STDIO o URL HTTP).
- **Uso típico:** Un agente IA se conecta al MCP server de Postman, lista Tools/Resources/Prompts, y puede invocar herramientas para crear colecciones o interactuar con flujos de API. Útil para **integración agente ↔ Postman** (validaciones solicitadas por IA, documentación viva, etc.).

**Referencias:** Postman Docs – MCP requests, MCP server, AI requests con MCP; Postman Remote MCP Server (producto).

### 2.2 Newman (CLI de Postman)

- **Qué es:** Ejecutor en línea de comandos de colecciones Postman. Permite **ejecutar** una colección (JSON) sin interfaz gráfica, ideal para CI/CD y para herramientas externas.
- **Comando típico:** `newman run <collection.json>` con opciones para variables (`-e`, `-g`), entorno, retardos, timeouts, carpetas concretas.
- **Salidas:** Múltiples reporters: CLI, **JSON**, JUnit XML, HTML. El reporter JSON permite resultado machine-readable para integrar en nuestra tool y cumplir tools-contract.
- **Ventaja para esta tarea:** La **validación real de endpoints** (HTTP requests + assertions en scripts de la colección) se realiza con Newman; la colección del proyecto ya existe y puede ejecutarse contra `baseUrl` (p. ej. `http://localhost:5010`).

**Conclusión técnica:**  
- **Newman** = ejecución concreta de la colección y validación de endpoints (status codes, tests en la colección).  
- **Postman MCP** = canal para que agentes IA descubran e invoquen capacidades (por ejemplo “ejecutar validación de endpoints” como una Tool del MCP).  
Una tool de “validación de endpoints con Postman MCP” puede combinar: **(1)** ejecución vía Newman de la colección del proyecto y **(2)** diseño preparado para que, en el futuro o en la misma entrega, esa ejecución sea invocable o coordinable vía MCP (p. ej. servidor MCP propio que exponga “run validation” y por debajo llame a la tool/Newman).

---

## 3. Opciones consideradas

| Opción | Descripción | Pros | Contras |
|--------|-------------|------|---------|
| **A. Tool Newman-only** | Tool **postman-mcp-validation** que invoque Newman sobre la colección del proyecto (ruta desde Cúmulo/config), con variables (baseUrl, internalSecret), y produzca salida JSON según tools-contract (feedback, exitCode, data con resumen de ejecución). Sin integración MCP en v1. | Implementación directa; cumple “validación externa de endpoints” y contrato; reutilizable por agentes y CI. | No cubre aún “interacción con Postman MCP” en el sentido de cliente/servidor MCP. |
| **B. Tool + cliente MCP** | Misma tool que A, más un componente que actúe como **cliente MCP** hacia el servidor de Postman (o un servidor MCP propio) para exponer “validar endpoints” como capacidad invocable por agentes. | Alinea con el enunciado “interacción con Postman MCP” y permite que un agente pida validaciones vía MCP. | Mayor complejidad; depende de disponibilidad/estabilidad del MCP de Postman y del flujo STDIO/HTTP. |
| **C. Tool Newman + diseño MCP-ready** | Tool como en A, con **diseño y documentación** explícitos para que la validación sea invocable vía MCP en una fase posterior (p. ej. servidor MCP que exponga una Tool “run-endpoint-validation” y delegue en esta tool). Especificación y spec.md/spec.json dejan claro el contrato de entrada/salida para esa futura integración MCP. | Entrega valor inmediato (validación trazada) y deja claro el camino a MCP sin bloquear en dependencias MCP en v1. | La “interacción con Postman MCP” queda como objetivo de diseño/fase siguiente, no implementada en v1. |

---

## 4. Recomendación

- **Fase actual (create-tool):** Adoptar **Opción C (Tool Newman + diseño MCP-ready)**.  
  - Entregable principal: herramienta **postman-mcp-validation** que ejecuta la colección Postman del proyecto (vía Newman o wrapper que cumpla tools-contract), con configuración (ruta colección, baseUrl, internalSecret, etc.), salida JSON y feedback según tools-contract, y documentación que especifique cómo podría exponerse como capacidad MCP (nombre de tool, argumentos, formato de resultado) para una fase posterior.
- **Opcional en la misma tarea:** Si el equipo prefiere tener en v1 un cliente MCP mínimo que invoque al servidor de Postman para “ejecutar validación”, puede documentarse como objetivo opcional en spec; la recomendación sigue siendo no bloquear la entrega en la estabilidad del MCP externo y priorizar Newman como motor de validación.

---

## 5. Situación actual relevante

- **Colección:** docs/postman/GesFer.Admin.Back.API.postman_collection.json (Health, Auth, Logs, etc.; variables baseUrl, internalSecret, adminToken).
- **paths.toolCapsules:** prepare-full-env, invoke-mysql-seeds, run-tests-local (Cúmulo).
- **paths.toolsDefinitionPath:** SddIA/tools/; **paths.toolsPath:** scripts/tools/; **paths.toolsIndexPath:** scripts/tools/index.json.
- **Contrato:** SddIA/tools/tools-contract.json (salida JSON, feedback, manifest, Karma2Token).
- No existe actualmente una tool registrada que ejecute la colección Postman ni que integre con Postman MCP.

---

## 6. Dependencias y restricciones

- **Contrato:** SddIA/tools/tools-contract.json (salida JSON, feedback, manifest, artefactos por cápsula).
- **Implementación por defecto:** Rust (paths.toolsRustPath); fallback .ps1 en cápsula y launcher .bat.
- **Entorno:** Windows 11, PowerShell 7+; no ejecutar comandos de sistema directamente desde el agente.
- **Token:** Herramienta bajo contexto Karma2Token (tools-contract).
- **Rama y documentación:** feat/create-tool-postman-mcp-validation; documentación en paths.featurePath/create-tool-postman-mcp-validation/.
- **Newman:** Requiere Node.js/npm (newman) instalado donde se ejecute la tool, o documentar como prerequisito; la cápsula puede invocar `newman run ...` desde un script .ps1 como fallback.

---

## 7. Riesgos y mitigación

- **Riesgo:** Dependencia de Newman/Node en el entorno. **Mitigación:** Documentar prerequisitos en la cápsula; fallback .ps1 que ejecute newman; opcionalmente en fase posterior un binario Rust que invoque newman o use una librería HTTP para reproducir los requests (mayor esfuerzo).
- **Riesgo:** La API no esté levantada durante la validación. **Mitigación:** La tool debe reflejar en feedback y en data el estado (p. ej. conexión rechazada, timeouts) y exitCode != 0; opcionalmente documentar que puede orquestarse con prepare-full-env u otra tool que levante la API.
- **Riesgo:** Postman MCP (servidor externo) cambie o no esté disponible. **Mitigación:** Diseño MCP-ready sin acoplar v1 al MCP de Postman; la validación real se basa en Newman + colección local.

---

*Análisis alineado con SddIA/process/create-tool y Cúmulo. Próximo paso: objetivos y especificación.*
