---
feature_name: start-api-contract-alignment
created: 2026-05-01
base:
  - objectives.md
  - spec.md
  - clarify.md
phases:
  - id: P1
    name: Contrato y configuración
  - id: P2
    name: Puertos y proceso
  - id: P3
    name: Build, lanzamiento y health
  - id: P4
    name: Salida, errores y documentación
tasks:
  - id: T1
    phase: P1
    summary: Modelar config JSON + merge CLI/capsule (camelCase, PascalCase, snake_case en request)
  - id: T2
    phase: P2
    summary: Comprobar puerto en uso; fail (2) vs kill Windows (3 si no libera)
  - id: T3
    phase: P3
    summary: dotnet build sln si no NoBuild; stderr heartbeat en build (spec)
  - id: T4
    phase: P3
    summary: dotnet run con perfil y --no-build opcional; captura stdout/stderr hijo
  - id: T5
    phase: P3
    summary: Bucle health HTTP hasta 200 o timeout; exit 7; MySQL patrones → 8
  - id: T6
    phase: P4
    summary: CapsuleResponse tool con feedback por fases; exitCode 0–8; result url_base, port, pid, healthy, profile
  - id: T7
    phase: P4
    summary: Ajustar start-api-config.json ejemplo, start-api.md, manifest; spec SddIA ruta fuente (D10)
  - id: T8
    phase: P4
    summary: cargo build release start_api; install.ps1 ya copia — verificar; prueba manual opcional
---

# Plan: start-api-contract-alignment

**Proceso:** feature  
**Ruta (Cúmulo):** `paths.featurePath/start-api-contract-alignment/`  
**Entrada:** objectives.md, spec.md, clarify.md (decisiones D1–D10)

---

## 1. Fase P1 — Contrato y configuración

| Paso | Acción |
|------|--------|
| 1.1 | Sustituir `Cli` / `JsonRequest` por campos alineados a SddIA: `config_path`, `no_build`, `profile`, `port`, `port_blocked`, `output_path`, `output_json` (clap + deserialización desde `request` con alias serde para variantes de nombre). |
| 1.2 | Definir struct `StartApiConfig` para `start-api-config.json`: campos actuales + `port_blocked` opcional; default de `config_path` = cápsula `start-api-config.json` junto al exe o ruta documentada en manifest. |
| 1.3 | Resolver `effective_port`, `effective_profile`, `effective_port_blocked` con prioridad **CLI > request > config > fail** (D2). |
| 1.4 | Calcular `health_url` efectiva: si hay override de `Port`, reemplazar host/puerto en la URL parseada de `healthUrl`; si no, usar `healthUrl` literal (D3, D4). |

**Salida de fase:** lectura de config validada; errores config inválida/no encontrada → **exitCode 1** (spec).

---

## 2. Fase P2 — Puertos y proceso

| Paso | Acción |
|------|--------|
| 2.1 | Feedback `port-check` antes de arrancar. |
| 2.2 | Si puerto ocupado y `fail` → **exitCode 2**. |
| 2.3 | Si `kill`: `netstat -ano` (Windows) → `taskkill /PID` (D9); revalidar puerto; si sigue ocupado → **exitCode 3**. |
| 2.4 | Feedback `port-kill` solo cuando aplique. |

---

## 3. Fase P3 — Build, lanzamiento y health

| Paso | Acción |
|------|--------|
| 3.1 | Si `no_build` false: `dotnet build src/GesFer.Admin.Back.sln` desde repo root (D5); en stderr líneas periódicas `[start-api] compilación en curso…` (spec). Fallo → **exitCode 5**, `result.error_type`: `build_failed`. Feedback `build`. |
| 3.2 | Construir comando `dotnet run` en `apiWorkingDir` (D6): `--launch-profile {profile}`; si `no_build`, añadir `--no-build`. **No** usar `cmd /C` genérico con string libre; invocar proceso `dotnet` con args explícitos para trazabilidad. |
| 3.3 | `spawn` hijo sin bloquear; registrar **PID**; pipes stdout/stderr para buffer circular (últimos N KiB o líneas, D8). Feedback `launch`. Fallo spawn → **exitCode 6**. |
| 3.4 | Bucle GET a `health_url` con intervalo corto hasta `healthCheckTimeoutSeconds`; 200 → `healthy: true`. Timeout → **exitCode 7**, `health_timeout`. Durante la espera, escanear buffer por patrones MySQL de la spec → **exitCode 8**, `database_unavailable`. Feedback `healthcheck`. |

**Limpieza en error:** si health falla tras arranque, documentar en implementation si se mata el hijo (recomendado para evitar APIs zombie en CI).

---

## 4. Fase P4 — Salida, errores y documentación

| Paso | Acción |
|------|--------|
| 4.1 | Mapear todas las ramas de error a la tabla **exitCode 0–8** de `SddIA/tools/start-api/spec.md`. |
| 4.2 | `write_capsule_response` / fichero `--output-path` según flags; respetar `GESFER_SKIP_STDIN` / envelope (start-api.md). |
| 4.3 | Actualizar `scripts/tools/start-api/start-api.md` y ejemplo JSON de `request`; opcional `portBlocked` en `start-api-config.json`. |
| 4.4 | Corregir en `SddIA/tools/start-api/spec.md` la ruta de fuente a `scripts/tools-rs/src/bin/start_api.rs` (D10). |
| 4.5 | **Validación:** `cargo build --release -p gesfer-tools` (o manifest tools-rs) y prueba manual con infra MySQL cuando exista; registrar en `validacion.md` en cierre de feature. |

---

## 5. Orden de ejecución recomendado

1. P1 (config + URLs)  
2. P2 (puerto)  
3. P3 (build → run → health + MySQL)  
4. P4 (respuesta JSON + docs + spec editorial)

---

## 6. Dependencias externas

- `prepare-full-env` / `invoke-mysql-seeds` no forman parte del código de esta tarea; la tool solo refleja **exitCode 8** cuando la salida del proceso lo indica.

---

## 7. Siguiente artefacto del proceso

**implementation.md** (touchpoints concretos y pseudocódigo/riesgos), luego ejecución en código y **execution.md**.
