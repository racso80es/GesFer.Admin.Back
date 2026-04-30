---
feature_name: user-crud
created: "2026-04-29"
items_applied:
  - id: I1
    path: "src/GesFer.Admin.Back.Domain/Entities/User.cs"
    action: "add"
    status: "ok"
    message: "Añadida entidad User (BaseEntity) con CompanyId y campos de contacto/geo."
    timestamp: "2026-04-29"
  - id: I1
    path: "src/GesFer.Admin.Back.Infrastructure/Data/Configurations/UserConfiguration.cs"
    action: "add"
    status: "ok"
    message: "Configuración EF: tabla Users, índice único (CompanyId, Username), conversion Email, longitudes."
    timestamp: "2026-04-29"
  - id: I1
    path: "src/GesFer.Admin.Back.Infrastructure/Data/AdminDbContext.cs"
    action: "update"
    status: "ok"
    message: "Añadido DbSet<User> y exposición por IApplicationDbContext."
    timestamp: "2026-04-29"
  - id: I3
    path: "src/GesFer.Admin.Back.Application/DTOs/User/UserDtos.cs"
    action: "add"
    status: "ok"
    message: "DTOs: UserDto/CreateUserDto/UpdateUserDto."
    timestamp: "2026-04-29"
  - id: I3
    path: "src/GesFer.Admin.Back.Application/Commands/User/*"
    action: "add"
    status: "ok"
    message: "Commands MediatR para CRUD."
    timestamp: "2026-04-29"
  - id: I3
    path: "src/GesFer.Admin.Back.Application/Handlers/User/*"
    action: "add"
    status: "ok"
    message: "Handlers CRUD + mapping."
    timestamp: "2026-04-29"
  - id: I4
    path: "src/GesFer.Admin.Back.Api/Controllers/UserController.cs"
    action: "add"
    status: "ok"
    message: "Endpoints /api/User (CRUD) con AuthorizeSystemOrAdmin."
    timestamp: "2026-04-29"
  - id: I4
    path: "src/GesFer.Admin.Back.Api/Security/ClaimsExtensions.cs"
    action: "add"
    status: "ok"
    message: "Helper para CompanyId desde claims (CompanyId/companyId)."
    timestamp: "2026-04-29"
  - id: I5
    path: "docs/diagnostics/feat/user-crud/execution_history.json"
    action: "update"
    status: "ok"
    message: "Build + tests ejecutados vía invoke-command (dotnet build/test)."
    timestamp: "2026-04-29"
---

## Evidencia

- `dotnet build` (Release) y `dotnet test` (Release, --no-build) ejecutados mediante la skill `invoke-command` (ver `docs/diagnostics/feat/user-crud/execution_history.json`).
