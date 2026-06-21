---
type: spec
feature_name: correccion-2026-06-21
base:
  - src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs
  - src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs
  - src/GesFer.Admin.Back.Application/Common/Interfaces/IAdminAuthService.cs
scope:
  in_scope:
    - Agregar `CancellationToken` en `IAdminAuthService.AuthenticateAsync` y pasarlo hasta `FirstOrDefaultAsync`.
    - Agregar `AsNoTracking()` en las consultas de validación y de sólo lectura de EF Core.
    - Actualizar llamadas de `IAdminAuthService` en `AdminLoginHandler.cs` (pasar el Cancellation token).
  out_scope:
    - Refactorizaciones completas de la arquitectura fuera de los archivos afectados directamente.
---
# Especificación de Cambios

1. **IAdminAuthService & AdminAuthService:**
   - Se debe cambiar la firma a `Task<AdminUser?> AuthenticateAsync(string username, string password, CancellationToken cancellationToken = default);`
   - Se debe aplicar `.AsNoTracking()` antes de `.FirstOrDefaultAsync(cancellationToken)`.

2. **AdminLoginHandler:**
   - La llamada a `_authService.AuthenticateAsync` debe recibir el parámetro extra del `CancellationToken` del MediatR context.

3. **AdminJsonDataSeeder:**
   - Modificar las consultas de `Select(x => x.Id).ToListAsync()` para usar `AsNoTracking().Select(x => x.Id).ToListAsync(cancellationToken)` en todos los métodos pertinentes.
   - Dado que los métodos no tienen `CancellationToken` actualmente como parámetro explícito en todos los métodos de semilla, modificarlos agregando un parámetro opcional o un overload para inyectar un CancellationToken donde proceda. (Si el cambio en la firma de Seeder implica que sea problemático, sólo se pasará CancellationToken de un parámetro CancellationToken.None a los EF core si no hay forma de inyectarlo, pero se debe usar .AsNoTracking() para lectura de ID y registros existentes).
