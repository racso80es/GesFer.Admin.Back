# Análisis de situación: Estabilidad BD tras inicialización

**Fecha:** 2026-02-23  
**Objetivo:** Dejar estable en local el proyecto (Docker, MySQL con estructura y datos, API 100% funcional).  
**Situación observada:** En BD solo existe la tabla `Logs`; faltan `Companies`, `AdminUsers` y tablas de geolocalización (`Country`, `State`, `City`, `Language`, `PostalCode`).

---

## 1. Resumen ejecutivo

El proyecto **GesFer.Admin.Back** fue diseñado para compartir base de datos con otro contexto (Product/ApplicationDbContext). Las migraciones de **AdminDbContext** solo crean parte del esquema (AdminUsers, AuditLogs, Logs) y **asumen** que el resto de tablas (Companies, geo, etc.) ya existen. Además, el arranque normal de la API **no aplica migraciones**, solo ejecuta seeds. La herramienta de seeds (Invoke-MySqlSeeds) usa rutas de proyectos incorrectas para este repositorio. El resultado es una BD que puede quedar solo con `Logs` tras ciertos flujos de inicialización.

---

## 2. Estado actual del esquema y migraciones

### 2.1 AdminDbContext y modelo

- **Ubicación:** `src/GesFer.Admin.Back.Infrastructure/Data/AdminDbContext.cs`
- **DbSets:** AdminUsers, AuditLogs, Logs, Companies, Cities, States, Countries, Languages, PostalCodes.
- **ModelSnapshot** incluye todas estas entidades y sus relaciones (Language → Country → State → City → PostalCode; Company con FKs a City, Country, State, Language, PostalCode).

### 2.2 Migraciones existentes (orden de aplicación)

| Migración | Tablas/acciones |
|-----------|------------------|
| `20260213154125_InitialAdmin` | Crea **AdminUsers**, **AuditLogs**. Comentario en código: *"Companies, Logs, etc. ya existen por las migraciones de Product (ApplicationDbContext)."* |
| `20260214110000_CreateLogsTableIfNotExists` | Crea tabla **Logs** (SQL raw, idempotente). |
| `20260214120000_AddMissingColumnsToLogs` | Añade columnas Source, CompanyId, UserId a **Logs** (idempotente). |

**Conclusión:** No existe ninguna migración en Admin que cree: **Companies**, **Country**, **State**, **City**, **Language**, **PostalCode**. Si la BD se crea solo con migraciones de Admin, el resultado es únicamente: AdminUsers, AuditLogs, Logs.

---

## 3. Flujo de arranque de la API y seeds

### 3.1 Program.cs

- Si `RUN_SEEDS_ONLY=1`: se llama `RunMigrationsAndSeedsThenExitAsync()` → **sí aplica** `context.Database.MigrateAsync()` y luego SeedCompaniesAsync + SeedAdminUsersAsync.
- Arranque normal (línea ~188): se llama `RunMigrationsAndSeedsAsync()` → **no** llama a `MigrateAsync()`; solo ejecuta `SeedCompaniesAsync()` y `SeedAdminUsersAsync()`.

### 3.2 WebAppExtensions.cs

- **RunMigrationsAndSeedsAsync:** no aplica migraciones; solo seeds. Cualquier fallo en seeds se traga en `catch (Exception)` sin re-lanzar.
- **RunMigrationsAndSeedsThenExitAsync:** aplica migraciones y luego los mismos dos seeds (Companies, AdminUsers).

Por tanto, en arranque normal **las tablas solo se crean** si en algún momento se ejecutó el flujo con `RUN_SEEDS_ONLY=1` o si se ejecutó `dotnet ef database update` manualmente. Y aun así, las migraciones actuales no crean Companies ni geo.

---

## 4. Seeds

### 4.1 AdminJsonDataSeeder

- **SeedAllAsync:** orden correcto: Languages → Countries → States → Cities → Companies → AdminUsers.
- **RunMigrationsAndSeedsAsync / RunMigrationsAndSeedsThenExitAsync** solo invocan **SeedCompaniesAsync** y **SeedAdminUsersAsync** (no Languages, Countries, States, Cities). Por tanto, incluso con todas las tablas creadas, el flujo actual no siembra geo ni languages; Companies y AdminUsers dependen de esos datos (FKs).

### 4.2 Dependencias entre seeds

- Companies depende de City, Country, State, Language, PostalCode (opcionales pero usados).
- Countries depende de Language.
- States depende de Country; Cities depende de State; PostalCodes depende de City.

Si las tablas de geo no existen o están vacías, SeedCompaniesAsync y SeedAdminUsersAsync pueden fallar o insertar datos incompletos.

---

## 5. Herramientas de inicialización

### 5.1 Prepare-FullEnv (scripts/tools/prepare-full-env/)

- Levanta Docker (MySQL, cache, Adminer), espera a MySQL y opcionalmente inicia la API en background.
- **No aplica migraciones ni ejecuta seeds.** Solo deja el contenedor MySQL vacío (o con el estado previo del volumen).

### 5.2 Invoke-MySqlSeeds (scripts/tools/invoke-mysql-seeds/)

- Debería aplicar `dotnet ef database update` y luego seeds vía `RUN_SEEDS_ONLY=1`.
- **mysql-seeds-config.json** apunta a proyectos que **no existen** en este repo:
  - `efProject`: `"src/Infrastructure/GesFer.Admin.Infra.csproj"` → en el repo actual es `src/GesFer.Admin.Back.Infrastructure/GesFer.Admin.Back.Infrastructure.csproj`
  - `startupProject`: `"src/Api/GesFer.Admin.Api.csproj"` → en el repo actual es `src/GesFer.Admin.Back.Api/GesFer.Admin.Back.Api.csproj`
- Si se ejecuta Invoke-MySqlSeeds, falla en “No se encuentran proyectos EF o Api” y no se aplican migraciones ni seeds.

### 5.3 InitDatabase (src/scripts/InitDatabase.cs)

- Usa **ApplicationDbContext** (proyecto Product), no AdminDbContext.
- Borra `__EFMigrationsHistory` y varias tablas (Companies, Cities, States, Countries, etc.) y luego aplica migraciones de **ApplicationDbContext**. No es la API Admin ni AdminDbContext; en este repositorio Admin.Back ese script no tiene el tipo ApplicationDbContext definido (pertenece a otro producto/solución).

---

## 6. Causas raíz de “solo existe la tabla Logs”

1. **Migraciones de Admin no crean Companies ni geo:** solo AdminUsers, AuditLogs y Logs.
2. **Arranque normal de la API no aplica migraciones:** solo ejecuta dos seeds; si las tablas no existen, los seeds fallan y el error se silencia.
3. **Invoke-MySqlSeeds con rutas incorrectas:** no encuentra proyectos y no aplica migraciones ni seeds.
4. **Prepare-FullEnv no toca la BD:** solo levanta Docker y opcionalmente la API.
5. La tabla **Logs** puede ser la única presente porque la migración `CreateLogsTableIfNotExists` se aplicó en algún momento (p. ej. ejecución manual de `ef database update` o un arranque previo con RUN_SEEDS_ONLY que aplicó migraciones hasta Logs), o porque Serilog/MySQL sink creó la tabla según su configuración.

---

## 7. Validaciones recomendadas con la API levantada

- `GET /health` → comprobar que la API responde.
- Comprobar en MySQL (Adminer o cliente) las tablas existentes: `SHOW TABLES;`
- Si existen tablas: comprobar `__EFMigrationsHistory` para ver qué migraciones se aplicaron.
- Probar un endpoint que use Companies (p. ej. listar empresas) y uno de geo (p. ej. países) para reproducir 404 o error de BD por tablas faltantes.

---

## 8. Recomendaciones para garantizar el resultado esperado

1. **Añadir migraciones en Admin** que creen todas las tablas del modelo actual (Language, Country, State, City, PostalCode, Companies) de forma idempotente o en orden de dependencias, para que el proyecto Admin sea autónomo y no dependa de Product.
2. **RunMigrationsAndSeedsAsync:** llamar a `context.Database.MigrateAsync()` antes de ejecutar seeds, para que cada arranque deje la BD al día.
3. **Seeds en arranque:** usar `SeedAllAsync()` (o al menos incluir Languages → Countries → States → Cities antes de Companies y AdminUsers) en RunMigrationsAndSeedsAsync y RunMigrationsAndSeedsThenExitAsync.
4. **Invoke-MySqlSeeds:** actualizar `mysql-seeds-config.json` con las rutas correctas del repo: `GesFer.Admin.Back.Infrastructure` y `GesFer.Admin.Back.Api`.
5. **Prepare-FullEnv (opcional):** documentar que tras levantar el entorno hay que ejecutar Invoke-MySqlSeeds (o equivalente) para aplicar migraciones y seeds, o integrar un paso que lo invoque.
6. **InitDatabase:** si se usa solo para Admin.Back, sustituir ApplicationDbContext por AdminDbContext y aplicar migraciones de Admin; si es compartido con Product, mantenerlo en su contexto y no usarlo como única fuente de verdad para Admin.

Con estos cambios, la secuencia “herramienta de inicialización + arranque API” dejará la BD con todas las tablas y datos necesarios para que la API funcione al 100%.
