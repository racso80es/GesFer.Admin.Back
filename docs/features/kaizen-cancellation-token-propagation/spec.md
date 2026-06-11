---
base:
  - src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs
  - src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs
  - src/GesFer.Admin.Back.Application/Common/Interfaces/IAdminAuthService.cs
scope:
  in_scope:
    - Propagación de CancellationToken a métodos FirstOrDefaultAsync, ToListAsync y otras llamadas a base de datos
  out_scope:
    - Otras refactorizaciones no relacionadas con asincronía.
---

# Especificación Técnica

Se ha identificado que hay llamadas a EF Core que no propagan el CancellationToken.

## Implementación
- Se modificará `AdminAuthService.cs` para propagar el token a `FirstOrDefaultAsync()`.
- Se modificará `IAdminAuthService.cs` para aceptar el parámetro `CancellationToken`.
- Se modificará `AdminJsonDataSeeder.cs` para propagar el token a `ToListAsync()`.
