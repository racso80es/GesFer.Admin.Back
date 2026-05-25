---
type: spec
feature_name: correccion-2026-05-25
base:
  - src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs
  - src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs
  - src/GesFer.Admin.Back.Application/Common/Interfaces/IAdminAuthService.cs
  - src/GesFer.Admin.Back.Application/Handlers/Auth/AdminLoginHandler.cs
scope:
  in_scope: Fix missing AsNoTracking and CancellationToken in AdminAuthService, AdminJsonDataSeeder, IAdminAuthService and AdminLoginHandler
  out_scope: Unrelated files
---

# Specification
Apply the Kaizen actions defined in AUDITORIA_2026_05_25.md to improve project testability, auditability and resilience.
