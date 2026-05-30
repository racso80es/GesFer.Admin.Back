---
type: spec
base: []
scope:
  in_scope:
    - Actualizar IAdminAuthService.cs
    - Actualizar AdminAuthService.cs
    - Actualizar AdminLoginHandler.cs
    - Actualizar tests relacionados en AdminLoginHandlerTests.cs y AdminAuthServiceTests.cs
  out_scope:
    - AuditLogService.cs (No se puede modificar fácilmente la firma debido al uso en scope y no hay CancellationToken disponible donde se llama en el log dispatcher).
---

# Especificación

Implementar las correcciones solicitadas en el informe de auditoría:
- Agregar parámetro `CancellationToken` a `IAdminAuthService.AuthenticateAsync`.
- Implementar `CancellationToken` y `AsNoTracking()` en `AdminAuthService.AuthenticateAsync`.
- Propagar `CancellationToken` desde `AdminLoginHandler`.
- Actualizar mocks en `AdminLoginHandlerTests` y `AdminAuthServiceTests`.
