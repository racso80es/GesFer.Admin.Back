---
type: spec
feature_name: correccion-2026-06-11
base:
  - src/GesFer.Admin.Back.Application/Common/Interfaces/IAdminAuthService.cs
  - src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs
  - src/GesFer.Admin.Back.Application/Handlers/Auth/AdminLoginHandler.cs
  - src/GesFer.Admin.Back.UnitTests/Handlers/AdminLoginHandlerTests.cs
scope:
  in_scope:
    - Modificar IAdminAuthService para aceptar CancellationToken con valor default.
    - Modificar AdminAuthService para usar AsNoTracking y propagar el CancellationToken.
    - Modificar AdminLoginHandler para enviar el token.
    - Actualizar pruebas unitarias relativas de Moq (It.IsAny<CancellationToken>()).
  out_scope:
    - Otras funcionalidades no reportadas en el informe S+.
---

# Especificación

Implementar CancellationToken y AsNoTracking en la autenticación de usuarios administrativos para mejorar el rendimiento y evitar bloqueo de hilos asíncronos por consultas de EF Core de solo lectura.
