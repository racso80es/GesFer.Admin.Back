---
base: []
scope:
  in_scope:
    - "Add CancellationToken and AsNoTracking to AdminAuthService"
  out_scope:
    - "Refactoring AdminJsonDataSeeder completely"
---

# Correccion Auditoria 2026-06-01

## Descripción
Propagación de CancellationToken y optimización de memoria (AsNoTracking) en `AdminAuthService`.

## Implementación
1. En `src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs`, modificar `AuthenticateAsync` para recibir un `CancellationToken cancellationToken = default` y usar `AsNoTracking()` al consultar el usuario, así como pasar el `cancellationToken` a `FirstOrDefaultAsync`.
2. Actualizar `IAdminAuthService.cs` y `AdminLoginHandler.cs`.
