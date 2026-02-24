# IMPLEMENTATION: Auditoría de Interacciones entre Entidades

**Feature:** auditoria-interacciones-entidades
**Fecha:** 2026-02-24
**Estado:** Completado

## Resumen de Cambios

Se han implementado los componentes CQRS faltantes y se ha refactorizado la arquitectura para cumplir con Clean Architecture y resolver los errores de compilación ("The Wall").

### 1. Capa de Aplicación (Application Layer)

Implementación de patrones CQRS para gestión de Logs:

-   **Commands:**
    -   `CreateLogCommand`: Maneja la creación de logs generales.
    -   `CreateAuditLogCommand`: Maneja la creación de logs de auditoría delegando en `IAuditLogService`.
    -   `PurgeLogsCommand`: Maneja la eliminación masiva de logs antiguos usando `ExecuteDeleteAsync`.
-   **Queries:**
    -   `GetLogsQuery`: Maneja la consulta paginada y filtrada de logs.
-   **Handlers:**
    -   `CreateLogHandler`, `CreateAuditLogHandler`, `PurgeLogsHandler`, `GetLogsHandler`.

### 2. Capa de API (Api Layer)

Refactorización profunda de `LogController` y configuración:

-   **LogController:**
    -   Eliminada lógica de negocio y validaciones manuales.
    -   Implementada delegación completa a MediatR.
    -   Manejo de excepciones centralizado.
-   **Program.cs:**
    -   Eliminada configuración explícita de Serilog y Sinks.
    -   Reemplazado por llamada a `builder.Host.ConfigureInfrastructureLogging()`.
-   **GesFer.Admin.Back.Api.csproj:**
    -   Eliminadas referencias a paquetes de implementación de infraestructura (`Serilog.Sinks.MySQL`, `Pomelo.EntityFrameworkCore.MySql`, `Serilog.Sinks.Async`).
    -   Mantenida referencia al proyecto `GesFer.Admin.Back.Infrastructure` únicamente para la Composición de Dependencias (DI) en `Program.cs`.

### 3. Capa de Infraestructura (Infrastructure Layer)

Centralización de configuración:

-   **Logging:**
    -   Creado `Logging/SerilogConfiguration.cs` con método de extensión `ConfigureInfrastructureLogging`.
    -   Encapsula la lógica de configuración de Serilog, incluyendo Sinks de MySQL y Console.
-   **Dependencias:**
    -   Añadidos paquetes `Serilog.AspNetCore`, `Serilog.Sinks.MySQL`, `Serilog.Sinks.Async`, `Serilog.Sinks.Console`.
-   **Extensiones:**
    -   Actualizado `WebAppExtensions.cs` para evitar ejecutar migraciones si el proveedor de base de datos no es relacional (fix para Tests).

### 4. Verificación

-   **Compilación:** Exitosa.
-   **Pruebas:** `dotnet test` ejecutado exitosamente (27 pasados, 1 saltado por limitación de InMemory).
-   **Arquitectura:** Respetada la inversión de dependencias; Api no conoce detalles de implementación de base de datos ni logging (paquetes NuGet eliminados).
