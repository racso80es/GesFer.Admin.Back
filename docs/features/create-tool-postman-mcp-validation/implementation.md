# Implementación: create-tool-postman-mcp-validation

**Tarea:** create-tool-postman-mcp-validation  
**Ruta (Cúmulo):** paths.featurePath/create-tool-postman-mcp-validation/  
**Fase:** implementation (documento de touchpoints y ejecución aplicada)

---

## Resumen

Se implementa la cápsula de la tool **postman-mcp-validation** según plan (fases 1–4) y se deja documentada la validación (fase 5) para ejecución por el usuario.

## Touchpoints (archivos creados o modificados)

| Archivo | Acción | Descripción |
|---------|--------|-------------|
| **scripts/tools/postman-mcp-validation/manifest.json** | Crear | Manifest de la cápsula: toolId, contract_ref, components. |
| **scripts/tools/postman-mcp-validation/postman-mcp-validation-config.json** | Crear | Config por defecto: collectionPath, baseUrl, internalSecret (o ref env). |
| **scripts/tools/postman-mcp-validation/postman-mcp-validation.md** | Crear | Documentación de uso, parámetros, prerequisitos (Newman/Node). |
| **scripts/tools/postman-mcp-validation/Postman-Mcp-Validation.bat** | Crear | Launcher: repo root, bin/postman_mcp_validation.exe si existe, si no .ps1. |
| **scripts/tools/postman-mcp-validation/Postman-Mcp-Validation.ps1** | Crear | Script que invoca Newman, captura salida, construye JSON tools-contract (feedback init/newman/done|error, data.run_summary, duration_ms). |
| **scripts/tools/index.json** | Modificar | Añadir entrada tool postman-mcp-validation (path, manifest, description). |
| **SddIA/agents/cumulo.paths.json** | Modificar | Añadir en toolCapsules: "postman-mcp-validation": "./scripts/tools/postman-mcp-validation/". |

## Detalle del script .ps1

- **Parámetros:** CollectionPath, BaseUrl, InternalSecret, EnvironmentPath, OutputPath, OutputJson (spec + clarify).
- **Repo root:** Resolver desde $PSScriptRoot (cápsula) como ..\..\..; working directory = repo root.
- **Config:** Cargar postman-mcp-validation-config.json; aplicar parámetros sobre valores por defecto; InternalSecret desde env POSTMAN_INTERNAL_SECRET si no en config ni parámetro.
- **Newman:** Comprobar `Get-Command newman` o `npx newman`; invocar `newman run <collection> --global-var baseUrl=... --global-var internalSecret=... -r json --reporter-json-export <temp>`; working directory = repo root.
- **Parsing:** Leer JSON de Newman (run.stats o estructura equivalente); extraer executed, passed, failed para run_summary; si no hay JSON (fallo previo), run_summary desde exit code (failed=1 si exitCode != 0).
- **Resultado:** toolId, exitCode (del newman o 1 si error de entorno), success, timestamp, message, feedback[], data.run_summary, data.duration_ms. Escribir OutputPath y/o OutputJson.

## Validación (fase 5)

- Ejecutar desde raíz: `.\scripts\tools\postman-mcp-validation\Postman-Mcp-Validation.bat` (o .ps1 con parámetros).
- Verificar JSON con toolId postman-mcp-validation, feedback con fases, data.run_summary.
- Si la API no está levantada: success false, feedback con level error (conexión rechazada/timeout).

## Referencias

- Plan: plan.md, plan.json
- Spec: SddIA/tools/postman-mcp-validation/
- Contrato: SddIA/tools/tools-contract.json
