# Objetivos: create-tool start-api (ejecutable Rust)

**Proceso:** create-tool (SddIA/process/create-tool)  
**Rama:** feat/create-tool-start-api  
**Persistencia:** paths.featurePath/create-tool-start-api (Cúmulo)

## Objetivo principal

Tener la herramienta **start-api** con **ejecutable Rust** como implementación por defecto, cumpliendo el estándar indicado en SddIA/tools/tools-contract.json (default_implementation.language: rust). Actualmente existe el script PowerShell (Start-Api.ps1) en la cápsula; el estándar exige binario .exe en Rust, con fallback a .ps1 cuando el .exe no esté presente.

## Situación actual

- **Cápsula:** paths.toolCapsules.start-api → `./scripts/tools/start-api/`
- **Definición SddIA:** SddIA/tools/start-api/ (spec.md, spec.json)
- **Implementación Rust:** paths.toolsRustPath → `./scripts/tools-rs/` → binario `start_api` (src/bin/start_api.rs)
- **Launcher:** Start-Api.bat invoca `start_api.exe` si existe en la cápsula; si no, fallback a Start-Api.ps1
- **Install:** scripts/tools-rs/install.ps1 compila con `cargo build --release` y copia `start_api.exe` a la cápsula start-api

## Entregables

1. **Ejecutable Rust** en la cápsula: `start_api.exe` disponible en scripts/tools/start-api/ (tras ejecutar install.ps1 desde scripts/tools-rs).
2. **Comportamiento:** El launcher .bat prioriza el .exe; el .ps1 queda como fallback.
3. **Contrato:** Salida JSON y feedback según tools-contract.json (ya implementado en start_api.rs).
4. **Documentación:** Objetivos y spec en esta carpeta; definición en SddIA/tools/start-api ya existe y está alineada.

## Criterio de éxito

- Tras `cargo build --release` y `install.ps1` en scripts/tools-rs, existe `scripts/tools/start-api/start_api.exe`.
- Al ejecutar Start-Api.bat (o el .exe directamente), la herramienta devuelve JSON de resultado y considera éxito solo si el endpoint health responde HTTP 200.
- Parámetros soportados: --no-build, --profile, --port, --port-blocked (fail|kill), --config-path, --output-path, --output-json.

## Notas

- No se ejecutan comandos de sistema directamente (norma SddIA: skills/herramientas/acciones). La compilación e instalación se realizan mediante la herramienta o el script install.ps1 según el proceso del proyecto.
- Rama de trabajo: feat/create-tool-start-api (crear con skill iniciar-rama si aplica).
