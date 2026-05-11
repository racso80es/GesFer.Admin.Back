---
feature_name: "kaizen-ef-asnotracking"
description: "Optimización de Query Handlers en EF Core usando AsNoTracking"
type: "objectives"
---

# Objetivos: Optimización Termodinámica EF Core (AsNoTracking)

El objetivo de esta característica/mejora es aplicar `.AsNoTracking()` a todos los query handlers de `GesFer.Admin.Back.Application` que hacen uso de consultas de solo lectura con Entity Framework Core a través de `IApplicationDbContext`.

El propósito de este cambio es eliminar fugas termodinámicas (consumo innecesario de memoria y CPU) al evitar que Entity Framework Core coloque entidades de solo lectura en su Change Tracker, lo cual degrada el rendimiento.

## Alcance

- `src/GesFer.Admin.Back.Application/Queries/Logs/GetAuditLogsQuery.cs`
- `src/GesFer.Admin.Back.Application/Queries/Logs/GetLogsQuery.cs`
- `src/GesFer.Admin.Back.Application/Handlers/Company/GetAllCompaniesHandler.cs`
- `src/GesFer.Admin.Back.Application/Handlers/Company/GetCompanyByIdHandler.cs`
- `src/GesFer.Admin.Back.Application/Handlers/Company/GetCompanyByNameHandler.cs`
- `src/GesFer.Admin.Back.Application/Handlers/Geo/GeoHandlers.cs`
- `src/GesFer.Admin.Back.Application/Handlers/User/GetAllUsersHandler.cs`
- `src/GesFer.Admin.Back.Application/Handlers/User/GetUserByIdHandler.cs`
