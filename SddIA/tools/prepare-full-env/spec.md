---
contract_ref: SddIA/tools/tools-contract.md
cumulo_ref: SddIA/agents/cumulo.json
env:
- Windows 11
- PowerShell 7+
- Docker Desktop
- .NET SDK
implementation_path_ref: paths.toolCapsules.prepare-full-env
inputs:
  ConfigPath: string (opcional). Ruta al JSON de configuración; por defecto en implementación.
  DockerOnly: boolean (opcional). Solo levantar Docker.
  NoDocker: boolean (opcional). No levantar Docker; ejecutar únicamente las fases no-Docker definidas por config (p. ej. clientes).
  OutputJson: boolean (opcional). Emitir resultado JSON por stdout.
  OutputPath: string (opcional). Fichero donde escribir el resultado JSON.
output:
  phases_feedback:
  - init
  - docker
  - mysql
  - clients
  - done
  - error
  schema_ref: tools-contract.md output.required_fields y optional_fields
toolId: prepare-full-env
version: 1.0.0
---

# Especificación: prepare-full-env

**toolId:** `prepare-full-env`  
**Definición (SddIA):** Este directorio.  
**Implementación (scripts):** Ruta canónica en Cúmulo → **implementation_path_ref:** `paths.toolCapsules.prepare-full-env` (consultar `SddIA/agents/cumulo.json`). La raíz del path de implementación la indica Cúmulo.

## Objetivo

Herramienta que prepara el entorno de desarrollo: levanta servicios Docker (MySQL, Memcached, Adminer), espera a que MySQL esté listo y opcionalmente restaura las seeds de datos.

## Entradas

| Parámetro | Tipo | Descripción |
|----------|------|-------------|
| DockerOnly | switch | Solo levantar Docker (DB, cache, Adminer). |
| NoDocker | switch | No levantar Docker; ejecutar únicamente las fases no-Docker habilitadas en config (p. ej. clientes). |
| ConfigPath | string | Ruta al JSON de configuración (por defecto en la implementación). |
| OutputPath | string | Fichero donde escribir el resultado JSON (contrato). |
| OutputJson | switch | Emitir el resultado JSON por stdout. |

## Salida

Cumple `SddIA/tools/tools-contract.md`: objeto JSON (envelope v2) con toolId, exitCode, success, timestamp, message, feedback[], **result** (docker, clients), duration_ms.

## Fases (feedback)

init → docker → mysql → clients → done (o error).

## Implementación

**Formato:** Ejecutable Rust (`.exe`)  
**Ubicación:** `scripts/tools/prepare-full-env/prepare_full_env.exe`  
**Fuente Rust:** `scripts/tools-rs/src/bin/prepare_full_env.rs`

**Estándar:** Solo se generan ejecutables `.exe`. No se deben crear archivos `.ps1`.

### Invocación

```powershell
# Invocación directa
& "scripts/tools/prepare-full-env/prepare_full_env.exe" [opciones]

# Opciones disponibles
--docker-only          # Solo levantar Docker (DB, cache, Adminer)
--no-docker           # No levantar Docker
--config-path <path>  # Ruta al JSON de configuración
--output-path <path>  # Fichero donde escribir resultado JSON
--output-json         # Emitir resultado JSON por stdout
```

**Implementación (scripts):** Ruta canónica en Cúmulo → **implementation_path_ref:** `paths.toolCapsules.prepare-full-env` (consultar `SddIA/agents/cumulo.json`). La raíz del path de implementación la indica Cúmulo.
