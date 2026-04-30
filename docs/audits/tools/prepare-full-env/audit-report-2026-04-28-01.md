---
process_id: audit-tool
spec_version: 3.0.0
tool_id: prepare-full-env
audit_date: 2026-04-28
audit_id: audit-2026-04-28-01
result: PARTIAL
tools_contract_ref: SddIA/tools/tools-contract.md
tool_spec_ref: paths.toolsDefinitionPath/prepare-full-env/spec.md
capsule_path_ref: paths.toolCapsules[prepare-full-env]
---

## Resumen ejecutivo

- **Veredicto**: **PARTIAL**
- **Tipología**: **Batch** (mutador de entorno Docker)
- **Hallazgos clave**:
  - Runtime cumple envelope v2 y usa `result`.
  - `phases_feedback` observado sin `api`.
  - La spec (cuerpo) aún menciona `data` (deuda documental).
- **Recomendaciones**:
  - **P1 (HIGH)**: corregir `SddIA/tools/prepare-full-env/spec.md` para describir `result` (no `data`) en §Salida.

## Contexto y alcance

- **Objetivo de la tool (tool_spec_ref/Objetivo)**:
  > “Herramienta que prepara el entorno de desarrollo: levanta servicios Docker (MySQL, Memcached, Adminer), espera a que MySQL esté listo y opcionalmente restaura las seeds de datos.”
- **Qué se audita**:
  - Ejecutabilidad `.exe`
  - Envelope v2 (`capsule-json-io` / tools-contract)
  - Trazabilidad `feedback[].phase` vs `output.phases_feedback`
  - Evidencia funcional Docker/MySQL
- **Qué NO se audita**:
  - Auditorías históricas previas y su evidencia (inmutables).

## Entradas y parámetros usados

Fuente de parámetros: `scripts/tools/prepare-full-env/prepare-env.json`.

Comandos ejecutados (cwd = raíz del repo):

```powershell
.\scripts\tools\prepare-full-env\prepare_full_env.exe `
  --docker-only `
  --output-json `
  --output-path .\docs\audits\tools\prepare-full-env\T1-result-2026-04-28-01.json `
  --config-path .\scripts\tools\prepare-full-env\prepare-env.json

.\scripts\tools\prepare-full-env\prepare_full_env.exe `
  --no-docker `
  --output-json `
  --output-path .\docs\audits\tools\prepare-full-env\T2-result-2026-04-28-01.json `
  --config-path .\scripts\tools\prepare-full-env\prepare-env.json
```

## Contrato de salida (tools-contract v2)

- **JSON emitido**: stdout + fichero (`--output-path`) en ambos casos.
- **Comprobaciones**:
  - `meta.schema_version == "2.0"`
  - `meta.entity_kind == "tool"`
  - `meta.entity_id == "prepare-full-env"`
  - coherencia `success` ↔ `exitCode`
  - `feedback[]` con `phase|level|message|timestamp`
  - payload en **`result`**

### Trazabilidad de fases (feedback)

- **Fases esperadas (SSOT)**: `init`, `docker`, `mysql`, `clients`, `done`, `error`
- **Fases observadas**:
  - T1: `init`, `docker`, `mysql`, `clients`, `done`
  - T2: `init`, `docker`, `clients`, `done`
- **Nota**: `error` no aparece en ejecuciones exitosas.

## Validación funcional del objetivo (fase 6)

### A1 — Servicios Docker up (T1)

- Evidencia: `evidence-docker-compose-ps-2026-04-28-01.txt`

### A2 — MySQL healthy (T1)

- Evidencia: `evidence-mysql-inspect-2026-04-28-01.json` + fase `mysql` en el JSON de T1.

## Limpieza / cierre

- `cleanup_after_audit`: `true`
- Acción: `docker compose down`
- Evidencia: `evidence-docker-compose-down-2026-04-28-01.txt`

## Evidencias

- `audit-result-2026-04-28-01.json`
- `T1-result-2026-04-28-01.json`
- `T2-result-2026-04-28-01.json`
- `evidence-docker-compose-ps-2026-04-28-01.txt`
- `evidence-mysql-inspect-2026-04-28-01.json`
- `evidence-docker-compose-down-2026-04-28-01.txt`

