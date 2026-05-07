---
feature_name: kaizen-ef-asnotracking
base: []
scope:
  in_scope: [
    "src/GesFer.Admin.Back.Application/Handlers/Company/GetAllCompaniesHandler.cs",
    "src/GesFer.Admin.Back.Application/Handlers/Company/GetCompanyByIdHandler.cs",
    "src/GesFer.Admin.Back.Application/Handlers/Company/GetCompanyByNameHandler.cs",
    "src/GesFer.Admin.Back.Application/Handlers/User/GetAllUsersHandler.cs",
    "src/GesFer.Admin.Back.Application/Handlers/User/GetUserByIdHandler.cs"
  ]
  out_scope: []
---
# Especificación: Optimización Termodinámica EF Core (AsNoTracking)

## Detalles Técnicos
Se inyectó el método `.AsNoTracking()` en las consultas de Entity Framework Core (`_context.Companies` y `_context.Users`) en los Handlers que representan operaciones de solo lectura (Queries). Esto optimiza el consumo de memoria y la velocidad de ejecución al evitar que EF Core asigne recursos de tracking a entidades que no van a ser modificadas en la transacción actual.
