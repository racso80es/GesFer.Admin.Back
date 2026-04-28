---
process_id: audit-tool
spec_version: 3.0.0
tool_id: prepare-full-env
audit_date: 2026-04-27
audit_id: audit-2026-04-27-02
tool_spec_ref: paths.toolsDefinitionPath/prepare-full-env/spec.md
capsule_path_ref: paths.toolCapsules[prepare-full-env]
process_master_ref: paths.processPath/audit-tool/spec.md
---

# Plan — audit-tool (prepare-full-env)

## Estado del proceso (punto de control)

| Fase | Nombre | Estado |
|:-----|:-------|:-------|
| 0 | Preparar entorno y Clasificar | **Completada** |
| 1 | Definir objetivos dinámicos | **Completada** |
| 2 | Analizar especificación y Contrato | **Completada** (resumen abajo) |
| 3 | Diseñar pruebas | **Completada** (matriz abajo) |
| 4 | Ejecutar herramienta | **Completada** → `execution.md` + ficheros bajo `paths.auditsPath/tools/prepare-full-env/` |
| 5 | Validar retorno JSON y Trazabilidad | **Completada** → `validacion.md` §5 |
| 6 | Validar objetivos funcionales | **Completada** → `validacion.md` §6 |
| 7 | Generar informe | **Completada** → `paths.auditsPath/tools/prepare-full-env/audit-report-2026-04-27-01.md` + `audit-result-2026-04-27-01.json` (**veredicto PARTIAL**, directriz Director) |
| 8 | Cierre y Limpieza | **Completada** → `docker compose down` (cwd raíz repo); evidencia `evidence-docker-compose-down.txt` |

---

## Anclajes obligatorios (SSOT)

| Rol | Referencia |
|:----|:-----------|
| Proceso maestro | `paths.processPath/audit-tool/spec.md` |
| Spec individual | `paths.toolsDefinitionPath/prepare-full-env/spec.md` |
| Contrato herramientas | `SddIA/tools/tools-contract.md` |
| Envelope JSON (payload en `result`) | `SddIA/norms/capsule-json-io.md` (`schema_version: "2.0"`) |
| Rutas | `SddIA/agents/cumulo.paths.json` |

---

## Fase 2 — Análisis especificación vs contrato (resumen)

**Nota (2026-04-28):** La flag `StartApi` / `--start-api` se retira de `prepare-full-env` (feature `prepare-full-env-drop-start-api`). Esta planificación queda como evidencia del diseño de pruebas original; los casos T4 asociados a `--start-api` pasan a ser **obsoletos** para ejecuciones futuras.

### Inputs / flags (frontmatter + cuerpo `tool_spec_ref`)

| Flag / input | Representación CLI (spec) |
|:-------------|:--------------------------|
| DockerOnly | `--docker-only` |
| NoDocker | `--no-docker` |
| ConfigPath | `--config-path <path>` |
| OutputPath | `--output-path <path>` |
| OutputJson | `--output-json` |

**Config efectiva de cápsula (manifest):** `prepare-env.json` (no coincide con el patrón genérico `<tool-id>-config.json` del proceso; queda anotado como convención operativa real).

### `output.phases_feedback` (SSOT trazabilidad)

`init`, `docker`, `mysql`, `api`, `clients`, `done`, `error`.

### Anomalía contractual (spec vs v2)

En el cuerpo de `tool_spec_ref`, sección **Salida**, la spec aún menciona payload como `data`; el contrato v2 y `capsule-json-io` exigen **`result`**. Las ejecuciones emiten **`result`** correctamente → **desalineación documental** en la spec, no en runtime.

### Condición de ejecución descubierta (repo root)

El `docker-compose.yml` vive en la **raíz del repositorio** (`./docker-compose.yml`). La invocación debe hacerse con **cwd = raíz del repo** y `--config-path` apuntando a la cápsula; si el cwd es solo la cápsula, la tool resolvió mal `docker-compose.yml` (hallazgo previo; mitigado en esta tanda).

---

## Fase 3 — Matriz de pruebas (cobertura inteligente)

**Reglas Director:** sin combinar flags mutuamente excluyentes; **cada caso** incluye `--output-json` **o** `--output-path`; cwd = `paths` raíz del repo (resolución compose).

### Comandos de evidencia Batch (A-OBJ-1 / A-OBJ-2)

Ejecutados tras escenarios que levantan Docker (T1, T2, T4), desde raíz del repo:

1. **A-OBJ-1 (servicios up):**  
   `docker compose ps --format "table {{.Name}}\t{{.Service}}\t{{.Status}}\t{{.Ports}}"`  
   → evidencia: `paths.auditsPath/tools/prepare-full-env/evidence-docker-compose-ps.txt`

2. **A-OBJ-2 (readiness MySQL):**  
   `docker inspect gesfer_db --format '{{json .State.Health}}'`  
   → evidencia: `paths.auditsPath/tools/prepare-full-env/evidence-mysql-inspect.txt`  
   (nombre de contenedor tomado del propio resultado JSON de la tool: `result.mysql_container`.)

| Caso | Invocación (cwd = raíz repo) | Flags cubiertos | Objetivo |
|:-----|:-----------------------------|:-----------------|:---------|
| **T1** | `.\scripts\tools\prepare-full-env\prepare_full_env.exe --docker-only --output-json --config-path .\scripts\tools\prepare-full-env\prepare-env.json` | DockerOnly, OutputJson, ConfigPath | Camino Docker-only + JSON stdout |
| **T2** | mismo exe `--docker-only --output-path docs\audits\tools\prepare-full-env\T2-result.json --config-path ...` | DockerOnly, OutputPath, ConfigPath | Misma semántica que T1; JSON en fichero |
| **T3** | `... --no-docker --output-json --config-path ...` | NoDocker, OutputJson, ConfigPath | Aislado: sin `--docker-only` |

**A-OBJ-3:** ningún caso activa “restaurar seeds”; queda **N/A** explícito en `validacion.md`.

---

## Fase 7–8 — Cierre (Director aprobado)

- **Informe:** `audit-report-2026-04-27-01.md` y `audit-result-2026-04-27-01.json` bajo `paths.auditsPath/tools/prepare-full-env/`.
- **Reversión Docker:** `docker compose down` desde la raíz del repositorio (detiene y elimina contenedores/red del stack auditado). Log: `evidence-docker-compose-down.txt`.
