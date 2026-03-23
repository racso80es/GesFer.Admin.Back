# Auditoría S+ - 2026-03-08

## 1. Métricas de Salud (0-100%)
- **Arquitectura**: 98% (Clean Architecture respetada en general, pero se encuentra acoplamiento a EF Core en la capa Application y API)
- **Nomenclatura**: 100% (Convenciones correctas)
- **Estabilidad Async**: 99% (Casi 100%, solo se encontró un uso sincrónico en `AuthorizeSystemOrAdminAttribute.cs` con `.Result`, pero es solo asignación de propiedad, no una llamada bloqueante).

## 2. Pain Points (🔴 Críticos / 🟡 Medios)

**Hallazgo 1: 🟡 Acoplamiento a Entity Framework Core en capas no de infraestructura**
- **Descripción**: La capa de Aplicación y la API no deben depender directamente de `Microsoft.EntityFrameworkCore`. Referenciar directamente EF Core viola el principio de abstracción de Clean Architecture. Los handlers de la capa de Application están usando directamente `using Microsoft.EntityFrameworkCore` y posiblemente métodos específicos de EF (`.FirstOrDefaultAsync`, `.ToListAsync`, etc) sobre los DbSets. Esto debería estar encapsulado en repositorios o mediante el `IApplicationDbContext` proveyendo abstracciones, pero la referencia existe en el `.csproj` y en el código.
- **Ubicación**:
  - `src/GesFer.Admin.Back.Api/Program.cs`
  - `src/GesFer.Admin.Back.Application/GesFer.Admin.Back.Application.csproj`
  - Múltiples handlers en `src/GesFer.Admin.Back.Application/Handlers/` (ej. `GetAllCompaniesHandler.cs`, `GeoHandlers.cs`, etc.)
  - Queries y Commands en `src/GesFer.Admin.Back.Application/Queries/` y `Commands/`.

**Hallazgo 2: 🟡 Uso de colecciones mutables (List) en DTOs y Handlers**
- **Descripción**: Se están devolviendo `List<T>` en vez de colecciones de solo lectura como `IReadOnlyCollection<T>` o `IEnumerable<T>` en los Handlers, Commands y DTOs, lo cual rompe la inmutabilidad esperada en estas estructuras de transferencia de datos.
- **Ubicación**:
  - `src/GesFer.Admin.Back.Application/Commands/Geo/GeoQueries.cs`
  - `src/GesFer.Admin.Back.Application/Commands/Company/GetAllCompaniesCommand.cs`
  - `src/GesFer.Admin.Back.Application/Handlers/Geo/GeoHandlers.cs`
  - `src/GesFer.Admin.Back.Application/Handlers/Company/GetAllCompaniesHandler.cs`
  - `src/GesFer.Admin.Back.Api/Controllers/CountriesController.cs`
  - `src/GesFer.Admin.Back.Api/Controllers/CompanyController.cs`

**Hallazgo 3: 🟡 Exposición de detalles de validación de JWT y Serilog en Program.cs**
- **Descripción**: `Program.cs` contiene lógica detallada de configuración de JWT (`AddJwtBearer`) que debería estar encapsulada en la capa de Infraestructura (`AddInfrastructureServices`). Esto aumenta el tamaño del `Program.cs` y mezcla responsabilidades.
- **Ubicación**: `src/GesFer.Admin.Back.Api/Program.cs`

## 3. Acciones Kaizen (Hoja de Ruta para el Executor)

### 3.1. Eliminar referencias directas a EF Core de la capa Application y API
**Instrucciones**:
1. Eliminar el paquete `Microsoft.EntityFrameworkCore` de `GesFer.Admin.Back.Application.csproj`.
2. Modificar `IApplicationDbContext` para no exponer `DbSet<T>` y no depender de EF Core, o bien, si se mantiene un patrón híbrido permitido, asegurar que los imports de EF Core se limiten a lo estrictamente necesario.
3. Nota: En muchos proyectos .NET de Clean Architecture, se permite que `Application` referencie a EF Core si se usa CQRS con MediatR para simplificar las consultas. Si la arquitectura elegida para el proyecto (verificada en la memoria) permite que `Application` referencie `EF Core` para evitar repositorios, este hallazgo se puede mitigar. Sin embargo, la directiva estricta de "no referenciar paquetes de implementación específicos" en API se aplica. En la API, `Microsoft.EntityFrameworkCore` solo se usa en `Program.cs` línea 6. Verificar si se puede quitar de los imports.

**DoD**: El proyecto compila sin errores. `GesFer.Admin.Back.Application` no tiene dependencias innecesarias que rompan la Clean Architecture (si el estándar del equipo prohíbe EF en Application). La API no tiene `using Microsoft.EntityFrameworkCore` innecesario.

### 3.2. Usar IReadOnlyCollection o IEnumerable en lugar de List en DTOs, Queries y Commands
**Instrucciones**:
1. En `src/GesFer.Admin.Back.Application/Commands/Geo/GeoQueries.cs`, cambiar `List<T>` a `IEnumerable<T>`.
2. En `src/GesFer.Admin.Back.Application/Handlers/Geo/GeoHandlers.cs`, cambiar los retornos a `IEnumerable<T>`.
3. Actualizar los controladores para devolver `IEnumerable<T>` en los tipos de respuesta de Swagger.

**DoD**: Se usa `IEnumerable<T>` o `IReadOnlyCollection<T>` en las respuestas de colecciones de los queries/comandos en lugar de `List<T>`. Los tests siguen pasando.

### 3.3. Encapsular la configuración de Auth en Infrastructure
**Instrucciones**:
1. Mover el bloque de configuración de `AddAuthentication` y `AddAuthorization` de `Program.cs` a un método de extensión en la capa de `Infrastructure` (ej. `DependencyInjection.cs`).

**DoD**: `Program.cs` queda más limpio y la configuración de seguridad queda encapsulada en la infraestructura.
