---
process_id: audit-tool
spec_version: 3.0.0
tool_id: prepare-full-env
audit_date: 2026-04-27
audit_id: audit-2026-04-27-02
phases: "5-6"
---

# Validación — Fases 5 y 6 (hallazgos en crudo)

**Fuentes:** `T1-stdout.json`, `T2-result.json`, `T3-stdout.json`, `T4-stdout.json` bajo `paths.auditsPath/tools/prepare-full-env/`.

**Nota (2026-04-28):** La flag `--start-api` se retiró de la tool `prepare-full-env` (feature `prepare-full-env-drop-start-api`). Los apartados que mencionan T4 se mantienen como evidencia del estado observado en la auditoría; no deben tomarse como guía para ejecuciones futuras.

---

## Fase 5 — Validación JSON (envelope v2) y trazabilidad `feedback[]`

### Reglas de envelope (`capsule-json-io` schema `2.0`)

Por caso (T1–T4), el JSON parseado cumple estructuralmente:

| Comprobación | T1 | T2 | T3 | T4 |
|:-------------|:--:|:--:|:--:|:--:|
| `meta.schema_version == "2.0"` | ✓ | ✓ | ✓ | ✓ |
| `meta.entity_kind == "tool"` | ✓ | ✓ | ✓ | ✓ |
| `meta.entity_id == "prepare-full-env"` | ✓ | ✓ | ✓ | ✓ |
| `success` boolean | ✓ | ✓ | ✓ | ✓ |
| `exitCode` number | ✓ | ✓ | ✓ | ✓ |
| Coherencia `success` ↔ `exitCode` (0 solo si true) | ✓ | ✓ | ✓ | ✓ |
| `message` string | ✓ | ✓ | ✓ | ✓ |
| `feedback` array no nulo | ✓ | ✓ | ✓ | ✓ |
| Entradas `feedback[]` con `phase`, `level`, `message`, `timestamp` | ✓ | ✓ | ✓ | ✓ |
| Payload en **`result`** (clave `data` ausente) | ✓ | ✓ | ✓ | ✓ |
| `duration_ms` presente | ✓ | ✓ | ✓ | ✓ |

**Anomalía contractual (documentación):** `tool_spec_ref` cuerpo §Salida menciona `data`; runtime cumple **`result`** (v2). No es fallo de ejecución; es **deuda de especificación**.

### Trazabilidad vs `output.phases_feedback`

Lista canónica (spec frontmatter):  
`init`, `docker`, `mysql`, `api`, `clients`, `done`, `error`.

| Caso | Fases observadas en `feedback[].phase` (orden) | Faltan respecto lista canónica | Nota |
|:-----|:-----------------------------------------------|:---------------------------------|:-----|
| T1 | `init`×2, `docker`×4, `mysql`×2, `done` | `api`, `clients`, `error` | `--docker-only`: no ejecuta API/clientes; ausencia de `clients` esperable; ausencia de `api` discutible si la spec exige siempre fase `api` aunque sea no-op. |
| T2 | (misma secuencia que T1) | igual que T1 | Misma semántica que T1; solo cambia transporte de salida. |
| T3 | `init`×2, `docker`×2, `api`, `done` | `mysql`, `clients`, `error` | Con `--no-docker`, aún aparece fase `docker` (“Docker OK”). `result.mysql_ready == false`. |
| T4 | `init`×2, `docker`×4, `mysql`×2, `api`, `done` | `clients`, `error` | Fase `api` solo informa comando manual `dotnet run ...`; **no** evidencia proceso API levantado por la tool. |

**Hallazgo crudo (trazabilidad):** en **ningún** caso aparece fase `clients` ni `error`. Para rutas exitosas cortas, `error` puede ser opcional; **`clients` ausente** frente a lista canónica completa → posible **PARTIAL** estricto si se exige cobertura 1:1 de todas las fases declaradas en frontmatter aunque no apliquen.

---

## Fase 6 — Aserciones funcionales (solo «Objetivo»)

### A-OBJ-3

**N/A** en todos los casos diseñados: ningún escenario activa restauración de seeds.

---

### T1 y T2 (DockerOnly + salida JSON)

| Aserción | Evidencia | Resultado |
|:---------|:----------|:----------|
| **A-OBJ-1** | `evidence-docker-compose-ps.txt`: servicios `gesfer-db` (contenedor `gesfer_db`), `cache`, `adminer` en **Up** (healthy en db/cache). | **PASS** |
| **A-OBJ-2** | (a) JSON: fases `mysql` incluyen mensaje **«MySQL listo»** y `result.mysql_ready == true`. (b) `evidence-mysql-inspect.txt`: health JSON `"Status":"healthy"`. | **PASS** |

---

### T4 (`--start-api`)

| Aserción | Evidencia | Resultado |
|:---------|:----------|:----------|
| **A-OBJ-1** | Misma evidencia `docker compose ps` post-T1/T4 (servicios up). | **PASS** (mismo stack Docker) |
| **A-OBJ-2** | JSON `mysql_ready true` + health inspect (consistente con T1). | **PASS** |
| **Nota StartApi** | `feedback` fase `api` solo sugiere comando manual; **no** se demuestra API escuchando en esta ejecución. | **Fuera de A-OBJ** (no forma parte del texto «Objetivo» citado). |

---

### T3 (`--no-docker`)

| Aserción | Evidencia | Resultado |
|:---------|:----------|:----------|
| **A-OBJ-1** | Escenario **aislado** explícitamente para validar flag; no pretende el camino completo de provisión Docker del objetivo global. `result.mysql_ready == false`. | **N/A** (por diseño del caso) |
| **A-OBJ-2** | No hay espera exitosa de MySQL en resultado (`mysql_ready false`); tampoco se pretende en este caso. | **N/A** |
| **Hallazgo semántico** | Aun con `--no-docker`, hay fases `docker` (“Docker OK”). | **Requiere interpretación**: posible confusión operativa; documentar para Director. |

---

## Resumen ejecutivo (crudo, no veredicto formal Fase 7)

- **Envelope v2:** OK en T1–T4 (`result` presente; sin `data`).
- **Funcional (A-OBJ-1/2) en caminos Docker reales (T1, T2, T4):** evidencias `docker compose ps` + `docker inspect` + JSON alineados → **PASS** en esas aserciones.
- **Trazabilidad estricta vs lista `phases_feedback` completa:** faltan `clients` (y a menudo `api` en docker-only); **PARTIAL** si política = lista completa obligatoria.
- **T3:** caso aislado `--no-docker`; A-OBJ **N/A**; semántica `docker` presente → revisión humana.
