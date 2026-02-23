# Especificación: Estabilidad BD tras inicialización

**Feature:** estabilidad-bd-inicializacion  
**Fecha:** 2026-02-23  
**Base:** [analysis.md](./analysis.md)

---

## 1. Objetivo

Garantizar que, tras ejecutar la herramienta de inicialización (Prepare-FullEnv + Invoke-MySqlSeeds o arranque de la API), el proyecto quede estable en local con:

- Docker (MySQL, cache, Adminer) operativo
- MySQL con **estructura completa** (todas las tablas de AdminDbContext)
- **Datos de seed** (Languages, Countries, States, Cities, Companies, AdminUsers) cargados
- API Admin 100% funcional (health, auth, companies, geo)

---

## 2. Alcance

| Incluido | No incluido |
|----------|-------------|
| Nueva migración EF que cree Language, Country, State, City, PostalCode, Companies | Cambios en InitDatabase.cs (script Product) |
| RunMigrationsAndSeedsAsync aplica migraciones antes de seeds | Migraciones de otro contexto (Product) |
| Flujo de seeds completo (SeedAllAsync) en arranque y RUN_SEEDS_ONLY | Prepare-FullEnv ejecutando migraciones (solo doc) |
| Corrección de mysql-seeds-config.json (rutas proyectos) | Cambios en docker-compose o servicios Docker |

---

## 3. Requerimientos funcionales

### RF1 – Migración “AddAdminCoreAndGeoTables”

- **Descripción:** Una nueva migración de EF Core para AdminDbContext que cree las tablas faltantes respecto al modelo actual, en orden de dependencias, de forma que sea segura sobre una BD que ya pueda tener AdminUsers, AuditLogs y Logs (creadas por migraciones anteriores).
- **Comportamiento:**
  - Crear tabla **Language** (Id char(36) PK, Code, Name, Description, CreatedAt, UpdatedAt, DeletedAt, IsActive) con collation/charset coherente (utf8mb4).
  - Crear tabla **Country** (Id, Code, Name, LanguageId FK→Language, CreatedAt, UpdatedAt, DeletedAt, IsActive).
  - Crear tabla **State** (Id, Code, Name, CountryId FK→Country, CreatedAt, UpdatedAt, DeletedAt, IsActive).
  - Crear tabla **City** (Id, Name, StateId FK→State, CreatedAt, UpdatedAt, DeletedAt, IsActive).
  - Crear tabla **PostalCode** (Id, Code, CityId FK→City, CreatedAt, UpdatedAt, DeletedAt, IsActive).
  - Crear tabla **Companies** (Id, Name, Address, Email, Phone, TaxId, CityId, CountryId, StateId, LanguageId, PostalCodeId, CreatedAt, UpdatedAt, DeletedAt, IsActive; índices en Name y FKs según modelo).
  - Tipos y longitudes según AdminDbContextModelSnapshot (Guid char(36), datetime(6), tinyint(1), longtext/varchar según caso).
  - **Idempotencia:** la migración debe comprobar existencia de tablas antes de crearlas (CREATE TABLE IF NOT EXISTS o comprobación vía información del esquema) para no fallar si se re-ejecuta o si alguna tabla ya existe por otro medio.
- **Ubicación:** `src/GesFer.Admin.Back.Infrastructure/Data/Migrations/` (nombre sugerido: `YYYYMMDDHHMMSS_AddAdminCoreAndGeoTables.cs`).
- **Criterio de aceptación:** Tras `dotnet ef database update` sobre una BD que solo tiene AdminUsers, AuditLogs y Logs, existan las tablas Language, Country, State, City, PostalCode, Companies con esquema coherente al snapshot.

### RF2 – Arranque aplica migraciones y seeds completos

- **Descripción:** En cada arranque de la API (y en el flujo RUN_SEEDS_ONLY), se aplican migraciones y a continuación todos los seeds en el orden correcto.
- **Comportamiento:**
  - **RunMigrationsAndSeedsAsync:** antes de ejecutar seeds, invocar `await context.Database.MigrateAsync()`. Sustituir la ejecución de solo SeedCompaniesAsync y SeedAdminUsersAsync por **SeedAllAsync()** (Languages → Countries → States → Cities → Companies → AdminUsers).
  - **RunMigrationsAndSeedsThenExitAsync:** mantener `MigrateAsync()`; sustituir los dos seeds por **SeedAllAsync()**.
  - En ambos métodos, no tragar excepciones en silencio: registrar el error y re-lanzar (o al menos registrar con nivel Error) para que fallos de migración/seed sean visibles.
- **Touchpoint:** `src/GesFer.Admin.Back.Infrastructure/Extensions/WebAppExtensions.cs`.
- **Criterio de aceptación:** Tras arrancar la API una vez con BD vacía (solo tablas creadas por migraciones), existan datos en Language, Country, State, City y opcionalmente Companies y AdminUsers según archivos JSON de Seeds; y un segundo arranque no falle.

### RF3 – Herramienta Invoke-MySqlSeeds operativa

- **Descripción:** La herramienta Invoke-MySqlSeeds debe usar los proyectos correctos del repositorio GesFer.Admin.Back para aplicar migraciones EF y ejecutar seeds.
- **Comportamiento:**
  - Actualizar `scripts/tools/invoke-mysql-seeds/mysql-seeds-config.json`:
    - `efProject`: `"src/GesFer.Admin.Back.Infrastructure/GesFer.Admin.Back.Infrastructure.csproj"`
    - `startupProject`: `"src/GesFer.Admin.Back.Api/GesFer.Admin.Back.Api.csproj"`
  - Opcional: actualizar `seedsPath` si existe clave, a `src/GesFer.Admin.Back.Infrastructure/Data/Seeds`.
- **Criterio de aceptación:** Ejecutar `Invoke-MySqlSeeds.ps1` desde la raíz del repo con MySQL levantado (Prepare-FullEnv) debe completar sin error “Proyectos no encontrados” y aplicar migraciones y seeds.

---

## 4. Requerimientos no funcionales

- **RNF1:** Las nuevas tablas deben usar el mismo charset/collation que el resto (utf8mb4) y tipos compatibles con MySQL 8.
- **RNF2:** No eliminar ni modificar datos existentes en AdminUsers, AuditLogs o Logs al aplicar la nueva migración.
- **RNF3:** El orden de creación de tablas debe respetar FKs (Language → Country → State → City → PostalCode → Companies).

---

## 5. Riesgos y mitigación

| Riesgo | Mitigación |
|--------|------------|
| BD ya tiene tablas con nombres iguales pero distinto esquema (p. ej. de Product) | Migración idempotente (IF NOT EXISTS / comprobar existencia); documentar que Admin asume esquema propio. |
| SeedAllAsync falla por archivos JSON faltantes | AdminJsonDataSeeder ya comprueba existencia de archivos; mantener comportamiento y registrar errores. |

---

## 6. Criterios de validación (resumen)

1. BD recién inicializada (solo Docker MySQL): tras arrancar la API o ejecutar Invoke-MySqlSeeds, existan las tablas Language, Country, State, City, PostalCode, Companies, AdminUsers, AuditLogs, Logs y __EFMigrationsHistory.
2. GET /health responde 200.
3. Endpoints de geo (países, estados, ciudades) y de companies responden sin error de tabla inexistente.
4. Login con usuario seed funciona (AdminUsers con datos).
5. Invoke-MySqlSeeds.ps1 termina con éxito usando mysql-seeds-config.json actualizado.

---

## 7. Referencias

- Análisis: [analysis.md](./analysis.md), [analysis.json](./analysis.json)
- Modelo: `src/GesFer.Admin.Back.Infrastructure/Data/Migrations/AdminDbContextModelSnapshot.cs`
- Seeds: `src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs` (SeedAllAsync)
- Contrato procesos: SddIA/process (artefactos .md + .json)
