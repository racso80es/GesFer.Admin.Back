---
base:
  - src/GesFer.Admin.Back.Application/Handlers/Company/GetCompanyByNameHandler.cs
  - src/GesFer.Admin.Back.Application/Handlers/Company/GetCompanyByIdHandler.cs
  - src/GesFer.Admin.Back.Application/Handlers/User/GetUserByIdHandler.cs
  - src/GesFer.Admin.Back.Application/Handlers/User/CreateUserHandler.cs
  - src/GesFer.Admin.Back.Application/Handlers/Company/UpdateCompanyHandler.cs
  - src/GesFer.Admin.Back.Application/Handlers/User/UpdateUserHandler.cs
  - src/GesFer.Admin.Back.Application/Handlers/Company/CreateCompanyHandler.cs
  - src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs
scope:
  in_scope:
    - "Agregar .AsNoTracking() a todas las consultas de solo lectura en los handlers especificados."
  out_scope:
    - "Modificar la lógica de negocio o queries de escritura."
---
# Spec: AsNoTracking Optimization

El objetivo es añadir el método `.AsNoTracking()` a las consultas Entity Framework de sólo lectura detectadas durante la exploración.
