---
feature: invoke-mysql-seeds-drop-create-db
date: 2026-04-28
status: draft
touchpoints:
  - scripts/tools-rs/src/bin/invoke_mysql_seeds.rs
  - scripts/tools/invoke-mysql-seeds/Invoke-MySqlSeeds.bat
  - scripts/tools/invoke-mysql-seeds/mysql-seeds.md
  - SddIA/tools/invoke-mysql-seeds/spec.md
  - scripts/tools-rs/Cargo.toml
  - scripts/tools-rs/install.ps1
---

## Implementación (diseño)

### Patrón base

Copiar el esqueleto de `prepare_full_env.rs`:

- Detección modo cápsula (`try_read_capsule_request`) vs CLI (`clap`).
- `normalize_legacy_args` para flags estilo `-SkipSeeds` usados por el `.bat`.
- `OutputPath` persiste el envelope `CapsuleResponse`.
- `Stdout`: obligatorio en modo cápsula; en CLI controlado por `OutputJson` (o si no hay OutputPath).

### Fases

`init → mysql → db_drop_create → migrations → seeds → done` (o `error`).

### Ejecución MySQL

- Validar contenedor `gesfer_db` listo (usar `docker inspect` health como `prepare-full-env`).
- Obtener:
  - `MYSQL_ROOT_PASSWORD` y `MYSQL_DATABASE` con `docker exec <container> printenv ...`
- Si `DropCreateDb`:
  - `docker exec <container> mysql -uroot -p<rootpwd> -e "DROP DATABASE IF EXISTS ...; CREATE DATABASE ..."`

### Migraciones EF

- `dotnet ef database update --project <efProject> --startup-project <startupProject>`

### Seeds

- Ejecutar el proyecto API con env `RUN_SEEDS_ONLY=1` (ej. `dotnet run --project <seedsProject>`).

