# Implementación: Estabilidad BD tras inicialización

**Feature:** estabilidad-bd-inicializacion  
**Fecha:** 2026-02-23  
**Base:** [plan.md](./plan.md), [clarify.md](./clarify.md)

---

## 1. Resumen

Se implementaron las tres fases del plan: migración AddAdminCoreAndGeoTables (RF1), WebAppExtensions con MigrateAsync + SeedAllAsync + log + rethrow (RF2), y actualización de mysql-seeds-config.json (RF3).

---

## 2. Touchpoints realizados

### 2.1 Fase 1 – Migración AddAdminCoreAndGeoTables (RF1)

| Archivo | Acción |
|---------|--------|
| `src/GesFer.Admin.Back.Infrastructure/Data/Migrations/20260223100000_AddAdminCoreAndGeoTables.cs` | **Creado.** Up() con SQL raw `CREATE TABLE IF NOT EXISTS` para Language, Country, State, City, PostalCode, Companies (orden de dependencias). Down() con `DROP TABLE IF EXISTS` en orden inverso. Charset utf8mb4; FKs con ON DELETE CASCADE (geo) y RESTRICT (Companies). |
| `src/GesFer.Admin.Back.Infrastructure/Data/Migrations/20260223100000_AddAdminCoreAndGeoTables.Designer.cs` | **Creado.** Partial class con [Migration("20260223100000_AddAdminCoreAndGeoTables")] y BuildTargetModel idéntico al AdminDbContextModelSnapshot actual para que el siguiente `ef migrations add` no genere diff. |

### 2.2 Fase 2 – WebAppExtensions (RF2)

| Archivo | Acción |
|---------|--------|
| `src/GesFer.Admin.Back.Infrastructure/Extensions/WebAppExtensions.cs` | **Modificado.** RunMigrationsAndSeedsAsync: resolver ILoggerFactory, crear logger con categoría "GesFer.Admin.Back.Infrastructure.WebAppExtensions"; llamar MigrateAsync() antes de seeds; sustituir SeedCompaniesAsync + SeedAdminUsersAsync por SeedAllAsync(); catch con LogError + rethrow. RunMigrationsAndSeedsThenExitAsync: mismo logger; MigrateAsync(); SeedAllAsync(); try/catch con LogError + rethrow; Environment.Exit(0) solo tras éxito. Añadido using Microsoft.Extensions.Logging. |

### 2.3 Fase 3 – mysql-seeds-config.json (RF3)

| Archivo | Acción |
|---------|--------|
| `scripts/tools/invoke-mysql-seeds/mysql-seeds-config.json` | **Modificado.** efProject → src/GesFer.Admin.Back.Infrastructure/GesFer.Admin.Back.Infrastructure.csproj; startupProject → src/GesFer.Admin.Back.Api/GesFer.Admin.Back.Api.csproj; seedsPath → src/GesFer.Admin.Back.Infrastructure/Data/Seeds. Formato JSON expandido para legibilidad. |

---

## 3. Validación

- **Compilación:** `dotnet build src/GesFer.Admin.Back.Api/GesFer.Admin.Back.Api.csproj` → correcta.
- **Migración:** Pendiente de ejecutar `dotnet ef database update` (o arranque API / Invoke-MySqlSeeds) con MySQL levantado para validar creación de tablas y seeds.
- **Invoke-MySqlSeeds:** Pendiente de ejecutar script con MySQL up para validar RF3.

---

## 4. Referencias

- Spec: [spec.md](./spec.md)
- Clarify: [clarify.md](./clarify.md), [clarify.json](./clarify.json)
- Plan: [plan.md](./plan.md), [plan.json](./plan.json)
