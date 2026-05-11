---
feature_name: "kaizen-ef-asnotracking"
base:
  - src/GesFer.Admin.Back.Application/Queries/Logs/GetAuditLogsQuery.cs
  - src/GesFer.Admin.Back.Application/Queries/Logs/GetLogsQuery.cs
  - src/GesFer.Admin.Back.Application/Handlers/Company/GetAllCompaniesHandler.cs
  - src/GesFer.Admin.Back.Application/Handlers/Company/GetCompanyByIdHandler.cs
  - src/GesFer.Admin.Back.Application/Handlers/Company/GetCompanyByNameHandler.cs
  - src/GesFer.Admin.Back.Application/Handlers/Geo/GeoHandlers.cs
  - src/GesFer.Admin.Back.Application/Handlers/User/GetAllUsersHandler.cs
  - src/GesFer.Admin.Back.Application/Handlers/User/GetUserByIdHandler.cs
scope:
  in_scope: "Modificación de archivos listados en la base para incluir la llamada al método .AsNoTracking() donde se usan consultas LINQ para la interfaz de Entity Framework Core de solo lectura."
  out_scope: "No se modificarán otras capas de la arquitectura, ni handlers que impliquen escritura o modificación del estado (Update, Delete, Create)."
type: "spec"
---

# Especificación: Optimización Termodinámica EF Core (AsNoTracking)

Se deben modificar los archivos especificados para incluir `.AsNoTracking()` en las consultas LINQ.
