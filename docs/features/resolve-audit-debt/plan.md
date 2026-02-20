# PLAN-RESOLVE-AUDIT-DEBT

**Título:** Plan de Implementación para Resolución de Deuda Técnica
**Feature:** resolve-audit-debt
**Estado:** Pendiente

## Fases de Implementación

### Fase 1: Estructura y DTOs (Compilación)
1.  **Crear DTOs:**
    *   En `src/GesFer.Admin.Back.Application/DTOs/Logs/`:
        *   `CreateLogDto.cs`
        *   `CreateAuditLogDto.cs`
        *   `LogDto.cs`
        *   `LogsPagedResponseDto.cs`
        *   `PurgeLogsResponseDto.cs`
    *   *Verificación:* `dotnet build` debe mostrar menos errores (solo quedarán los de referencias de proyecto).

### Fase 2: Inversión de Dependencias (Interfaces)
1.  **Definir Interfaces en Application:**
    *   En `src/GesFer.Admin.Back.Application/Common/Interfaces/`:
        *   `IApplicationDbContext.cs` (con `DbSet<Log>`, `DbSet<AuditLog>`, `SaveChangesAsync`).
        *   `IAdminAuthService.cs` (extraer de Infra).
        *   `IAdminJwtService.cs` (extraer de Infra).
        *   `IAuditLogService.cs` (extraer de Infra).
2.  **Implementar Interfaces en Infrastructure:**
    *   Hacer que `AdminDbContext` implemente `IApplicationDbContext`.
    *   Asegurar que los servicios en `Infrastructure/Services` implementen las interfaces de `Application`.
3.  **Corregir Referencias de Proyecto (.csproj):**
    *   En `src/GesFer.Admin.Back.Application/GesFer.Admin.Back.Application.csproj`: Eliminar referencia a `GesFer.Admin.Back.Infrastructure`.
    *   En `src/GesFer.Admin.Back.Infrastructure/GesFer.Admin.Back.Infrastructure.csproj`: Añadir referencia a `GesFer.Admin.Back.Application`.

### Fase 3: Desacoplamiento de API (LogController)
1.  **Refactorizar LogController:**
    *   Modificar constructor para inyectar `IApplicationDbContext` en lugar de `AdminDbContext`.
    *   Actualizar `using` statements.
    *   *Verificación:* `dotnet build` exitoso.

### Fase 4: Limpieza Legacy
1.  **Eliminar Código Muerto:**
    *   Borrar carpeta `src/tests/` y todo su contenido.
    *   Eliminar referencia a `src/tests/...` en `GesFer.Admin.Back.sln` si existe.

## Verificación Final
1.  Ejecutar `dotnet build src/GesFer.Admin.Back.sln`.
2.  Verificar que no existen referencias circulares.
3.  Verificar que `LogController` no depende de `Infrastructure`.
