---
process_id: audit-tool
spec_version: 3.0.0
tool_id: invoke-mysql-seeds
audit_date: 2026-04-28
audit_id: audit-2026-04-28-01
result: PASS
tools_contract_ref: SddIA/tools/tools-contract.md
tool_spec_ref: paths.toolsDefinitionPath/invoke-mysql-seeds/spec.md
capsule_path_ref: paths.toolCapsules[invoke-mysql-seeds]
---

## Resumen ejecutivo

- **Veredicto**: **PASS**
- **Tipología**: **Batch** (mutador de entorno: BD / migraciones / seeds)
- **Hallazgos clave**:
  - El ejecutable `invoke_mysql_seeds.exe` existe en la raíz de la cápsula y expone los flags esperados.
  - La salida JSON cumple **envelope v2** (`meta`, `success`, `exitCode`, `message`, `feedback`, `result`, `duration_ms`) y usa **`result`**.
  - La trazabilidad por fases en `feedback[].phase` coincide con la spec: `init`, `mysql`, `db_drop_create`, `migrations`, `seeds`, `done` (en ejecuciones exitosas).
  - El wrapper `Invoke-MySqlSeeds.bat` ejecuta **DropCreateDb** por defecto (estrategia B).

## Contexto y alcance

- **Objetivo de la tool (tool_spec_ref/Objetivo)**:
  > “Herramienta que comprueba la disponibilidad de MySQL, aplica migraciones EF Core y ejecuta los seeds de Admin (companies, admin-users) mediante la variable de entorno RUN_SEEDS_ONLY=1 en la API.”
- **Qué se audita**:
  - Ejecutabilidad `.exe`
  - Contrato estructural (tools-contract v2 / `capsule-json-io`)
  - Trazabilidad de fases (`feedback[].phase`)
  - Cumplimiento funcional (MySQL listo, migraciones y seeds ejecutables)
- **Qué NO se audita**:
  - Correctitud de contenido de seeds (datos de negocio), solo ejecutabilidad y contrato.

## Entradas y parámetros usados

Fuente de parámetros: `scripts/tools/invoke-mysql-seeds/mysql-seeds-config.json`.

Comandos ejecutados (cwd = raíz del repo):

```powershell
.\scripts\tools\invoke-mysql-seeds\invoke_mysql_seeds.exe --help

.\scripts\tools\prepare-full-env\prepare_full_env.exe `
  --docker-only `
  --output-json `
  --output-path .\docs\audits\tools\invoke-mysql-seeds\setup-prepare-full-env-2026-04-28-01.json

.\scripts\tools\invoke-mysql-seeds\invoke_mysql_seeds.exe `
  --drop-create-db `
  --skip-migrations `
  --skip-seeds `
  --output-json `
  --output-path .\docs\audits\tools\invoke-mysql-seeds\T2-dropcreate-only-2026-04-28-01.json

.\scripts\tools\invoke-mysql-seeds\invoke_mysql_seeds.exe `
  --drop-create-db `
  --output-json `
  --output-path .\docs\audits\tools\invoke-mysql-seeds\T3-full-2026-04-28-01.json

.\scripts\tools\invoke-mysql-seeds\Invoke-MySqlSeeds.bat `
  -SkipMigrations -SkipSeeds -OutputJson `
  -OutputPath .\docs\audits\tools\invoke-mysql-seeds\T4-bat-default-2026-04-28-01.json
```

## Contrato de salida (tools-contract v2)

- **JSON emitido**: stdout + fichero (`--output-path`) en T2/T3/T4.
- **Comprobaciones**:
  - `meta.schema_version == "2.0"`
  - `meta.entity_kind == "tool"`
  - `meta.entity_id == "invoke-mysql-seeds"`
  - coherencia `success: true` ↔ `exitCode: 0`
  - `feedback[]` con `phase|level|message|timestamp`
  - payload específico en **`result`**

### Trazabilidad de fases (feedback)

- **Fases esperadas (SSOT)**: `init`, `mysql`, `db_drop_create`, `migrations`, `seeds`, `done`, `error`
- **Fases observadas**:
  - T2: `init`, `mysql`, `db_drop_create`, `migrations`, `seeds`, `done`
  - T3: `init`, `mysql`, `db_drop_create`, `migrations`, `seeds`, `done`
  - T4: `init`, `mysql`, `db_drop_create`, `migrations`, `seeds`, `done`
- **Nota**: `error` no aparece en ejecuciones exitosas.

## Validación funcional del objetivo (fase 6)

### A1 — MySQL listo

- Evidencia: fase `mysql` en T2/T3/T4 + `setup-prepare-full-env-2026-04-28-01.json`.

### A2 — Estrategia B (Drop/Create DB) ejecutada

- Evidencia: `result.db.dropCreate.attempted == true` en T2/T3/T4.

### A3 — Migraciones EF ejecutables

- Evidencia: `result.migrations.success == true` en T3.

### A4 — Seeds ejecutables (RUN_SEEDS_ONLY)

- Evidencia: `result.seeds.success == true` en T3.

### A5 — Wrapper `.bat` ejecuta ambas acciones por defecto

- Evidencia: T4 muestra `db_drop_create` ejecutado aunque no se pasó `-DropCreateDb` explícito (lo inyecta el `.bat`).

## Limpieza / cierre

- `cleanup_after_audit`: `true`
- Acción: **NO definida** explícitamente por la tool `invoke-mysql-seeds` (tipología Batch). No se aplica reversión adicional.

## Evidencias

- `audit-result-2026-04-28-01.json`
- `T1-help-2026-04-28-01.txt`
- `setup-prepare-full-env-2026-04-28-01.json`
- `T2-dropcreate-only-2026-04-28-01.json`
- `T3-full-2026-04-28-01.json`
- `T4-bat-default-2026-04-28-01.json`

