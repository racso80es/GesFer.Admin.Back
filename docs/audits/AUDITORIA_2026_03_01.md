# Auditoría S+ - Guardián de la Infraestructura
**Fecha:** 2026-03-01 (UTC)

## 1. Métricas de Salud (0-100%)
Arquitectura: 95% | Nomenclatura: 100% | Estabilidad Async: 100%

## 2. Pain Points (🔴 Críticos / 🟡 Medios)

Hallazgo: [🟡 Medio] El atributo `AuthorizeSystemOrAdminAttribute` requiere `IConfiguration` resuelto en tiempo de ejecución de cada petición usando Service Locator (`GetRequiredService`). Además no tiene un middleware intermedio que lo intercepte limpiamente.
Ubicación: `src/GesFer.Admin.Back.Api/Attributes/AuthorizeSystemOrAdminAttribute.cs`, línea 14

Hallazgo: [🟡 Medio] En `AdminJsonDataSeeder.cs`, al instanciar `Company`, el código convierte explícitamente `Guid.Parse` con cadenas que pueden ser nulas o vacías (`string.IsNullOrWhiteSpace(companyData.LanguageId) ? null : Guid.Parse(companyData.LanguageId)`). Podría causar excepciones no controladas si el seeder de `LanguageId` tiene un formato inválido.
Ubicación: `src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs`, línea 549

## 3. Acciones Kaizen (Hoja de Ruta para el Executor)

### Acción 1: Refactorización de `AuthorizeSystemOrAdminAttribute`

**Instrucciones para el Executor:**
1. Cambiar el uso del ServiceLocator en `AuthorizeSystemOrAdminAttribute` para depender de inyección de dependencias o usar un IAsyncAuthorizationFilter inyectado de forma adecuada en la configuración de la API.
2. Extraer el string constante `SharedSecret` y usar `IOptions` de estar disponible.

**Definition of Done (DoD):**
* El proyecto compila y los test pasan sin problema.
* El atributo `AuthorizeSystemOrAdminAttribute` sigue bloqueando solicitudes no autorizadas de acuerdo al esquema esperado.
* `dotnet test` se ejecuta sin errores en UnitTests, E2ETests, y IntegrationTests.

### Acción 2: Parseo seguro en `AdminJsonDataSeeder`

**Instrucciones para el Executor:**
1. Modificar el bloque de `SeedCompaniesAsync` en `AdminJsonDataSeeder.cs` para utilizar `Guid.TryParse` al procesar `LanguageId` en las companies.
2. Lanzar log de advertencia si no parsea correctamente y asignar `null`.

**Definition of Done (DoD):**
* El proyecto compila.
* Las pruebas pasan.
* No hay riesgos de FormatException silenciosos durante el seeding.
