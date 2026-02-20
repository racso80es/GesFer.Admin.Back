# Proceso: Creación de herramientas (create-tool)

Este documento define el **proceso de tarea** para crear una nueva herramienta (tool) en el proyecto. Está ubicado en paths.processPath/create-tool.md (Cúmulo). Las rutas de herramientas se obtienen de **Cúmulo** (paths.toolsPath, paths.toolCapsules, paths.toolsIndexPath).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): la tarea de creación genera en **{persist}** (documentación de la tarea) al menos un **`.md`** (objectives.md, spec.md, implementation.md) y al menos un **`.json`** (spec.json, implementation.json, validacion.json). El **resultado ejecutable** es la cápsula en **paths.toolCapsules[&lt;tool-id&gt;]** con todos los artefactos requeridos por el contrato de herramientas.

## Propósito

El proceso **create-tool** define el procedimiento para incorporar una nueva herramienta al ecosistema de paths.toolsPath (Cúmulo): desde la definición del objetivo hasta la cápsula lista, el índice actualizado y Cúmulo sincronizado. Garantiza que cada herramienta cumpla `SddIA/tools/tools-contract.json` y que las rutas queden registradas en Cúmulo y en `scripts/tools/index.json`.

## Alcance del procedimiento

- **{persist}** = documentación de la tarea = **paths.featurePath**/create-tool-&lt;tool-id&gt;/ (opcional pero recomendado para trazabilidad).
- **Definición (SddIA):** **paths.toolsDefinitionPath**/&lt;tool-id&gt;/ = SddIA/tools/&lt;tool-id&gt;/ con spec.md y spec.json (implementation_path_ref obligatorio; raíz de implementación vía Cúmulo).
- **Cápsula (implementación):** **paths.toolsPath**/&lt;tool-id&gt;/ = **paths.toolCapsules[&lt;tool-id&gt;]** (tras actualizar Cúmulo).

| Fase | Nombre | Descripción |
| :--- | :--- | :--- |
| **0** | Preparar entorno | Rama feat/create-tool-&lt;tool-id&gt; desde main/master. Skill: `iniciar-rama` con BranchName create-tool-&lt;tool-id&gt;. |
| **1** | Objetivos y especificación | En {persist}/: objectives.md (objetivo de la herramienta, alcance, contrato). spec.md / spec.json (entrada/salida, fases, parámetros, formato JSON de salida según tools-contract). |
| **1b** | Definición en SddIA | Crear **paths.toolsDefinitionPath**/&lt;tool-id&gt;/ con spec.md y spec.json. En spec.json incluir **implementation_path_ref**: `paths.toolCapsules.&lt;tool-id&gt;` (ruta de implementación se resuelve desde Cúmulo; no usar ruta literal). |
| **2** | Estructura de la cápsula | Crear **paths.toolsPath**/&lt;tool-id&gt;/ con: manifest.json, &lt;ToolName&gt;.ps1, &lt;ToolName&gt;.bat, &lt;tool-id&gt;-config.json (o equivalente), &lt;tool-id&gt;.md, directorio bin/ (.gitkeep si no hay exe aún). |
| **3** | Manifest y documentación | manifest.json con toolId, version, description, contract_ref, components (launcher_bat, launcher_ps1, config, doc, bin), env. Documentación .md en la cápsula: uso, parámetros, configuración, salida JSON. |
| **4** | Scripts PowerShell y launcher | .ps1: lógica de la herramienta, salida JSON según contrato (toolId, exitCode, success, timestamp, message, feedback[], data, duration_ms). .bat en la cápsula: invocar bin/&lt;tool_bin&gt;.exe si existe, si no el .ps1. |
| **5** | Wrapper e índice | En **paths.toolsPath**: wrapper .bat que delegue a la cápsula (call %SCRIPT_DIR%&lt;tool-id&gt;\&lt;ToolName&gt;.bat). Actualizar paths.toolsIndexPath (Cúmulo): añadir entrada en array tools (toolId, path, manifest, wrapper_bat, description). |
| **6** | Actualizar Cúmulo | En Cúmulo (SddIA/agents/cumulo.json), añadir en paths.toolCapsules la entrada "&lt;tool-id&gt;": "./scripts/tools/&lt;tool-id&gt;/". |
| **7** | Opcional: implementación Rust | Si aplica: paths.toolsRustPath + src/bin/&lt;tool_bin&gt;.rs; actualizar install.ps1 para copiar el .exe a la cápsula bin/. |
| **8** | Validación | Comprobar que la herramienta cumple tools-contract.json (salida JSON, feedback, artefactos). Generar {persist}/validacion.json si existe {persist}. |
| **9** | Cierre | PR, Evolution Logs, referencia a {persist}/ o a la cápsula en docs. |

## Artefactos obligatorios por herramienta

**Definición (SddIA, paths.toolsDefinitionPath/&lt;tool-id&gt;/):**

| Artefacto | Descripción |
| :--- | :--- |
| **spec.md** | Especificación legible: objetivo, entradas, salidas, fases. |
| **spec.json** | Especificación machine-readable; debe incluir **implementation_path_ref** (ej. paths.toolCapsules.&lt;tool-id&gt;) para que la ruta de implementación se resuelva desde Cúmulo. |

**Implementación (cápsula, paths.toolCapsules[&lt;tool-id&gt;]):** Según `SddIA/tools/tools-contract.json` y Cúmulo, cada cápsula debe contener:

| Artefacto | Ubicación | Descripción |
| :--- | :--- | :--- |
| **manifest.json** | cápsula/ | toolId, version, description, contract_ref, components, env. |
| **&lt;ToolName&gt;.ps1** | cápsula/ | Fallback PowerShell; salida JSON según contrato. |
| **&lt;ToolName&gt;.bat** | cápsula/ | Launcher: binario en bin/ si existe, si no .ps1. |
| **Config .json** | cápsula/ | Configuración machine-readable (nombre según convención, ej. &lt;tool-id&gt;-config.json). |
| **&lt;tool-id&gt;.md** | cápsula/ | Documentación en es-ES: uso, parámetros, configuración, salida. |
| **bin/** | cápsula/bin/ | Opcional: ejecutable Rust copiado por tools-rs/install.ps1. |
| **Wrapper .bat** | paths.toolsPath/ | Opcional pero recomendado: delega a cápsula. |

Tras crear la herramienta, paths.toolsIndexPath debe incluir la nueva entrada en `tools` y **Cúmulo** paths.toolCapsules debe incluir el nuevo tool-id.

## Restricciones

- **toolId** en kebab-case. Sin commits en main; rama feat/create-tool-&lt;tool-id&gt;.
- **Entorno:** Windows 11, PowerShell 7+ (Ley 2 AGENTS.md).
- **Contrato:** Salida JSON con toolId, exitCode, success, timestamp, message, feedback[]; opcional data, duration_ms. No incluir datos sensibles en feedback ni message.

## Referencias

- Contrato: `SddIA/tools/tools-contract.json`, `SddIA/tools/tools-contract.md`.
- Cúmulo: paths.toolsDefinitionPath, paths.toolsPath, paths.toolCapsules, paths.toolsIndexPath.
- Definición por herramienta: **paths.toolsDefinitionPath**/&lt;tool-id&gt;/ (spec.md, spec.json con implementation_path_ref).
- Índice: **paths.toolsIndexPath** (listado de herramientas).
- Proceso machine-readable: `SddIA/process/create-tool.json`.
