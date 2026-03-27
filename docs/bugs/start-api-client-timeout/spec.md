---
fix_id: start-api-client-timeout
title: "Especificación técnica: start-api y Timeout del cliente"
process: bug-fix
paths_ref: Cúmulo paths.fixPath
spec_version: 1.0.0
created: 2026-03-26
contract_ref: SddIA/norms/capsule-json-io.md
implementation_ref: scripts/tools-rs/src/bin/start_api.rs, scripts/gesfer-capsule/src/lib.rs
---

# Especificación: fix start-api / Timeout cliente

## 1. Síntoma

Invocación de `start_api.exe` (o ruta equivalente en cápsula) desde un **cliente** que limita tiempo total o exige actividad periódica; el proceso termina por **timeout del cliente** o no devuelve JSON a tiempo.

## 2. Causas raíz (validadas)

| Causa | Descripción | Evitable |
|-------|-------------|----------|
| **A. stdin bloqueante** | Con stdin **no TTY**, `read_to_string` espera **EOF**. Si el cliente lanza el proceso con stdin abierto y **sin datos ni EOF**, la herramienta **no avanza** (bloqueo indefinido). | Sí: env `GESFER_SKIP_STDIN` / `GESFER_CAPSULE_REQUEST`; además **no leer stdin si no hay datos pendientes** (peek / disponibilidad) o si hay **argv adicional** (modo CLI explícito). |
| **B. Build silencioso** | `dotnet build` con `.output()` no emite nada hasta terminar; duraciones de minutos son normales en cold build. Clientes que cortan por **inactividad** en stderr/stdout disparan timeout. | Sí: **líneas periódicas a stderr** durante la compilación (`[start-api] ...`). |
| **C. Health lento / BD** | API tarda en aceptar conexiones; health supera ventana; MySQL no listo → ya se cubre con exit 7/8; se mejora **payload** `error_type`. | Parcial: mensajes y `error_type` explícitos; timeout configurable ya existe. |

## 3. Requisitos de solución

### 3.1 `gesfer-capsule` — `try_read_capsule_request`

- Antes de un `read` bloqueante sobre stdin no TTY, determinar si hay **bytes disponibles** (Windows: `PeekNamedPipe`; Unix: `ioctl` FIONREAD). Si **0**, tratar como **sin petición capsule** → `Ok(None)` (modo CLI).
- Si la detección no aplica (p. ej. error de peek), mantener lectura o aplicar heurística **argv**: más de un argumento en línea de comandos sin stdin TTY → `Ok(None)` para evitar bloqueo con flags típicos (`--output-json`, etc.).

### 3.2 `start_api` — build y feedback

- Durante `dotnet build`, mantener un **hilo de progreso** que cada **N segundos** (p. ej. 10) escriba en **stderr** una línea fija `GESFER:` o `[start-api]` para que el cliente no vea inactividad.
- Al terminar el build, detener el hilo.

### 3.3 Resultado JSON — errores

- Incluir en `result` cuando exista fallo:
  - `error_type`: `"health_timeout"` | `"database_unavailable"` | `"build_failed"` | ...
  - Mensajes cortos y accionables (sin secretos).

## 4. Criterios de aceptación

- [x] Invocación `start_api.exe --output-json` sin pipe stdin y sin variables de entorno especiales **no bloquea** indefinidamente (peek stdin + heurística argv en `gesfer-capsule`).
- [x] Durante build, stderr recibe mensajes periódicos (`BUILD_PROGRESS_SECS`, p. ej. 10 s).
- [x] Fallos de health timeout incluyen `error_type: "health_timeout"` y tiempos en `result`.

## 5. Referencias

- `SddIA/norms/capsule-json-io.md` §4.1 (variables de entorno).
- `docs/bugs/start-api-client-timeout/objectives.md` (objetivos del fix).
