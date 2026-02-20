# IMPL-RESOLVE-AUDIT-DEBT

**Título:** Documento de Implementación - Resolución de Deuda Técnica
**Feature:** resolve-audit-debt
**Plan:** docs/features/resolve-audit-debt/plan.md

## Fases de Implementación

### Fase 1: Estructura y DTOs (Compilación)

#### 1.1 Crear CreateLogDto
- **Acción:** Crear
- **Ruta:** `src/GesFer.Admin.Back.Application/DTOs/Logs/CreateLogDto.cs`
- **Contenido:**
  ```csharp
  namespace GesFer.Admin.Back.Application.DTOs.Logs;
  public class CreateLogDto
  {
      public string Level { get; set; } = default!;
      public string Message { get; set; } = default!;
      public string? Exception { get; set; }
      public DateTime TimeStamp { get; set; }
      public Dictionary<string, object>? Properties { get; set; }
  }
  ```

#### 1.2 Crear CreateAuditLogDto
- **Acción:** Crear
- **Ruta:** `src/GesFer.Admin.Back.Application/DTOs/Logs/CreateAuditLogDto.cs`
- **Contenido:**
  ```csharp
  namespace GesFer.Admin.Back.Application.DTOs.Logs;
  public class CreateAuditLogDto
  {
      public string? CursorId { get; set; }
      public string? Username { get; set; }
      public string Action { get; set; } = default!;
      public string HttpMethod { get; set; } = default!;
      public string Path { get; set; } = default!;
      public string? AdditionalData { get; set; }
      public DateTime ActionTimestamp { get; set; }
  }
  ```

#### 1.3 Crear LogDto
- **Acción:** Crear
- **Ruta:** `src/GesFer.Admin.Back.Application/DTOs/Logs/LogDto.cs`
- **Contenido:**
  ```csharp
  namespace GesFer.Admin.Back.Application.DTOs.Logs;
  public class LogDto
  {
      public int Id { get; set; }
      public string Level { get; set; } = default!;
      public string Message { get; set; } = default!;
      public string? Exception { get; set; }
      public DateTime TimeStamp { get; set; }
      public string? Source { get; set; }
      public Guid? CompanyId { get; set; }
      public Guid? UserId { get; set; }
  }
  ```

#### 1.4 Crear LogsPagedResponseDto
- **Acción:** Crear
- **Ruta:** `src/GesFer.Admin.Back.Application/DTOs/Logs/LogsPagedResponseDto.cs`
- **Contenido:**
  ```csharp
  namespace GesFer.Admin.Back.Application.DTOs.Logs;
  public class LogsPagedResponseDto
  {
      public List<LogDto> Logs { get; set; } = new();
      public int TotalCount { get; set; }
      public int PageNumber { get; set; }
      public int PageSize { get; set; }
      public int TotalPages { get; set; }
  }
  ```

#### 1.5 Crear PurgeLogsResponseDto
- **Acción:** Crear
- **Ruta:** `src/GesFer.Admin.Back.Application/DTOs/Logs/PurgeLogsResponseDto.cs`
- **Contenido:**
  ```csharp
  namespace GesFer.Admin.Back.Application.DTOs.Logs;
  public class PurgeLogsResponseDto
  {
      public int DeletedCount { get; set; }
      public DateTime DateLimit { get; set; }
  }
  ```

### Fase 2: Inversión de Dependencias (Interfaces)

#### 2.1 Definir IApplicationDbContext
- **Acción:** Crear
- **Ruta:** `src/GesFer.Admin.Back.Application/Common/Interfaces/IApplicationDbContext.cs`
- **Contenido:**
  ```csharp
  using GesFer.Admin.Back.Domain.Entities;
  using Microsoft.EntityFrameworkCore;
  namespace GesFer.Admin.Back.Application.Common.Interfaces;
  public interface IApplicationDbContext
  {
      DbSet<Log> Logs { get; }
      DbSet<AuditLog> AuditLogs { get; }
      Task<int> SaveChangesAsync(CancellationToken cancellationToken = default);
  }
  ```

#### 2.2 Definir Interfaces de Servicio
- **Acción:** Crear
- **Ruta:** `src/GesFer.Admin.Back.Application/Common/Interfaces/IAdminAuthService.cs`
- **Contenido:** (Definir interfaz vacía o con métodos si se conocen, por ahora placeholder para mover implementación)
  ```csharp
  namespace GesFer.Admin.Back.Application.Common.Interfaces;
  public interface IAdminAuthService { }
  ```
- **Ruta:** `src/GesFer.Admin.Back.Application/Common/Interfaces/IAdminJwtService.cs`
- **Ruta:** `src/GesFer.Admin.Back.Application/Common/Interfaces/IAuditLogService.cs`

#### 2.3 Implementar IApplicationDbContext en Infra
- **Acción:** Modificar
- **Ruta:** `src/GesFer.Admin.Back.Infrastructure/Data/AdminDbContext.cs` (ruta asumida, verificar)
- **Propuesta:** Hacer que la clase implemente `IApplicationDbContext`.

#### 2.4 Corregir Application.csproj
- **Acción:** Modificar
- **Ruta:** `src/GesFer.Admin.Back.Application/GesFer.Admin.Back.Application.csproj`
- **Propuesta:** Eliminar `<ProjectReference Include="..\GesFer.Admin.Back.Infrastructure\GesFer.Admin.Back.Infrastructure.csproj" />`.

#### 2.5 Corregir Infrastructure.csproj
- **Acción:** Modificar
- **Ruta:** `src/GesFer.Admin.Back.Infrastructure/GesFer.Admin.Back.Infrastructure.csproj`
- **Propuesta:** Añadir `<ProjectReference Include="..\GesFer.Admin.Back.Application\GesFer.Admin.Back.Application.csproj" />` (si no existe).

### Fase 3: Desacoplamiento de API

#### 3.1 Refactorizar LogController
- **Acción:** Modificar
- **Ruta:** `src/GesFer.Admin.Back.Api/Controllers/LogController.cs`
- **Propuesta:**
  - Sustituir `private readonly AdminDbContext _context;` por `private readonly IApplicationDbContext _context;`.
  - Actualizar constructor.
  - Añadir `using GesFer.Admin.Back.Application.Common.Interfaces;`.
  - Eliminar `using GesFer.Admin.Back.Infrastructure.Data;`.

### Fase 4: Limpieza

#### 4.1 Eliminar Tests Legacy
- **Acción:** Eliminar Carpeta
- **Ruta:** `src/tests/`
