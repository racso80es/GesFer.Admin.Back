# Implementation: Corrección auditoría 2026-02-23

**Ruta (Cúmulo):** paths.featurePath/correccion-auditorias-20260223/  
**Spec:** spec.md | **Plan:** plan.md

## Touchpoints por fase

### Fase 1 — Modularizar inyección de dependencias

| # | Archivo | Cambio |
|---|---------|--------|
| 1.1 | `src/GesFer.Admin.Back.Application/DependencyInjection.cs` | **Crear.** AddApplicationServices(IServiceCollection): solo MediatR desde Assembly ejecutante. |
| 1.2 | `src/GesFer.Admin.Back.Infrastructure/DependencyInjection.cs` | **Crear.** AddInfrastructureServices(IServiceCollection, IConfiguration, IWebHostEnvironment?): DbContext, IApplicationDbContext, AdminAuthService, AdminJwtService, AuditLogService, AdminJsonDataSeeder, SequentialGuidGenerator, SensitiveDataSanitizer. |
| 1.3 | `src/GesFer.Admin.Back.Api/DependencyInjection.cs` | **Reemplazar.** Solo invocar Application.DependencyInjection.AddApplicationServices(services) e Infrastructure.DependencyInjection.AddInfrastructureServices(services, configuration, environment). Sin usings de Infrastructure.Data ni Infrastructure.Services. |
| 1.4 | `src/GesFer.Admin.Back.Infrastructure/Extensions/WebAppExtensions.cs` | **Crear (opcional).** RunMigrationsAndSeedsAsync(IApplication) para que Program no resuelva AdminDbContext directamente. |
| 1.5 | `src/GesFer.Admin.Back.Api/Program.cs` | Quitar usings Infrastructure.Data/Services; llamar extension RunMigrationsAndSeedsAsync si existe, o mantener resolución de AdminDbContext/Seeder en un solo bloque con using mínimo. |

### Fase 2 — CQRS en Logs

| # | Archivo | Cambio |
|---|---------|--------|
| 2.1 | `src/GesFer.Admin.Back.Application/Commands/Logs/CreateLogCommand.cs` | **Crear.** IRequest<Unit> con CreateLogDto. |
| 2.2 | `src/GesFer.Admin.Back.Application/Commands/Logs/CreateAuditLogCommand.cs` | **Crear.** IRequest<Unit> con CreateAuditLogDto. |
| 2.3 | `src/GesFer.Admin.Back.Application/Commands/Logs/GetLogsQuery.cs` | **Crear.** IRequest<LogsPagedResponseDto> con filtros (fromDate, toDate, level, companyId, userId, pageNumber, pageSize). |
| 2.4 | `src/GesFer.Admin.Back.Application/Commands/Logs/PurgeLogsCommand.cs` | **Crear.** IRequest<PurgeLogsResponseDto> con DateTime dateLimit. |
| 2.5 | `src/GesFer.Admin.Back.Application/Handlers/Logs/LogHandlers.cs` | **Crear.** CreateLogCommandHandler, CreateAuditLogCommandHandler, GetLogsQueryHandler, PurgeLogsCommandHandler (inyección IApplicationDbContext). |
| 2.6 | `src/GesFer.Admin.Back.Api/Controllers/LogController.cs` | Inyectar IMediator (ISender); ReceiveLog → Send(CreateLogCommand), ReceiveAuditLog → Send(CreateAuditLogCommand), GetLogs → Send(GetLogsQuery), PurgeLogs → Send(PurgeLogsCommand). Eliminar IApplicationDbContext. |

### Fase 3 — Test de integración

| # | Archivo | Cambio |
|---|---------|--------|
| 3.1 | `src/GesFer.Admin.Back.IntegrationTests/AdminWebAppFactory.cs` | Sustituir UseInMemoryDatabase por UseSqlite("DataSource=:memory:") (o conexión única) para que ExecuteDeleteAsync funcione. EnsureCreated. |
| 3.2 | `src/GesFer.Admin.Back.IntegrationTests/GesFer.Admin.Back.IntegrationTests.csproj` | Añadir referencia a Microsoft.EntityFrameworkCore.Sqlite si se usa SQLite. |

## Orden de ejecución
1.1 → 1.2 → 1.3 → 1.4/1.5 → 2.1–2.5 → 2.6 → 3.1–3.2 → build → test.
