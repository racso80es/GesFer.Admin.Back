---
base:
  - main
scope:
  in_scope:
    - src/GesFer.Admin.Back.Application/Common/Interfaces/IAdminAuthService.cs
    - src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs
    - src/GesFer.Admin.Back.Application/Handlers/Auth/AdminLoginHandler.cs
    - src/GesFer.Admin.Back.Application/Common/Interfaces/IAuditLogService.cs
    - src/GesFer.Admin.Back.Infrastructure/Services/AuditLogService.cs
  out_scope:
    - Otros handlers o repositorios.
---

# Corrección de Auditoría (2026-06-06)

Se aplica el plan de remediación de auditoría detallado en `docs/audits/AUDITORIA_2026_06_06.md`:
1. Aplicar `CancellationToken` y `AsNoTracking()` en `AdminAuthService.cs` y su interfaz.
2. Propagar `CancellationToken` en `AdminLoginHandler.cs`.
3. Propagar `CancellationToken` en `IAuditLogService.cs` y `AuditLogService.cs`.
