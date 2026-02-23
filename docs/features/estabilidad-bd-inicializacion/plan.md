# Plan: Estabilidad BD tras inicialización

**Feature:** estabilidad-bd-inicializacion  
**Fecha:** 2026-02-23  
**Base:** [spec.md](./spec.md), [clarify.md](./clarify.md)

---

## 1. Orden de ejecución

| Fase | Tarea | Touchpoint | RF |
|------|--------|------------|-----|
| 1 | Crear migración AddAdminCoreAndGeoTables (SQL IF NOT EXISTS) | Infrastructure/Data/Migrations/ | RF1 |
| 2 | WebAppExtensions: MigrateAsync + SeedAllAsync + log + rethrow | Infrastructure/Extensions/WebAppExtensions.cs | RF2 |
| 3 | Actualizar mysql-seeds-config.json | scripts/tools/invoke-mysql-seeds/mysql-seeds-config.json | RF3 |

---

## 2. Detalle por tarea

### 2.1 Fase 1 – Migración AddAdminCoreAndGeoTables

- **Acción:** Añadir archivo de migración en `src/GesFer.Admin.Back.Infrastructure/Data/Migrations/` con nombre `20260223100000_AddAdminCoreAndGeoTables.cs` (o timestamp coherente con convención del proyecto).
- **Contenido Up():**
  - Ejecutar SQL raw con `CREATE TABLE IF NOT EXISTS` para cada tabla, en orden: **Language** → **Country** → **State** → **City** → **PostalCode** → **Companies**.
  - Columnas y tipos según AdminDbContextModelSnapshot (char(36) para Guid, datetime(6), tinyint(1), longtext/varchar según caso); charset utf8mb4; FKs e índices según modelo.
- **Contenido Down():** DROP TABLE en orden inverso (Companies, PostalCode, City, State, Country, Language), con `IF EXISTS` si se desea mayor seguridad.
- **Designer:** Generar o copiar desde snapshot actual para que EF reconozca la migración (si se usa `dotnet ef migrations add`, editar después el Up/Down; si se escribe a mano, añadir clase partial con atributo Migration y Designer que refleje el mismo modelo).
- **Validación:** `dotnet ef database update` sobre BD con solo AdminUsers, AuditLogs, Logs → deben aparecer las 6 tablas nuevas.

### 2.2 Fase 2 – WebAppExtensions

- **RunMigrationsAndSeedsAsync:**
  1. Crear scope y resolver AdminDbContext, AdminJsonDataSeeder, **ILogger&lt;WebAppExtensions&gt;**.
  2. `await context.Database.MigrateAsync();`
  3. `await seeder.SeedAllAsync();`
  4. En catch: `logger.LogError(ex, "Error en migraciones o seeds: {Message}", ex.Message); throw;`
- **RunMigrationsAndSeedsThenExitAsync:**
  1. Resolver context, seeder, **ILogger&lt;WebAppExtensions&gt;**.
  2. Mantener `await context.Database.MigrateAsync();`
  3. Sustituir llamadas a SeedCompaniesAsync/SeedAdminUsersAsync por `await seeder.SeedAllAsync();`
  4. Envolver en try/catch: en catch, log Error + rethrow; solo llamar `Environment.Exit(0)` si todo fue correcto.
- **Validación:** Arranque con BD vacía (solo tablas de migraciones) → datos en Language, Country, State, City, Companies, AdminUsers; segundo arranque sin fallo.

### 2.3 Fase 3 – mysql-seeds-config.json

- **Acción:** Editar `scripts/tools/invoke-mysql-seeds/mysql-seeds-config.json`.
- **Cambios:**
  - `efProject`: `"src/GesFer.Admin.Back.Infrastructure/GesFer.Admin.Back.Infrastructure.csproj"`
  - `startupProject`: `"src/GesFer.Admin.Back.Api/GesFer.Admin.Back.Api.csproj"`
  - `seedsPath`: `"src/GesFer.Admin.Back.Infrastructure/Data/Seeds"`
- **Validación:** Ejecutar `Invoke-MySqlSeeds.ps1` desde raíz con MySQL levantado → sin error “Proyectos no encontrados”; migraciones y seeds aplicados.

---

## 3. Prerequisitos

- Archivos de seed presentes en `src/GesFer.Admin.Back.Infrastructure/Data/Seeds/`: languages.json, countries.json, states.json, cities.json, companies.json, admin-users.json (según clarify, son prerequisito del proyecto).

---

## 4. Criterios de validación (resumen)

1. Migración aplicada: tablas Language, Country, State, City, PostalCode, Companies creadas.
2. Arranque API aplica migraciones y SeedAllAsync; errores no silenciados.
3. Invoke-MySqlSeeds.ps1 exitoso con config actualizada.
4. GET /health 200; endpoints geo y companies operativos; login con usuario seed posible.
