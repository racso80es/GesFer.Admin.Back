# Clarificación: Estabilidad BD tras inicialización

**Feature:** estabilidad-bd-inicializacion  
**Fecha:** 2026-02-23  
**Base:** [spec.md](./spec.md)

---

## 1. Gaps resueltos

### 1.1 RF1 – Idempotencia de la migración

**Gap:** La spec pide migración idempotente (no fallar si la tabla ya existe). Las migraciones EF estándar usan `migrationBuilder.CreateTable()`, que falla si la tabla existe.

**Resolución:** Implementar el `Up()` de la nueva migración con **SQL raw** usando `CREATE TABLE IF NOT EXISTS` para cada tabla (Language, Country, State, City, PostalCode, Companies), en ese orden. Así la migración solo se ejecuta una vez (registrada en `__EFMigrationsHistory`), pero si por cualquier motivo la tabla ya existiera (p. ej. creada por otro contexto o script), no falla. El `Down()` puede dejar las tablas (no hacer DROP) para evitar borrar datos que pudieran ser compartidos, o documentar que la reversión manual es opcional; se opta por **Down() que hace DROP TABLE** solo de las tablas creadas por esta migración, en orden inverso (Companies, PostalCode, City, State, Country, Language), asumiendo que esta migración es la única propietaria de esas tablas en el flujo Admin.

**Decisión:** Down() con DROP TABLE en orden inverso; en entornos donde Product pudiera haber creado las mismas tablas, no se ejecutará Down() salvo rollback explícito.

---

### 1.2 RF2 – Registro de excepciones en WebAppExtensions

**Gap:** La spec pide no tragar excepciones en silencio y registrar el error. Actualmente `RunMigrationsAndSeedsAsync` hace `catch (Exception)` sin log ni re-lanzar. Los métodos de extensión reciben `IServiceProvider`, no `ILogger`.

**Resolución:** Resolver `ILogger` desde el scope: `var logger = services.GetRequiredService<ILogger<WebAppExtensions>>();`. En el `catch`: registrar con `logger.LogError(ex, "Error en migraciones o seeds: {Message}", ex.Message);` y **re-lanzar** la excepción (`throw;`) para que el arranque falle de forma visible. Aplicar el mismo patrón en `RunMigrationsAndSeedsThenExitAsync` (try/catch con log + rethrow antes de `Environment.Exit(0)`), de modo que si MigrateAsync o SeedAllAsync fallan, se registre y se propague (y el proceso salga con código de error al no llegar a Exit(0)).

**Decisión:** Log Error + rethrow en ambos métodos. Program.cs ya tiene un try/catch global que registrará el fallo; el rethrow garantiza que el host no continúe como si todo hubiera ido bien.

---

### 1.3 RF2 – SeedAllAsync y archivos JSON opcionales

**Gap:** SeedAllAsync llama a SeedLanguagesAsync, SeedCountriesAsync, etc. Cada método comprueba si existe el archivo JSON; si no existe, devuelve sin error. Si falta `languages.json`, Languages no se carga y Countries (que depende de LanguageId) puede fallar al insertar.

**Resolución:** No cambiar la lógica de AdminJsonDataSeeder: ya comprueba existencia de archivos y no lanza si el archivo falta (devuelve resultado sin entidades cargadas). Para cumplir el objetivo de “estabilidad”, los **archivos de seed deben existir** en `src/GesFer.Admin.Back.Infrastructure/Data/Seeds/` (languages.json, countries.json, states.json, cities.json, companies.json, admin-users.json). Se documenta en plan/validación como prerequisito. Si en runtime falta un JSON necesario para la cadena (p. ej. languages), el seed fallará al insertar en la siguiente tabla; el log + rethrow (ver 1.2) hará visible el fallo.

**Decisión:** Mantener comportamiento actual del seeder; considerar los JSON de Seeds como parte del entregable del proyecto; no añadir comprobación explícita de “todos los JSON obligatorios” en esta feature.

---

### 1.4 RF3 – Rutas en mysql-seeds-config.json

**Gap:** La spec indica actualizar `efProject` y `startupProject`; opcionalmente `seedsPath`. En el repo las rutas usan barras `/` en config (Invoke-MySqlSeeds.ps1 hace `-replace "/", "\"` para Windows).

**Resolución:** Usar rutas con **slash** (`src/GesFer.Admin.Back.Infrastructure/...`) para mantener consistencia con el JSON actual y con el script que las normaliza. Actualizar también `seedsPath` a `src/GesFer.Admin.Back.Infrastructure/Data/Seeds` para documentación y posibles usos futuros del tool.

**Decisión:** Actualizar efProject, startupProject y seedsPath.

---

## 2. Resumen de decisiones

| Tema | Decisión |
|------|----------|
| Idempotencia migración | Up() con SQL CREATE TABLE IF NOT EXISTS; Down() con DROP TABLE en orden inverso |
| Excepciones en WebAppExtensions | Resolver ILogger del scope; LogError + rethrow en ambos métodos |
| SeedAllAsync y JSON faltantes | No cambiar seeder; Seeds son prerequisito; fallo visible por rethrow |
| Rutas en mysql-seeds-config | Slash; actualizar efProject, startupProject y seedsPath |

---

## 3. Referencias

- Spec: [spec.md](./spec.md), [spec.json](./spec.json)
- Análisis: [analysis.md](./analysis.md)
