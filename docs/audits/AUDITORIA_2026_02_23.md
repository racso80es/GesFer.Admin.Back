# Reporte de Auditoría Técnica - GesFer.Admin.Back

**Fecha:** 2026-02-23
**Auditor:** Guardián de la Infraestructura (AI Agent)
**Estado:** 🔴 REQUIERE ATENCIÓN INMEDIATA (Final Version)

## 1. Métricas de Salud (Semaforización)

| Dimensión | Puntuación | Estado | Observaciones |
|-----------|:----------:|:------:|---------------|
| **Arquitectura** | **40%** | 🔴 Crítico | Violación flagrante de Clean Architecture (Api -> Infrastructure). Lógica de negocio en Controladores. |
| **Nomenclatura** | **100%** | 🟢 Óptimo | Correcto uso de `DTOs`, Namespaces y convenciones PascalCase. |
| **Estabilidad Async** | **100%** | 🟢 Óptimo | No se detectaron bloqueos síncronos (`.Result`, `.Wait()`) ni `async void`. |
| **Testabilidad** | **80%** | 🟡 Medio | Tests unitarios pasan (51/51), pero falla 1 test de integración crítico (`PurgeLogs`). |

---

## 2. Pain Points (Hallazgos)

### 🔴 Críticos (Bloqueantes para Escalabilidad/Mantenimiento)

1.  **Violación de Dependencias (Api referencia a Infrastructure):**
    -   **Ubicación:** `src/GesFer.Admin.Back.Api/DependencyInjection.cs` y `.csproj`.
    -   **Descripción:** La API registra manualmente servicios de infraestructura (`AdminDbContext`, `AdminAuthService`, `MySqlSequentialGuidGenerator`). Esto acopla fuertemente la capa de presentación con detalles de implementación (MySQL, Serilog Sinks, etc.).
    -   **Impacto:** Imposible intercambiar infraestructura sin tocar la API. Dificulta testing aislado.

2.  **Lógica de Negocio en Controladores (Fat Controllers):**
    -   **Ubicación:** `src/GesFer.Admin.Back.Api/Controllers/LogController.cs`.
    -   **Descripción:** El controlador inyecta `IApplicationDbContext` y realiza operaciones de base de datos directamente (`Add`, `Where`, `ExecuteDeleteAsync`).
    -   **Impacto:** Viola Single Responsibility Principle. La lógica no es reutilizable ni fácilmente testeable unitariamente sin mocks complejos del DbContext. Debería usar **MediatR (CQRS)** como el resto del sistema.

3.  **Ausencia de Modularidad en Inyección de Dependencias:**
    -   **Ubicación:** Faltan `src/GesFer.Admin.Back.Application/DependencyInjection.cs` y `src/GesFer.Admin.Back.Infrastructure/DependencyInjection.cs`.
    -   **Descripción:** La responsabilidad de registrar servicios está centralizada en la API o dispersa, en lugar de que cada capa exponga un método de extensión (`AddApplicationServices`, `AddInfrastructureServices`).

### 🟡 Medios (Deuda Técnica / Bugs)

1.  **Fallo en Test de Integración:**
    -   **Ubicación:** `GesFer.Admin.Back.IntegrationTests.LogControllerTests.PurgeLogs_ShouldDeleteOldLogs`.
    -   **Descripción:** El test falla con `500 Internal Server Error` cuando se espera un `200 OK`.
    -   **Causa probable:** Excepción no controlada dentro de `ExecuteDeleteAsync` en el entorno de test, o configuración incorrecta del `DbContext` en tests.

---

## 3. Acciones Kaizen (Hoja de Ruta para el Executor)

El **Kaizen Executor** debe realizar las siguientes tareas en orden de prioridad:

### Paso 1: Modularizar Inyección de Dependencias (Refactorización Estructural)

1.  **Crear `src/GesFer.Admin.Back.Application/DependencyInjection.cs`:**
    ```csharp
    namespace GesFer.Admin.Back.Application;

    public static class DependencyInjection
    {
        public static IServiceCollection AddApplicationServices(this IServiceCollection services)
        {
            // Registrar MediatR
            services.AddMediatR(cfg => cfg.RegisterServicesFromAssembly(Assembly.GetExecutingAssembly()));
            // Registrar Validators, Behaviors, etc.
            return services;
        }
    }
    ```

2.  **Crear `src/GesFer.Admin.Back.Infrastructure/DependencyInjection.cs`:**
    ```csharp
    namespace GesFer.Admin.Back.Infrastructure;

    public static class DependencyInjection
    {
        public static IServiceCollection AddInfrastructureServices(this IServiceCollection services, IConfiguration configuration)
        {
            // Mover toda la lógica de DB, Auth, Serilog Sinks aquí.
            // Registrar implementaciones de interfaces (IAdminAuthService, etc.)
            return services;
        }
    }
    ```

3.  **Limpiar `src/GesFer.Admin.Back.Api/DependencyInjection.cs`:**
    -   Eliminar referencias a `Infrastructure.Data` y `Infrastructure.Services`.
    -   Reemplazar código manual por llamadas a `builder.Services.AddApplicationServices()` y `builder.Services.AddInfrastructureServices(config)`.

### Paso 2: Implementar CQRS en Logs (Refactorización Lógica)

1.  **Crear Comandos/Queries en Application:**
    -   `CreateLogCommand`
    -   `CreateAuditLogCommand`
    -   `GetLogsQuery`
    -   `PurgeLogsCommand`
2.  **Migrar lógica de `LogController` a Handlers:**
    -   Mover la lógica de `ExecuteDeleteAsync` al `PurgeLogsCommandHandler`.
    -   Mover los filtros de búsqueda al `GetLogsQueryHandler`.
3.  **Actualizar Controller:**
    -   Inyectar `ISender` (MediatR).
    -   Los métodos del controlador deben ser simples "dispatchers": `await _sender.Send(command)`.

### Paso 3: Corregir Test de Integración

1.  Investigar el error 500 en `PurgeLogs`.
2.  Asegurar que el test usa una base de datos efímera compatible o mocks adecuados para `ExecuteDeleteAsync` (Nota: `ExecuteDeleteAsync` no funciona bien con InMemory, requiere base de datos relacional real o Testcontainers).

### Definition of Done (DoD)

- [ ] `GesFer.Admin.Back.Api` no tiene `using GesFer.Admin.Back.Infrastructure.*`.
- [ ] Todos los servicios se registran mediante métodos de extensión por capa.
- [ ] `LogController` no inyecta `IApplicationDbContext`.
- [ ] `dotnet test` pasa al 100% (incluyendo integración).
