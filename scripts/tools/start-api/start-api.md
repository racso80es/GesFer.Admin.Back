# start-api

**toolId:** start-api  
**Cápsula:** paths.toolCapsules.start-api (Cúmulo)

## Objetivo

Levanta la API del proyecto GesFer.Admin.Back. Comprueba si el puerto está ocupado; según `--port-blocked` puede fallar o cerrar el proceso que lo usa. Tras build opcional, inicia la API y considera **éxito** solo si el endpoint **health** responde correctamente (HTTP 200).

## Uso (binario Rust)

Desde la raíz del repositorio (o con `GESFER_REPO_ROOT` apuntando a la raíz):

```powershell
# Desde scripts/tools/start-api/ o con PATH
.\start_api.exe

# Puerto ocupado: fallar (por defecto)
.\start_api.exe --port-blocked fail

# Puerto ocupado: cerrar proceso que usa el puerto y continuar
.\start_api.exe --port-blocked kill

# Opciones
.\start_api.exe --no-build
.\start_api.exe --profile Development
.\start_api.exe --port 5010
.\start_api.exe --output-json
.\start_api.exe --output-path result.json
.\start_api.exe --config-path ruta/a/start-api-config.json
```

### Invocación por agente (IA)

Mismo envelope que `SddIA/norms/capsule-json-io.md`. Si el entorno no puede hacer pipe + EOF de forma fiable:

- **`GESFER_SKIP_STDIN=1`** y flags CLI, p. ej. `--output-json --port-blocked kill`.
- O **`GESFER_CAPSULE_REQUEST`** con el JSON del envelope (sin usar stdin).

#### Objeto `request` (parámetros = flags CLI)

Cada clave es opcional. El binario acepta **camelCase** (recomendado), **PascalCase** (como en `SddIA/tools/start-api/spec.md`) y **snake_case** en los campos compuestos.

| JSON (`request`) | Flags CLI equivalentes |
|------------------|-------------------------|
| `noBuild` / `NoBuild` / `no_build` | `--no-build` |
| `profile` / `Profile` | `--profile <valor>` |
| `port` / `Port` | `--port <número>` |
| `configPath` / `ConfigPath` / `config_path` | `--config-path <ruta>` |
| `outputPath` / `OutputPath` / `output_path` | `--output-path <ruta>` |
| `outputJson` / `OutputJson` / `output_json` | `--output-json` |
| `portBlocked` / `PortBlocked` / `port_blocked` | `--port-blocked fail` \| `kill` |

#### `start-api-config.json`

Además de `apiWorkingDir`, `defaultPort`, `healthUrl`, etc., puede incluir **`portBlocked`** (`"fail"` \| `"kill"`). **Prioridad:** flag `--port-blocked` > objeto `request` del envelope > esta propiedad en el JSON de configuración > `fail`.

Si un cliente solo fusiona `portBlocked` en un objeto de configuración pero **no** lo envía en `request` ni en la CLI, las versiones anteriores del binario **no** lo aplicaban (se quedaba el valor por defecto `fail`).

## Salida

JSON según `SddIA/norms/capsule-json-io.md` y `SddIA/tools/tools-contract.md`: `meta`, `success`, `exitCode`, `message`, `feedback[]`, **`result`** (p. ej. `url_base`, `port`, `pid`, `healthy`, `profile` cuando aplica), `duration_ms`. **success = true** solo si el health responde 200.

### Códigos de error relevantes

| exitCode | Situación |
|----------|-----------|
| 5 | Build fallido (`result.error_type`: `build_failed`) |
| 7 | Health no respondió a tiempo (`result.error_type`: `health_timeout`; revisar MySQL, tiempo en `healthCheckTimeoutSeconds`) |
| 8 | **Base de datos (MySQL) no disponible** (`result.error_type`: `database_unavailable`) — ejecute prepare-full-env e invoke-mysql-seeds antes |

Durante `dotnet build`, la herramienta escribe líneas periódicas en **stderr** (`[start-api] compilación en curso…`) para evitar que clientes con watchdog por inactividad corten el proceso.

## Implementación

Implementación **obligatoria en Rust** (binario `start_api.exe` en la misma carpeta que el .bat). El launcher .bat invoca el .exe si existe; si no, fallback a Start-Api.ps1.
