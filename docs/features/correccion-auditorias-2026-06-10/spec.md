---
type: spec
feature_name: correccion-auditorias-2026-06-10
base:
  - src/GesFer.Admin.Back.Application/Common/Interfaces/IAdminAuthService.cs
  - src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs
  - src/GesFer.Admin.Back.Application/Handlers/Auth/AdminLoginHandler.cs
  - src/GesFer.Admin.Back.UnitTests/Services/AdminAuthServiceTests.cs
scope:
  in_scope:
    - Agregar parámetro CancellationToken con default value a IAdminAuthService.AuthenticateAsync
    - Agregar parámetro CancellationToken con default value a AdminAuthService.AuthenticateAsync
    - Modificar la llamada a EF Core para incluir .AsNoTracking()
    - Modificar la llamada a FirstOrDefaultAsync para pasar el CancellationToken
    - Modificar AdminLoginHandler para pasar el CancellationToken proporcionado en el Handle
  out_scope:
    - Modificaciones a otros servicios.
---

# Especificación

Implementar propagación de CancellationToken y optimizar el rastreo de entidades en el login de Admin.
