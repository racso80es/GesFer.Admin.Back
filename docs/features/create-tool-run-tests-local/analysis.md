# Análisis: Herramienta para ejecutar tests en condiciones locales (SddIA)

**Tarea:** create-tool-run-tests-local  
**Fuente canónica de rutas:** Cúmulo (SddIA/agents/cumulo.json → paths).  
**Proceso de referencia:** paths.processPath/create-tool/ (spec.md, spec.json).

---

## 1. Contexto y motivación

- Los tests (unitarios, integración, E2E) deben ejecutarse **mediante elementos de SddIA**, no invocando comandos de sistema directamente desde el agente (norma SddIA/norms/commands-via-skills-or-tools.md).
- Existe un escenario de **validación en local**: infra en punto de origen (Docker, MySQL), datos y servicios preparados (prepare-full-env, invoke-mysql-seeds), y API opcional; sobre ese estado se deben poder ejecutar tests de forma trazada y con resultado machine-readable.
- Actualmente existe un script **Run-E2ELocal.ps1** en scripts/ que orquesta prepare-full-env, invoke-mysql-seeds, compilación y E2E; no está registrado como herramienta en paths.toolCapsules ni cumple el contrato de herramientas (salida JSON, feedback, manifest, definición en paths.toolsDefinitionPath).

## 2. Opciones consideradas

| Opción | Descripción | Pros | Contras |
|--------|-------------|------|---------|
| **A. Herramienta (tool)** | Nueva tool **run-tests-local** en paths.toolCapsules: orquesta opcionalmente prepare-full-env e invoke-mysql-seeds, compila y ejecuta tests (unit, integration, E2E) según parámetros; salida JSON y feedback según tools-contract. | Una sola invocación trazada; cumple contrato; índice y Cúmulo actualizados; reutilizable por agentes y CI. | Requiere cápsula, manifest, definición en SddIA y posible fallback .ps1. |
| **B. Plantilla (template)** | Nueva plantilla en paths.templatesPath que **define la situación** "validación tests en local": proceso, herramientas a invocar (prepare-full-env, invoke-mysql-seeds, run-tests-local), orden y criterios. | Documenta el escenario y el procedimiento; no ejecuta por sí sola. | No sustituye la ejecución: algo tiene que invocar las herramientas; si run-tests-local no existe, la plantilla referenciaría una tool por crear. |
| **C. Tool + plantilla** | Crear la tool **run-tests-local** y, opcionalmente, una plantilla **test-local-validacion** que referencie esa tool y el flujo (prepare-env → seeds → run-tests-local). | Tool para ejecutar; plantilla para documentar y procedimentar el caso de uso. | Más artefactos que mantener. |

## 3. Recomendación

- **Principal: Opción A (herramienta run-tests-local).** Así la ejecución de tests en estas condiciones se hace siempre a través de una herramienta de SddIA, alineada con commands-via-skills-or-tools y con Cúmulo.
- **Complementario (opcional en esta tarea):** Una plantilla que defina la situación "validación en local" y referencie run-tests-local y las tools de inicialización (prepare-full-env, invoke-mysql-seeds) puede añadirse en una fase posterior o en el mismo create-tool si se documenta como "template relacionado".

## 4. Situación actual relevante

- **paths.toolCapsules:** prepare-full-env, invoke-mysql-seeds (paths según Cúmulo).
- **paths.toolsDefinitionPath:** SddIA/tools/ con definiciones por tool-id (spec.md, spec.json).
- **paths.toolsIndexPath:** scripts/tools/index.json (listado de tools).
- **Script Run-E2ELocal.ps1:** en scripts/; orquesta prepare-full-env, invoke-mysql-seeds, build, comprobación/arranque API, dotnet test E2E; no es una cápsula ni cumple tools-contract (salida JSON, feedback, manifest).
- **Proyectos de test:** GesFer.Admin.Back.UnitTests, GesFer.Admin.Back.IntegrationTests, GesFer.Admin.Back.E2ETests (Category=E2E); solución en paths según Cúmulo (src/GesFer.Admin.Back.sln u otra ruta de solución referenciada en Cúmulo si existe).

## 5. Dependencias y restricciones

- **Contrato:** SddIA/tools/tools-contract.json (salida JSON, feedback, manifest, artefactos por cápsula).
- **Implementación por defecto:** Rust (paths.toolsRustPath); fallback .ps1 en cápsula y launcher .bat.
- **Entorno:** Windows 11, PowerShell 7+; no ejecutar comandos de sistema directamente desde el agente.
- **Token:** Herramienta bajo contexto Karma2Token (tools-contract).
- **Rama y documentación:** feat/create-tool-run-tests-local; documentación en paths.featurePath/create-tool-run-tests-local/.

## 6. Riesgos y mitigación

- **Riesgo:** Duplicar lógica ya existente en Run-E2ELocal.ps1. **Mitigación:** La cápsula de run-tests-local puede reutilizar o invocar la lógica actual (p. ej. script .ps1 en la cápsula) y añadir la capa de contrato (JSON, feedback, manifest).
- **Riesgo:** La solución o rutas de proyectos de test cambien. **Mitigación:** Rutas y proyectos obtenidos de config en la cápsula (manifest o <tool-id>-config.json), con valores por defecto alineados a la estructura actual; Cúmulo como SSOT para paths canónicos.

---

*Análisis alineado con SddIA/process/create-tool y Cúmulo. Próximo paso: objetivos y especificación.*
