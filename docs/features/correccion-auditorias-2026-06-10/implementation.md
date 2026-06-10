---
type: implementation
feature_name: correccion-auditorias-2026-06-10
---

# Implementación de Correcciones

- Se ha añadido el parámetro `CancellationToken cancellationToken = default` al método `AuthenticateAsync` en `IAdminAuthService.cs` y `AdminAuthService.cs`.
- Se ha encadenado `.AsNoTracking()` a la consulta de `AdminUsers` en `AuthenticateAsync`.
- Se ha pasado el `CancellationToken` a la llamada de `.FirstOrDefaultAsync()`.
- Se ha actualizado `AdminLoginHandler.cs` para propagar el `CancellationToken` al método `AuthenticateAsync`.
