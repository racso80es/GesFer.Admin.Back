# REPORTE DE AUDITORÍA S+ (GesFer.Admin.Back)

**Fecha:** 2026-02-24 (UTC)
**Auditor:** Guardián de la Infraestructura (SddIA Protocol)

---

## 1. Métricas de Salud (0-100%)

| Métrica | Valor | Estado | Observaciones |
| :--- | :--- | :--- | :--- |
| **Arquitectura** | **40%** | 🔴 Crítico | Violación flagrante de Clean Architecture: `Api` referencia `Infrastructure` directamente para configuración de Logging y Database. Lógica de negocio en Controladores. |
| **Nomenclature** | **95%** | 🟢 Estable | Estructura de proyectos y carpetas correcta (`GesFer.Admin.Back.*`). DTOs bien ubicados. |
| **Estabilidad Async** | **0%** | 🔴 Crítico | **THE WALL FAILED.** El proyecto **NO COMPILA**. Faltan clases fundamentales (Comandos/Queries) invocadas en el código. |

---

## 2. Pain Points (🔴 Críticos / 🟡 Medios)

### 🔴 1. Integridad Estructural Comprometida (The Wall)
**Hallazgo:** El proyecto falla en compilación (Error CS0234). El controlador `LogController` invoca comandos y queries (`CreateLogCommand`, `CreateAuditLogCommand`, `GetLogsQuery`, `PurgeLogsCommand`) que **no existen** en la capa de Aplicación.

**Ubicación:**
- `src/GesFer.Admin.Back.Api/Controllers/LogController.cs` (Líneas 42, 66, 92, 119)
- `src/GesFer.Admin.Back.Application/Commands/Logs/` (Directorio inexistente)

### 🔴 2. Violación de Clean Architecture (Dependencia Directa)
**Hallazgo:** El proyecto `Api` tiene referencias directas a paquetes de implementación (`Serilog.Sinks.MySQL`, `Pomelo.EntityFrameworkCore.MySql`) y configura detalles de bajo nivel (connection strings, sinks específicos) en `Program.cs`. Esto rompe la Inversión de Dependencias.

**Ubicación:**
- `src/GesFer.Admin.Back.Api/GesFer.Admin.Back.Api.csproj`
- `src/GesFer.Admin.Back.Api/Program.cs`

### 🟡 3. Fuga de Lógica de Negocio en Controlador
**Hallazgo:** `LogController` contiene validaciones manuales (`if (dto == null)`, `string.IsNullOrWhiteSpace`) y reglas de negocio explícitas (`if (dateLimit > sevenDaysAgo)`). Estas responsabilidades pertenecen a la capa de Aplicación (Validadores/Handlers).

**Ubicación:**
- `src/GesFer.Admin.Back.Api/Controllers/LogController.cs` (Líneas 33-37, 112-116)

---

## 3. Acciones Kaizen (Hoja de Ruta para el Executor)

> **Definition of Done (DoD):** El proyecto debe compilar sin errores, los tests deben pasar, y la arquitectura debe respetar la separación de responsabilidades.

### Acción 1: Implementar Comandos CQRS Faltantes (Prioridad Alta)
Crear la estructura de carpetas `src/GesFer.Admin.Back.Application/Commands/Logs/` e implementar los siguientes archivos:

**1.1 `CreateLogCommand.cs`**
```csharp
using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.Logs;
using MediatR;

namespace GesFer.Admin.Back.Application.Commands.Logs;

public record CreateLogCommand(CreateLogDto Dto) : IRequest<Unit>;

public class CreateLogHandler : IRequestHandler<CreateLogCommand, Unit>
{
    private readonly IApplicationDbContext _context;

    public CreateLogHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<Unit> Handle(CreateLogCommand request, CancellationToken cancellationToken)
    {
        // TODO: Mapear DTO a Entidad Log y guardar.
        // Nota: Log no hereda de BaseEntity según memoria (int Id).
        // var entity = new Domain.Entities.Log { ... };
        // _context.Logs.Add(entity);
        // await _context.SaveChangesAsync(cancellationToken);
        return Unit.Value;
    }
}
```

**1.2 `CreateAuditLogCommand.cs`**
```csharp
using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.Logs;
using MediatR;

namespace GesFer.Admin.Back.Application.Commands.Logs;

public record CreateAuditLogCommand(CreateAuditLogDto Dto) : IRequest<Unit>;

public class CreateAuditLogHandler : IRequestHandler<CreateAuditLogCommand, Unit>
{
    private readonly IAuditLogService _auditService;

    public CreateAuditLogHandler(IAuditLogService auditService)
    {
        _auditService = auditService;
    }

    public async Task<Unit> Handle(CreateAuditLogCommand request, CancellationToken cancellationToken)
    {
        await _auditService.LogAsync(request.Dto);
        return Unit.Value;
    }
}
```

**1.3 `GetLogsQuery.cs`**
```csharp
using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.Logs;
using MediatR;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Commands.Logs;

public record GetLogsQuery(
    DateTime? FromDate,
    DateTime? ToDate,
    string? Level,
    Guid? CompanyId,
    Guid? UserId,
    int PageNumber,
    int PageSize) : IRequest<LogsPagedResponseDto>;

public class GetLogsHandler : IRequestHandler<GetLogsQuery, LogsPagedResponseDto>
{
    private readonly IApplicationDbContext _context;

    public GetLogsHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<LogsPagedResponseDto> Handle(GetLogsQuery request, CancellationToken cancellationToken)
    {
        // Implementar lógica de filtrado y paginación
        return new LogsPagedResponseDto(); // Placeholder para compilación
    }
}
```

**1.4 `PurgeLogsCommand.cs`**
```csharp
using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.Logs;
using MediatR;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Commands.Logs;

public record PurgeLogsCommand(DateTime DateLimit) : IRequest<PurgeLogsResponseDto>;

public class PurgeLogsHandler : IRequestHandler<PurgeLogsCommand, PurgeLogsResponseDto>
{
    private readonly IApplicationDbContext _context;

    public PurgeLogsHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<PurgeLogsResponseDto> Handle(PurgeLogsCommand request, CancellationToken cancellationToken)
    {
        // Validar regla de negocio (7 días) aquí o en Validator
        var count = await _context.Logs
            .Where(l => l.TimeStamp < request.DateLimit)
            .ExecuteDeleteAsync(cancellationToken);

        return new PurgeLogsResponseDto(count);
    }
}
```

### Acción 2: Limpiar LogController (Refactorización)
Eliminar la lógica de validación y negocio del controlador. Delegar todo a MediatR.

```csharp
// Fragmento para PurgeLogs en LogController.cs
[HttpDelete]
[Authorize(Policy = "AdminOnly")]
public async Task<IActionResult> PurgeLogs([FromQuery] DateTime dateLimit)
{
    // La validación de fecha debe moverse al Handler o a un Validator (FluentValidation)
    var result = await _sender.Send(new PurgeLogsCommand(dateLimit));
    return Ok(result);
}
```

### Acción 3: Desacoplar Infraestructura (Arquitectura)
1. Mover la configuración de Serilog (`WriteTo.MySQL`) a un método de extensión en `GesFer.Admin.Back.Infrastructure`.
2. En `GesFer.Admin.Back.Api`, eliminar referencias a paquetes `Serilog.Sinks.MySQL` y `Pomelo.EntityFrameworkCore.MySql`.
3. Usar solo `AddInfrastructureServices` en `Program.cs`.

---
*Fin del Reporte.*
