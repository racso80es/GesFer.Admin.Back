# Especificación: create-tool start-api (ejecutable Rust)

**Proceso:** create-tool  
**tool_id:** start-api  
**Contrato:** SddIA/tools/tools-contract.json

## Resumen

La herramienta start-api debe cumplir el estándar de implementación por defecto en Rust. La cápsula ya existe; la implementación Rust ya está en paths.toolsRustPath (scripts/tools-rs/src/bin/start_api.rs). Este proceso documenta el entregable (ejecutable en la cápsula) y los pasos para tener el .exe disponible.

## Definición de la herramienta (SddIA)

- **toolId:** start-api
- **Definición:** SddIA/tools/start-api/ (spec.md, spec.json)
- **implementation_path_ref:** paths.toolCapsules.start-api
- **Entradas:** NoBuild, Profile, Port, PortBlocked (fail|kill), ConfigPath, OutputPath, OutputJson
- **Éxito:** endpoint health responde HTTP 200 en el tiempo configurado
- **Fases feedback:** init → port-check → [port-kill] → build → launch → healthcheck → done | error

## Implementación Rust (existente)

| Elemento | Ruta |
|----------|------|
| Binario | scripts/tools-rs/src/bin/start_api.rs |
| Crate | gesfer-tools (scripts/tools-rs/Cargo.toml) |
| Lib compartida | scripts/tools-rs/src/lib.rs (FeedbackEntry, ToolResult, to_contract_json) |
| Config | CamelCase en JSON → struct Config con snake_case (api_working_dir, default_profile, default_port, health_url, health_check_timeout_seconds) |
| Config path por defecto | start-api-config.json (resuelto desde repo root o scripts/tools/start-api/) |

## Entregable en la cápsula

- **Ejecutable:** start_api.exe en scripts/tools/start-api/ (misma carpeta que Start-Api.bat).
- **Origen:** Copiado desde scripts/tools-rs/target/release/start_api.exe por install.ps1.
- **Launcher:** Start-Api.bat ya implementado: si existe start_api.exe lo invoca; si no, powershell -File Start-Api.ps1.

## Pasos para obtener el ejecutable

1. Asegurar Rust en PATH (por ejemplo .cargo\bin).
2. Desde scripts/tools-rs: ejecutar install.ps1 (o equivalente vía skill/herramienta). Esto ejecuta `cargo build --release` y copia start_api.exe a scripts/tools/start-api/.
3. Opcional: validar ejecutando Start-Api.bat o start_api.exe con --output-json.

## Validación

- Build: `cargo build --release` en scripts/tools-rs sin errores.
- Presencia: scripts/tools/start-api/start_api.exe existe tras install.ps1.
- Ejecución: la herramienta devuelve JSON con toolId "start-api", exitCode 0 cuando health OK, feedback con fases y data con url_base, port, pid, healthy.
- Contrato: cumple output (toolId, exitCode, success, timestamp, message, feedback, data, duration_ms) y feedback entry schema.

## Referencias

- SddIA/process/create-tool/spec.md
- SddIA/tools/tools-contract.json
- SddIA/agents/cumulo.paths.json (paths.toolCapsules.start-api, paths.toolsRustPath)
- scripts/tools/start-api/start-api.md (uso del binario Rust)
