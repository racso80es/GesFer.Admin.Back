# Objetivos — Corrección según auditoría 2026-02-23

**Origen:** paths.auditsPath — `AUDITORIA_2026_02_23.md` (última auditoría generada)  
**Plantilla:** paths.templatesPath/correccion-auditorias-feature/  
**Proceso:** correccion-auditorias (paths.processPath/correccion-auditorias/)

## Resumen del informe de auditoría

- **Fecha:** 2026-02-23  
- **Estado:** Requiere atención inmediata  
- **Dimensiones:** Arquitectura 40% (crítico), Nomenclatura 100%, Estabilidad Async 100%, Testabilidad 80% (test integración PurgeLogs falla).

## Hallazgos consolidados y prioridades

### Críticos (bloqueantes)

| # | Hallazgo | Ubicación | Criterio de cierre |
|---|----------|-----------|--------------------|
| 1 | **Violación de dependencias:** Api referencia a Infrastructure (registro manual de servicios en Api). | `src/GesFer.Admin.Back.Api/DependencyInjection.cs`, `.csproj` | Api sin `using GesFer.Admin.Back.Infrastructure.*`; registro por métodos de extensión por capa. |
| 2 | **Lógica de negocio en controladores (Fat Controller):** LogController usa IApplicationDbContext y operaciones directas (Add, Where, ExecuteDeleteAsync). | `src/GesFer.Admin.Back.Api/Controllers/LogController.cs` | LogController no inyecta IApplicationDbContext; uso de MediatR (CQRS) como resto del sistema. |
| 3 | **Ausencia de modularidad en DI:** Faltan DependencyInjection en Application e Infrastructure; responsabilidad centralizada en Api. | Application e Infrastructure | Existencia de `AddApplicationServices` y `AddInfrastructureServices`; Api solo invoca esos métodos. |

### Medios (deuda / bugs)

| # | Hallazgo | Ubicación | Criterio de cierre |
|---|----------|-----------|--------------------|
| 4 | **Fallo test de integración:** PurgeLogs devuelve 500 en lugar de 200. | `IntegrationTests.LogControllerTests.PurgeLogs_ShouldDeleteOldLogs` | `dotnet test` pasa al 100% (incluyendo integración); investigar ExecuteDeleteAsync en entorno test / InMemory vs relacional. |

## Criterios de cierre (Definition of Done)

- [ ] `GesFer.Admin.Back.Api` no tiene referencias a `GesFer.Admin.Back.Infrastructure.*`.
- [ ] Servicios registrados mediante métodos de extensión por capa (Application, Infrastructure).
- [ ] `LogController` no inyecta `IApplicationDbContext`; usa `ISender` (MediatR) y handlers (CreateLogCommand, CreateAuditLogCommand, GetLogsQuery, PurgeLogsCommand).
- [ ] `dotnet test` pasa al 100% (unitarios + integración).

## Alcance acotado

Solo lo reportado en AUDITORIA_2026_02_23.md. No se amplía funcionalidad nueva; refactorización estructural y corrección del test de integración.
