---
type: spec
feature_name: correccion-2026-06-15
base:
  - src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs
  - src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs
  - src/GesFer.Admin.Back.UnitTests/Services/AuditLogServiceTests.cs
  - src/GesFer.Admin.Back.UnitTests/Handlers/Company/CreateCompanyHandlerTests.cs
  - src/GesFer.Admin.Back.UnitTests/Handlers/Company/DeleteCompanyHandlerTests.cs
  - src/GesFer.Admin.Back.IntegrationTests/AdminAuthIntegrationTests.cs
  - src/GesFer.Admin.Back.E2ETests/AdminApiE2ETests.cs
scope:
  in_scope:
    - Aplicar CancellationToken a llamadas de EF Core.
    - Aplicar .AsNoTracking() a operaciones de solo lectura en AdminAuthService.cs.
  out_scope:
    - Modificar la lógica de negocio de los handlers.
---

# Especificación

1. **AdminAuthService.cs:**
   - Modificar las consultas de `_context.AdminUsers` en `AuthenticateAdminAsync` para incluir `.AsNoTracking()`.
   - Pasar `cancellationToken` en la función, usar sobrecarga predeterminada y pasarlo a `.FirstOrDefaultAsync(cancellationToken)`.

2. **AdminJsonDataSeeder.cs:**
   - Agregar parámetro opcional `CancellationToken cancellationToken = default` en los métodos de `AdminJsonDataSeeder.cs` donde aplique, o simplemente pasarlo en llamadas EF, y propagarlo (o usar default si seeder no tiene). En este caso, ya que es IAdminJsonDataSeeder, veamos su interfaz antes de modificar la firma. Si la interfaz no tiene CT, y el seeder lo llama `SeedDataAsync(CancellationToken cancellationToken = default)`, pasaremos el `cancellationToken` a las llamadas EF Core.

3. **Tests:**
   - E2E / Integration / UnitTests: Pasar `CancellationToken.None` a los métodos de EF Core que estén en tests (como `FirstOrDefaultAsync(CancellationToken.None)` y `ToListAsync(CancellationToken.None)`).
