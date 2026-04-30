---
process_id: audit-tool
spec_version: 3.0.0
tool_id: prepare-full-env
audit_date: 2026-04-27
audit_id: audit-2026-04-27-01
result: PARTIAL
tools_contract_ref: SddIA/tools/tools-contract.md
tool_spec_ref: paths.toolsDefinitionPath/prepare-full-env/spec.md
capsule_path_ref: paths.toolCapsules[prepare-full-env]
director_validation_ref: "Validación Director 2026-04-27 — Fases 7-8 aprobadas"
---

## Resumen ejecutivo

- **Veredicto**: **PARTIAL**
- **Tipología**: Batch / Mutador de entorno
- **Hallazgos clave**:
  1. **Core funcional (Objetivo):** con **cwd = raíz del repositorio**, los escenarios Docker (`--docker-only` y `--start-api` sin `--no-docker`) **levantan** los servicios declarados en config y **MySQL queda listo** (coherente con A-OBJ-1 y A-OBJ-2 en `objectives.md`).
  2. **Trazabilidad:** la fase **`clients`** declarada en `tool_spec_ref.frontmatter.output.phases_feedback` **no aparece** en la salida real de ningún caso ejecutado. Además, en **`--no-docker`** sigue apareciendo la fase **`docker`** en `feedback[]`, lo que contradice la semántica esperada del flag.
  3. **Flag `--start-api`:** la herramienta **no levanta la API**; solo emite un mensaje informativo (p. ej. sugerencia de `dotnet run`). Es una **promesa rota** respecto a la expectativa operativa del flag (ver anomalía **[HIGH]**).
- **Recomendaciones**:
  - **P1:** La herramienta **no debería depender** del **CWD** (directorio de trabajo actual) siendo la raíz del repositorio; debe resolver rutas canónicas (p. ej. anclar `dockerComposePath` al repo mediante detección explícita o variable/documentación mínima inequívoca).
  - **P2:** El **contrato de fases** (`output.phases_feedback` + texto de spec) debe **sincronizarse con el código real** (fases emitidas, semántica de `--no-docker`, comportamiento real de `--start-api`).

## Contexto y alcance

### Objetivo de la tool (cita literal, `tool_spec_ref`)

> Herramienta que prepara el entorno de desarrollo: levanta servicios Docker (MySQL, Memcached, Adminer), espera a que MySQL esté listo y opcionalmente restaura las seeds de datos.

### Qué se audita

- Ejecutabilidad (`.exe` en raíz de cápsula).
- Contrato estructural **v2** (envelope `capsule-json-io`, payload en **`result`**, no `data`).
- Trazabilidad: `feedback[].phase` frente a `output.phases_feedback`.
- Aserciones funcionales **A-OBJ-1** y **A-OBJ-2** exclusivamente del párrafo «Objetivo» (A-OBJ-3 N/A en esta auditoría).

### Qué no se audita

- Restauración de seeds (**A-OBJ-3**): ningún caso activó esa rama.

## Entradas y parámetros usados

- **Fuente de parámetros:** `prepare-env.json` (manifest `components.config` de la cápsula; diverge del patrón genérico `<tool-id>-config.json` del proceso `audit-tool`).
- **Condición de ejecución descubierta:** invocación con **cwd = raíz del repo** (`c:\Proyectos\GesFer.Admin.Back`) para que `dockerComposePath: docker-compose.yml` resuelva frente al `docker-compose.yml` del repo.

### Comandos ejecutados (resumen; detalle en `paths.featurePath/audit-tool-prepare-full-env/execution.md`)

```powershell
Set-Location 'c:\Proyectos\GesFer.Admin.Back'

# T1
.\scripts\tools\prepare-full-env\prepare_full_env.exe `
  --docker-only --output-json `
  --config-path .\scripts\tools\prepare-full-env\prepare-env.json

# T2
.\scripts\tools\prepare-full-env\prepare_full_env.exe `
  --docker-only `
  --output-path .\docs\audits\tools\prepare-full-env\T2-result.json `
  --config-path .\scripts\tools\prepare-full-env\prepare-env.json

# T3 (aislado, sin --docker-only)
.\scripts\tools\prepare-full-env\prepare_full_env.exe `
  --no-docker --output-json `
  --config-path .\scripts\tools\prepare-full-env\prepare-env.json

# T4 (aislado, sin --no-docker)
.\scripts\tools\prepare-full-env\prepare_full_env.exe `
  --start-api --output-json `
  --config-path .\scripts\tools\prepare-full-env\prepare-env.json
```

## Contrato de salida (tools-contract v2)

### Envelope

En **T1–T4** el JSON cumple: `meta.schema_version == "2.0"`, `success`/`exitCode` coherentes, `feedback[]` con campos exigidos, **`result`** presente (sin clave `data` a nivel raíz), `duration_ms` presente.

### Trazabilidad de fases (feedback)

- **Fases esperadas (SSOT):** `init`, `docker`, `mysql`, `api`, `clients`, `done`, `error` (`tool_spec_ref` frontmatter).
- **Hallazgos:**
  - **`clients`:** ausente en todos los casos → incumplimiento de trazabilidad contractual frente a la lista declarada.
  - **`--no-docker` (T3):** aparece fase **`docker`** (“Docker OK”) → incoherencia semántica con el flag.
  - **`--start-api` (T4):** fase `api` **no** implica API levantada; solo texto guía → ver anomalía **[HIGH]**.

## Validación funcional del objetivo (Fase 6)

> Aserciones **solo** desde «Objetivo» (`objectives.md`: A-OBJ-1, A-OBJ-2).

| ID | Resultado | Evidencia |
|:---|:----------|:----------|
| **A-OBJ-1** | **PASS** (T1, T2, T4) | `evidence-docker-compose-ps.txt`: servicios `gesfer-db` / contenedor `gesfer_db`, `cache`, `adminer` en estado **Up** (healthy en DB). |
| **A-OBJ-2** | **PASS** (T1, T2, T4) | JSON: mensajes de fase `mysql` → “MySQL listo”, `result.mysql_ready == true`; `evidence-mysql-inspect.txt`: health `"Status":"healthy"`. |
| **A-OBJ-3** | **N/A** | Sin escenario de seeds. |

## Limpieza / cierre (Fase 8)

- **`cleanup_after_audit`:** true (directriz Director).
- **Acción de reversión:** desde la raíz del repo, `docker compose down` (detiene y elimina contenedores/servicios del compose auditado).
- **Resultado:** ver salida capturada en `evidence-docker-compose-down.txt` (misma carpeta de auditoría).

## Evidencias

- `T1-stdout.json`, `T2-result.json`, `T3-stdout.json`, `T4-stdout.json`
- `evidence-docker-compose-ps.txt`, `evidence-mysql-inspect.txt`
- `evidence-docker-compose-down.txt` (post-cierre)
- Trazabilidad de tarea: `paths.featurePath/audit-tool-prepare-full-env/execution.md`, `validacion.md`, `plan.md`

## Anomalías y recomendaciones

### Anomalías

- **[HIGH] START_API_NO_OP:** con `--start-api`, la tool **no inicia** la Admin API; solo emite mensaje informativo. Promesa operativa del flag **rota** frente a expectativa de usuario/automatización.
- **[MEDIUM] TRACEABILITY_CLIENTS_MISSING:** la fase `clients` declarada en `output.phases_feedback` **no** aparece en `feedback[]` en ningún caso.
- **[MEDIUM] NO_DOCKER_EMITS_DOCKER_PHASE:** con `--no-docker`, `feedback[]` incluye fase `docker` (“Docker OK”), incoherente con el nombre del flag.
- **[LOW] SPEC_VS_RUNTIME_DATA_FIELD:** el cuerpo de `tool_spec_ref` §Salida menciona `data`; runtime usa **`result`** (v2) — deuda documental.

### Recomendaciones accionables

- **P1:** Eliminar dependencia implícita del **CWD**; resolver `docker-compose.yml` y paths de forma **explícita** (p. ej. detección de repo root robusta o ruta absoluta configurable validada).
- **P2:** Alinear **código** y **contrato de fases**: emitir `clients` cuando aplique (o retirarla del contrato); corregir semántica `--no-docker`; implementar o renombrar/documentar `--start-api` si solo es asistente.

## Apéndice: Matriz de pruebas (Fase 3)

| Caso | Parámetros | Esperado | Observado | Resultado |
|:-----|:-------------|:---------|:----------|:----------|
| T1 | `--docker-only --output-json` + config | Docker up + MySQL ready + JSON v2 | Cumple | PASS (funcional) / PARTIAL (trazabilidad) |
| T2 | `--docker-only --output-path` + config | Igual que T1, JSON en fichero | Cumple | PASS / PARTIAL |
| T3 | `--no-docker --output-json` | Sin fase docker / sin provisión Docker | Aparece fase `docker` | PARTIAL |
| T4 | `--start-api --output-json` | API levantada o evidencia de proceso | Solo mensaje | PARTIAL (HIGH) |
