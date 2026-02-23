# PLAN: Corrección según auditoría 2026-02-23

**Proceso:** correccion-auditorias  
**Ruta (Cúmulo):** paths.featurePath/correccion-auditorias-20260223/  
**Especificación:** spec.md | **Objetivos:** objectives.md  

---

## 1. Fases (orden Kaizen)

### Fase 1 — Modularizar inyección de dependencias
1. Crear `src/GesFer.Admin.Back.Application/DependencyInjection.cs` → AddApplicationServices (MediatR, validators, behaviors).
2. Crear `src/GesFer.Admin.Back.Infrastructure/DependencyInjection.cs` → AddInfrastructureServices (DB, Auth, Serilog Sinks, implementaciones).
3. Limpiar `src/GesFer.Admin.Back.Api/DependencyInjection.cs`: eliminar registros directos de Infrastructure; invocar AddApplicationServices() y AddInfrastructureServices(config).
4. Revisar Api .csproj: sin referencia a proyecto Infrastructure.
5. Verificar: dotnet build.

### Fase 2 — CQRS en Logs
1. Crear en Application: CreateLogCommand, CreateAuditLogCommand, GetLogsQuery, PurgeLogsCommand y Handlers (inyección IApplicationDbContext / interfaces Application).
2. Migrar lógica de LogController a Handlers (ExecuteDeleteAsync → PurgeLogsCommandHandler; filtros → GetLogsQueryHandler).
3. Refactorizar LogController: inyectar ISender; métodos como await _sender.Send(command)/Send(query).
4. Verificar: dotnet build, tests unitarios.

### Fase 3 — Corregir test de integración PurgeLogs
1. Investigar causa del 500 en PurgeLogs_ShouldDeleteOldLogs (ExecuteDeleteAsync en InMemory vs relacional).
2. Ajustar configuración de tests (base efímera relacional, Testcontainers o mock adecuado).
3. Verificar: dotnet test (100%).

---

## 2. Orden de ejecución
1. Fase 1 (modular DI)  
2. Fase 2 (CQRS Logs)  
3. Fase 3 (test integración)

## 3. Entregables posteriores
- implementation.md / implementation.json (touchpoints por fase).
- execution (aplicación de cambios).
- validacion.json (validate).
- finalize (Evolution Logs, PR).
