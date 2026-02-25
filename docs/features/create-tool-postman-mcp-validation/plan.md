# Plan: create-tool-postman-mcp-validation

**Tarea:** create-tool-postman-mcp-validation  
**Ruta (Cúmulo):** paths.featurePath/create-tool-postman-mcp-validation/  
**Referencias:** spec (SddIA/tools/postman-mcp-validation/), clarify.md, clarify.json

---

## Objetivo del plan

Transformar la especificación y clarificación en una hoja de ruta ejecutable: crear la cápsula de la tool **postman-mcp-validation** en paths.toolCapsules, registrar en índice y Cúmulo, y validar con el caso de uso (ejecución de la colección Postman del proyecto).

## Fases

| Fase | Alcance | Entregable |
|------|---------|------------|
| **1** | Estructura de la cápsula | Directorio scripts/tools/postman-mcp-validation/ con manifest.json, config, doc .md. |
| **2** | Launchers | Run script .bat (binario en bin/ si existe, si no .ps1) y .ps1 que invoque Newman y genere salida tools-contract. |
| **3** | Configuración | postman-mcp-validation-config.json con collectionPath, baseUrl, internalSecret por defecto (rutas relativas al repo). |
| **4** | Índice y Cúmulo | Registrar en scripts/tools/index.json y añadir postman-mcp-validation en SddIA/agents/cumulo.paths.json (toolCapsules). |
| **5** | Validación | Ejecutar la tool con la colección por defecto (API levantada o no) y verificar JSON y exitCode. |

---

## Fase 1: Estructura de la cápsula

**Objetivo:** Crear la carpeta y artefactos obligatorios del contrato (manifest, config, doc).

**Tareas técnicas:**

1. Crear `scripts/tools/postman-mcp-validation/`.
2. Añadir **manifest.json**: toolId, version, description, contract_ref, components (launcher_bat, launcher_ps1, config, doc, bin opcional).
3. Añadir **postman-mcp-validation-config.json**: collectionPath (por defecto docs/postman/GesFer.Admin.Back.API.postman_collection.json), baseUrl (http://localhost:5010), internalSecret (valor por defecto o desde env).
4. Añadir **postman-mcp-validation.md**: uso, parámetros, salida, prerequisito Newman/Node, definición SddIA.

**Criterio de aceptación:** La carpeta existe con manifest, config y doc; manifest referencia tools-contract.

---

## Fase 2: Launchers (.bat y .ps1)

**Objetivo:** Launcher .bat que use .exe en bin/ si existe, si no .ps1; script .ps1 que ejecute Newman y produzca JSON según tools-contract.

**Tareas técnicas:**

1. **Postman-Mcp-Validation.bat:** Misma lógica que Run-Tests-Local.bat: REPO_ROOT desde cápsula (..\..\..), cd a repo root; si existe bin\postman_mcp_validation.exe invocarlo; si no, ejecutar Postman-Mcp-Validation.ps1 con pwsh/powershell.
2. **Postman-Mcp-Validation.ps1:**
   - Parámetros: CollectionPath, BaseUrl, InternalSecret, EnvironmentPath, OutputPath, OutputJson (alineados a spec y clarify).
   - Resolver repo root; cargar config por defecto; aplicar parámetros sobre config.
   - Fase feedback init: comprobar que la colección existe, que newman está disponible (Get-Command newman o npx newman).
   - Invocar: `newman run <collectionPath> --global-var baseUrl=<BaseUrl> --global-var internalSecret=<InternalSecret> -r json [--reporter-json-export <temp>]` (o equivalente). Working directory = repo root.
   - Capturar exit code de newman; si hay reporter JSON, parsear para run_summary (executed, passed, failed); si no, derivar de exit code (0 → passed, else failed).
   - Construir resultado: toolId, exitCode, success, timestamp, message, feedback (init, newman, done/error), data (run_summary, duration_ms). Escribir OutputPath y/o OutputJson.
   - En caso de error (colección no encontrada, newman no encontrado, API no disponible): success false, exitCode != 0, feedback con level error.

**Criterio de aceptación:** Ejecutar .bat o .ps1 produce un JSON válido conforme a tools-contract; feedback incluye al menos init y newman/done o error.

---

## Fase 3: Configuración

**Objetivo:** Valores por defecto y resolución de InternalSecret (config o variable de entorno).

**Tareas técnicas:**

1. Config ya creada en Fase 1; asegurar que internalSecret pueda leerse desde variable de entorno (ej. POSTMAN_INTERNAL_SECRET) si no viene en config (clarify).
2. Documentar en postman-mcp-validation.md el prerequisito Node/Newman y la variable de entorno opcional.

**Criterio de aceptación:** La tool puede ejecutarse sin pasar InternalSecret por línea de comandos si está en config o en env.

---

## Fase 4: Índice y Cúmulo

**Objetivo:** La tool queda registrada en el ecosistema.

**Tareas técnicas:**

1. Editar **scripts/tools/index.json**: añadir entrada en "tools" con toolId postman-mcp-validation, path postman-mcp-validation/, manifest, wrapper_bat (opcional: Postman-Mcp-Validation.bat en raíz de tools si se desea wrapper), description.
2. Editar **SddIA/agents/cumulo.paths.json**: en "toolCapsules" añadir "postman-mcp-validation": "./scripts/tools/postman-mcp-validation/".

**Criterio de aceptación:** index.json y cumulo.paths.json contienen la nueva tool; rutas consistentes.

---

## Fase 5: Validación

**Objetivo:** Caso de uso verificado sin invocar newman directamente desde el agente.

**Tareas técnicas:**

1. Ejecutar la tool desde la raíz del repo (vía .bat de la cápsula o documentación): con CollectionPath por defecto, BaseUrl y InternalSecret por defecto o config.
2. Comprobar: salida JSON con toolId postman-mcp-validation, exitCode coherente (0 si todos los requests pasaron, distinto de 0 si fallo o API no disponible), feedback con fases, data.run_summary presente.
3. Documentar resultado en validacion.json de la feature (opcional).

**Criterio de aceptación:** Al menos una ejecución exitosa de la tool produce JSON válido; fallo controlado (ej. API apagada) produce success false y feedback con error.

---

## Resumen de entregables

| Entregable | Fase |
|------------|------|
| scripts/tools/postman-mcp-validation/ (manifest, config, .md) | 1 |
| Postman-Mcp-Validation.bat, Postman-Mcp-Validation.ps1 | 2 |
| Config y doc de env/internalSecret | 3 |
| index.json y cumulo.paths.json actualizados | 4 |
| Ejecución verificada y validacion.json (opcional) | 5 |

---

## Referencias

- SPEC: SddIA/tools/postman-mcp-validation/spec.md, spec.json
- Clarificación: clarify.md, clarify.json
- Contrato: SddIA/tools/tools-contract.json
- Cúmulo: SddIA/agents/cumulo.paths.json
- Colección: docs/postman/GesFer.Admin.Back.API.postman_collection.json
