---
feature: invoke-mysql-seeds-drop-create-db
date: 2026-04-28
status: draft
checks:
  - id: V1
    title: Compila tools Rust y copia exe a cápsula
  - id: V2
    title: invoke_mysql_seeds.exe muestra help con flags nuevos
evidence:
  - source: docs/diagnostics/feat/invoke-mysql-seeds-drop-create-db/execution_history.json
---

## Validación

### V1 — Build tools Rust

- Se ejecuta `scripts/tools-rs/install.ps1` y se copia `invoke_mysql_seeds.exe` en `scripts/tools/invoke-mysql-seeds/`.

### V2 — Help del ejecutable

- `invoke_mysql_seeds.exe --help` muestra los flags:
  - `--drop-create-db`
  - `--skip-migrations`
  - `--skip-seeds`
  - `--output-json`, `--output-path`, `--config-path`

