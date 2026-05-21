---
feature_name: kaizen-asnotracking
created: 2026-05-10
base:
  - src/GesFer.Admin.Back.Application/Handlers/Company/CreateCompanyHandler.cs
  - src/GesFer.Admin.Back.Application/Handlers/Company/UpdateCompanyHandler.cs
  - src/GesFer.Admin.Back.Application/Handlers/User/CreateUserHandler.cs
  - src/GesFer.Admin.Back.Application/Handlers/User/UpdateUserHandler.cs
scope:
  in_scope:
    - Agregar .AsNoTracking() a validaciones de unicidad FirstOrDefaultAsync y AnyAsync.
  out_scope:
    - Modificar lógicas de negocio o validaciones adicionales.
---
# Especificación técnica
Modificar los Handlers identificados para incorporar `.AsNoTracking()` a aquellas consultas que no requieren el tracking del contexto de EF Core, específicamente en validaciones previas a mutaciones (como verificación de existencia de nombres o usuarios).
