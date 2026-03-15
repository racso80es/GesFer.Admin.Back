# AUDITORIA_2026_03_15

## 1. Métricas de Salud
* **Arquitectura:** 95%
* **Nomenclatura:** 100%
* **Estabilidad Async:** 100%

## 2. Pain Points

### 🔴 Crítico: Falta clase `GetAuditLogsQuery` para endpoints de auditoría
**Hallazgo:** El proyecto presenta un error de compilación debido a la ausencia de la clase `GetAuditLogsQuery` requerida por el método `GetAuditLogs` en el controlador `LogController.cs`. Esto impide que la aplicación inicie y provea sus servicios fundamentales.
**Ubicación:** `src/GesFer.Admin.Back.Api/Controllers/LogController.cs:62`

## 3. Acciones Kaizen (Hoja de Ruta para el Executor)
**Instrucción:** Crear la clase CQRS `GetAuditLogsQuery` y su manejador en la capa de Aplicación para restablecer la compilación y resolver el endpoint del controlador.

### Fragmento de Código (GetAuditLogsQuery)
Se deberá crear el archivo `src/GesFer.Admin.Back.Application/Queries/Logs/GetAuditLogsQuery.cs` con el siguiente contenido:

```csharp
using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.Logs;
using MediatR;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Queries.Logs;

public record GetAuditLogsQuery(
    string? Action,
    string? Username,
    int PageNumber = 1,
    int PageSize = 50) : IRequest<AuditLogsPagedResponseDto>;

public class GetAuditLogsHandler : IRequestHandler<GetAuditLogsQuery, AuditLogsPagedResponseDto>
{
    private readonly IApplicationDbContext _context;

    public GetAuditLogsHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<AuditLogsPagedResponseDto> Handle(GetAuditLogsQuery request, CancellationToken cancellationToken)
    {
        var query = _context.AuditLogs.AsQueryable();

        if (!string.IsNullOrEmpty(request.Action))
            query = query.Where(x => x.Action == request.Action);

        if (!string.IsNullOrEmpty(request.Username))
            query = query.Where(x => x.Username == request.Username);

        var totalCount = await query.CountAsync(cancellationToken);

        var logs = await query
            .OrderByDescending(x => x.ActionTimestamp)
            .Skip((request.PageNumber - 1) * request.PageSize)
            .Take(request.PageSize)
            .ToListAsync(cancellationToken);

        var auditLogDtos = logs.Select(x => new AuditLogDto
        {
            Id = x.Id,
            CursorId = x.CursorId,
            Username = x.Username,
            Action = x.Action,
            HttpMethod = x.HttpMethod,
            Path = x.Path,
            AdditionalData = x.AdditionalData,
            ActionTimestamp = x.ActionTimestamp
        }).ToList();

        return new AuditLogsPagedResponseDto
        {
            AuditLogs = auditLogDtos,
            TotalCount = totalCount,
            PageNumber = request.PageNumber,
            PageSize = request.PageSize,
            TotalPages = (int)Math.Ceiling(totalCount / (double)request.PageSize)
        };
    }
}
```

### Definition of Done (DoD)
- El archivo `GetAuditLogsQuery.cs` ha sido creado correctamente en la ruta indicada.
- El proyecto vuelve a compilar correctamente, con 0 errores en la solución.
- Los tests pasan exitosamente demostrando que no hay regresiones.
