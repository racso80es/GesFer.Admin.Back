---
type: spec
feature_name: correccion-2026-05-23
base:
  - docs/audits/AUDITORIA_2026_23_05.md
scope:
  in_scope:
    - Actualizar IAdminAuthService, AdminAuthService, AdminLoginHandler y Tests asociados.
    - Actualizar AdminJsonDataSeeder para propagar CancellationToken en llamadas a EF Core.
  out_scope:
    - Cambios de lógica de negocio o de seed de datos fuera de agregar soporte de cancelación.
---

# Especificación

Implementar la corrección descrita en `AUDITORIA_2026_23_05.md` para solucionar la deuda técnica relacionada a `AsNoTracking` y falta de propagación de `CancellationToken`.

**Pasos de implementación:**
1. En `IAdminAuthService.cs`, agregar el parámetro opcional `CancellationToken cancellationToken = default` al método `AuthenticateAsync`.
2. En `AdminAuthService.cs`, usar `.AsNoTracking()` y propagar el token a `FirstOrDefaultAsync(cancellationToken)`.
3. En `AdminLoginHandler.cs`, pasar `cancellationToken` a `AuthenticateAsync`.
4. En `AdminAuthServiceTests.cs` y `AdminLoginHandlerTests.cs`, actualizar llamadas a `AuthenticateAsync`.
5. En `AdminJsonDataSeeder.cs`, propagar `cancellationToken` a `ToListAsync` y métodos similares asíncronos.
