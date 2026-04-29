---
feature: invoke-mysql-seeds-drop-create-db
date: 2026-04-28
status: draft
plan:
  - Implementar invoke_mysql_seeds.rs con patrón prepare_full_env.rs
  - Actualizar SddIA/tools/invoke-mysql-seeds/spec.md (inputs y fases)
  - Actualizar scripts/tools/invoke-mysql-seeds/mysql-seeds.md (uso y ejemplos)
  - Ajustar Invoke-MySqlSeeds.bat (default DropCreateDb)
  - Compilar tools Rust y copiar invoke_mysql_seeds.exe a la cápsula
  - Validación mínima local (ejecución con --output-json / output-path)
---

## Plan

El trabajo sigue el patrón de `prepare-full-env` (Rust + feedback por fases + legacy args + wrapper `.bat`).

