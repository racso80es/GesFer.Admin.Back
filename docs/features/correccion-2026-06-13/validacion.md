---
feature_name: correccion-2026-06-13
branch: feat/correccion-2026-06-13
global:
  - AdminAuthService
  - AuditLogService
  - AdminJsonDataSeeder
  - Unit Tests
checks:
  - CancellationToken propagado a EF Core.
  - Tests ajustados para soportar el nuevo parámetro.
git_changes:
  files_added:
    - docs/features/correccion-2026-06-13/validacion.md
    - docs/features/correccion-2026-06-13/finalize-process.md
  files_modified:
    - src/GesFer.Admin.Back.Application/Common/Interfaces/IAdminAuthService.cs
    - src/GesFer.Admin.Back.Application/Common/Interfaces/IAuditLogService.cs
    - src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs
    - src/GesFer.Admin.Back.Infrastructure/Services/AuditLogService.cs
    - src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs
    - src/GesFer.Admin.Back.Application/Handlers/Auth/AdminLoginHandler.cs
    - src/GesFer.Admin.Back.Application/Handlers/Company/GetCompanyByNameHandler.cs
    - src/GesFer.Admin.Back.Application/Commands/Logs/CreateAuditLogCommand.cs
    - src/GesFer.Admin.Back.UnitTests/Handlers/AdminLoginHandlerTests.cs
    - src/GesFer.Admin.Back.UnitTests/Services/AdminAuthServiceTests.cs
    - src/GesFer.Admin.Back.UnitTests/Services/AuditLogServiceTests.cs
  files_deleted: []
---
# Validación: correccion-2026-06-13

Validación estructural y tests pasando.
