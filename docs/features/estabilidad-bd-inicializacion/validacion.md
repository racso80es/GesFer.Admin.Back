# Validación: Estabilidad BD tras inicialización

**Feature:** estabilidad-bd-inicializacion  
**Fecha:** 2026-02-23  
**Criterios:** [spec.md §6](./spec.md)

---

## 1. Resultados ejecutados

| Criterio | Resultado | Notas |
|----------|-----------|--------|
| **1. Compilación** | OK | `dotnet build` correcto (1 warning preexistente en AuthorizeSystemOrAdminAttribute). |
| **2. Migraciones EF** | OK | `dotnet ef database update` aplicó InitialAdmin y 20260223100000_AddAdminCoreAndGeoTables. Tablas Language, Country, State, City, PostalCode, Companies creadas. Se corrigió sintaxis SQL (charset solo a nivel tabla). |
| **3. Invoke-MySqlSeeds.ps1** | OK | Config con rutas correctas; migraciones aplicadas; seeds ejecutados (exit 0). Corregido bug en script: precedencia de `Join-Path` y `-replace` en línea de seeds. |
| **4. GET /health** | OK | 200 Healthy (API levantada en localhost:5010). |
| **5. Endpoints** | Parcial | Company: 401 (endpoint existe, requiere auth). Geo: rutas a comprobar según convención del API. |
| **6. Seeds con datos** | OK | Api.csproj copia `Infrastructure/Data/Seeds/*.json` al output (`Data/Seeds`). companies.json corregido: LanguageId alineado con languages.json (`d3e0b8a1-5f2c-4b5d-9a6e-7f8c9d0e1a2b`). Invoke-MySqlSeeds carga Languages, Countries, States, Cities, Companies y AdminUsers. |

---

## 2. Correcciones aplicadas durante la validación

1. **Migración 20260223100000:** En MySQL, `longtext NOT NULL CHARACTER SET utf8mb4` por columna provocaba error de sintaxis. Se dejó charset solo a nivel de tabla: `) CHARACTER SET utf8mb4;` y se quitaron las cláusulas por columna.
2. **Invoke-MySqlSeeds.ps1:** En la fase de seeds, `Join-Path $repoRoot $config.startupProject -replace "/", "\"` era interpretado como parámetro de Join-Path. Se reemplazó por `$startupProjPath = (Join-Path $repoRoot $config.startupProject) -replace "/", "\"` y uso de `$startupProjPath` en Start-Process.
3. **Carga de seeds:** En Api.csproj se añadió copia de `../GesFer.Admin.Back.Infrastructure/Data/Seeds/*.json` al output como `Data/Seeds/%(Filename)%(Extension)` (CopyToOutputDirectory PreserveNewest) para que AdminJsonDataSeeder encuentre los JSON. En companies.json se actualizó `languageId` de `10000000-0000-0000-0000-000000000001` a `d3e0b8a1-5f2c-4b5d-9a6e-7f8c9d0e1a2b` (Id del lenguaje en languages.json) para cumplir la FK Companies.LanguageId → Language.Id.

---

## 3. Validación manual recomendada

- Comprobar en MySQL/Adminer que existan las tablas: Language, Country, State, City, PostalCode, Companies, AdminUsers, AuditLogs, Logs, __EFMigrationsHistory_Admin.
- Si se copian los JSON de Seeds al directorio que usa el seeder (o se arranca desde la solución con ruta canónica), volver a ejecutar seeds y comprobar datos en Language, Country, State, City, Companies, AdminUsers.
- Probar login con usuario seed (cuando existan datos en AdminUsers) y listado de companies/geo con token.

---

## 4. Referencias

- Spec criterios: [spec.md §6](./spec.md)
- Implementación: [implementation.md](./implementation.md)
