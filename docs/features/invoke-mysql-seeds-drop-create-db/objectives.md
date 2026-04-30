---
feature: invoke-mysql-seeds-drop-create-db
date: 2026-04-28
owner: tekton-developer
status: draft
---

## Objetivo

Convertir `invoke-mysql-seeds` en una herramienta operativa (Rust `.exe`) que permita:

- Crear la **base de datos** objetivo de MySQL si no existe y generar la **estructura** (migraciones EF Core).
- Insertar **seeds** de Admin.
- Ejecutar ambas acciones **juntas** (default) o **separadas** (skip flags).
- Si existen datos previos, **eliminarlos** aplicando **estrategia B**: `DROP DATABASE IF EXISTS` + `CREATE DATABASE` antes de migraciones/seeds.
- Asegurar que el wrapper `.bat` quede configurado para ejecutar **ambas acciones** (estructura + seeds) usando estrategia B por defecto.

## Alcance

- Implementación del binario `scripts/tools/invoke-mysql-seeds/invoke_mysql_seeds.exe` (copia desde `scripts/tools-rs`).
- Actualización de documentación y contrato de inputs en `SddIA/tools/invoke-mysql-seeds/spec.md` y `scripts/tools/invoke-mysql-seeds/mysql-seeds.md`.
- Ajuste del wrapper `scripts/tools/invoke-mysql-seeds/Invoke-MySqlSeeds.bat` para comportamiento default requerido.

## Fuera de alcance

- Cambios en `docker-compose.yml` o en la configuración existente de conexión (SSOT).
- Cambios funcionales en el seeder de la API más allá de ser invocado por la tool.

