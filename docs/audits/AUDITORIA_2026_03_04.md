# AUDITORIA_2026_03_04.md

## 1. Métricas de Salud (0-100%)
Arquitectura: 95% | Nomenclatura: 100% | Estabilidad Async: 100%

*   **Arquitectura:** El proyecto compila y los tests pasan, demostrando buena integridad estructural inicial ("The Wall"). Existe una buena segregación por capas (Domain, Application, Infrastructure, Api) con uso adecuado de MediatR para orquestación. Sin embargo, hay un problema en la API donde se están instanciando objetos tipo DTO (ejemplo en Handlers o Controllers) en lugar de utilizar Automapper u otro mecanismo (aunque se permite en consultas sencillas, en los Controllers no deben alojarse responsabilidades que correspondan a la capa de abstracción o mapeo, aunque en nuestra revisión, los DTOs están ubicados en Application/DTOs lo cual es correcto). Un problema estructural crítico es que `GesFer.Admin.Back.Api` tiene una referencia NuGet a `Microsoft.EntityFrameworkCore.Design` y el uso directo en `Program.cs` para el `DbContext` aunque la inyección está abstraída, hay un punto de acoplamiento. También hay un problema: el API tiene referencia a la capa de `Infrastructure` explícita para la inyección de dependencias (`DependencyInjection.cs`). Aunque la regla dicta que puede referenciar a `Infrastructure` para inyección de dependencias, *NO* debe referenciar paquetes NuGet específicos de implementación en el API si es posible. Un análisis de `GesFer.Admin.Back.Api.csproj` revela una dependencia NuGet sobre `Microsoft.EntityFrameworkCore.Design`. Se evaluará como pain point.
*   **Nomenclatura:** Todos los namespaces y rutas parecen estar conformes con `GesFer.Admin.Back.*`.
*   **Estabilidad Async:** 100%. No se ha encontrado ninguna instancia de `async void`, `.Result`, ni `.Wait()`.

## 2. Pain Points (🔴 Críticos / 🟡 Medios)

🟡 **Hallazgo:** Acoplamiento de Paquete NuGet de Infraestructura en capa API.
*   **Descripción:** El proyecto `GesFer.Admin.Back.Api` tiene una referencia directa al paquete NuGet `Microsoft.EntityFrameworkCore.Design`. Los paquetes relacionados con la tecnología de base de datos o herramientas de Entity Framework deben estar confinados a la capa `Infrastructure`. `Api` solo debe interactuar con abstracciones de la capa `Application` y configurar inyección vía extensión method.
*   **Ubicación:** `src/GesFer.Admin.Back.Api/GesFer.Admin.Back.Api.csproj`, línea 12.

🟡 **Hallazgo:** Instanciación manual de DTOs en Handlers.
*   **Descripción:** En varios Handlers de la capa `Application` (ej. `GetAllCompaniesHandler.cs`, `GetCompanyByIdHandler.cs`, `GetCompanyByNameHandler.cs`, `GeoHandlers.cs`), se están instanciando manualmente los DTOs en consultas LINQ (`.Select(c => new CompanyDto...)`). Aunque no es un error de compilación o de ejecución crítico, representa una oportunidad de mejora para desacoplar el mapeo (Kaizen).
*   **Ubicación:**
    * `src/GesFer.Admin.Back.Application/Handlers/Company/GetAllCompaniesHandler.cs`, línea 23.
    * `src/GesFer.Admin.Back.Application/Handlers/Company/GetCompanyByIdHandler.cs`, línea 22.
    * `src/GesFer.Admin.Back.Application/Handlers/Company/GetCompanyByNameHandler.cs`, línea 29.
    * `src/GesFer.Admin.Back.Application/Handlers/Geo/GeoHandlers.cs`, múltiples líneas.

## 3. Acciones Kaizen (Hoja de Ruta para el Executor)

**Acción 1: Desacoplar `Microsoft.EntityFrameworkCore.Design` de la capa API**
*   **Instrucciones:** Mover el paquete `Microsoft.EntityFrameworkCore.Design` del proyecto `GesFer.Admin.Back.Api.csproj` al proyecto `GesFer.Admin.Back.Infrastructure.csproj`. Si la capa API necesita usar EntityFramework migrations tooling en runtime para semillas, asegurarse de que los métodos de extensión en Infrastructure exponen la funcionalidad sin forzar a la API a depender del paquete NuGet. En este caso `RunMigrationsAndSeedsAsync` ya está expuesto en `Infrastructure`. Eliminar la dependencia del `.csproj` de la API.
*   **Código:**
    Eliminar en `src/GesFer.Admin.Back.Api/GesFer.Admin.Back.Api.csproj`:
    ```xml
    <PackageReference Include="Microsoft.EntityFrameworkCore.Design" Version="8.0.0">
      <PrivateAssets>all</PrivateAssets>
      <IncludeAssets>runtime; build; native; contentfiles; analyzers; buildtransitive</IncludeAssets>
    </PackageReference>
    ```
    Y, si es necesario, añadir a `src/GesFer.Admin.Back.Infrastructure/GesFer.Admin.Back.Infrastructure.csproj`.
*   **Definition of Done (DoD):**
    1. El proyecto `GesFer.Admin.Back.Api` ya no tiene el paquete `Microsoft.EntityFrameworkCore.Design`.
    2. El proyecto compila correctamente (`dotnet build`).
    3. Los tests pasan correctamente (`dotnet test`).

*(Nota: En esta auditoría inicial nos enfocamos en el cumplimiento de los pilares críticos de la arquitectura "Clean" y testabilidad. El acoplamiento es el punto más relevante encontrado basado en el contexto provisto.)*
