---
feature: invoke-mysql-seeds-drop-create-db
date: 2026-04-28
status: draft
scope:
  toolId: invoke-mysql-seeds
  pattern_source: prepare-full-env
requirements:
  - id: RF1
    title: Ejecutar drop/create DB (estrategia B)
  - id: RF2
    title: Migraciones EF Core (estructura)
  - id: RF3
    title: Seeds Admin (RUN_SEEDS_ONLY)
  - id: RF4
    title: Ejecución conjunta o separada (flags)
  - id: RF5
    title: Wrapper .bat ejecuta ambas acciones por defecto
---

## Contexto

`prepare-full-env` define el patrón de tool (Rust + feedback por fases + wrapper `.bat` + soporte legacy flags). `invoke-mysql-seeds` debe alinearse al mismo patrón y ser operativa (hoy el binario Rust está en `NOT_IMPLEMENTED`).

## Requerimientos funcionales

### RF1 — Drop/Create DB (estrategia B)

- **Comportamiento:** si está habilitado, ejecutar:
  - `DROP DATABASE IF EXISTS <db>`
  - `CREATE DATABASE <db> CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci`
- **Origen de configuración:** la BD objetivo **ya está definida** (SSOT). Se obtiene desde el contenedor MySQL (`MYSQL_DATABASE`) y/o del connection string existente (no se introduce nueva configuración).
- **Credenciales:** se obtiene `MYSQL_ROOT_PASSWORD` desde el contenedor para ejecutar el SQL como `root`.

### RF2 — Migraciones EF (estructura)

- Ejecutar `dotnet ef database update` usando `efProject` y `startupProject` del `mysql-seeds-config.json`.

### RF3 — Seeds (RUN_SEEDS_ONLY)

- Ejecutar el proyecto API definido por `seedsProject` con `RUN_SEEDS_ONLY=1` para que aplique migraciones+seeds y termine.

### RF4 — Ejecución conjunta o separada

Inputs (CLI + envelope JSON), siguiendo el patrón `prepare-full-env`:

- `DropCreateDb` (default **true** desde `.bat`)
- `SkipMigrations`
- `SkipSeeds`
- `ConfigPath`
- `OutputPath`
- `OutputJson`

### RF5 — Wrapper `.bat`

- `Invoke-MySqlSeeds.bat` debe ejecutar el `.exe` y por defecto aplicar estrategia B (drop/create) y luego migraciones+seeds.

## Fases (feedback)

`init → mysql → db_drop_create → migrations → seeds → done` (o `error`).

