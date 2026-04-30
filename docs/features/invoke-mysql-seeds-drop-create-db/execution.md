---
feature: invoke-mysql-seeds-drop-create-db
date: 2026-04-28
status: draft
changes:
  - Implementación real del binario Rust invoke_mysql_seeds.rs (patrón prepare-full-env)
  - Nuevo flag DropCreateDb (estrategia B) y fase feedback db_drop_create
  - Wrapper .bat forzado a ejecutar DropCreateDb por defecto
---

## Ejecución

Se implementa `scripts/tools-rs/src/bin/invoke_mysql_seeds.rs` siguiendo el patrón de `prepare_full_env.rs`:

- Modo cápsula (stdin JSON) y modo CLI (clap).
- Normalización de flags legacy para wrapper `.bat`.
- Feedback por fases: `init`, `mysql`, `db_drop_create`, `migrations`, `seeds`, `done/error`.

El reset requerido se implementa como **estrategia B**:

- Lee `MYSQL_DATABASE` y `MYSQL_ROOT_PASSWORD` desde el contenedor `gesfer_db`.
- Ejecuta `DROP DATABASE IF EXISTS` + `CREATE DATABASE` (utf8mb4/utf8mb4_unicode_ci).

Se actualiza documentación y spec para reflejar el nuevo input `DropCreateDb` y el nuevo flujo.

