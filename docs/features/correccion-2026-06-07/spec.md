---
type: spec
feature_name: correccion-2026-06-07
base:
  - "docs/audits/AUDITORIA_2026_06_07.md"
scope:
  in_scope:
    - "Actualizar IAdminAuthService.cs"
    - "Actualizar AdminAuthService.cs"
    - "Actualizar referencias a AuthenticateAsync en Handlers/Controladores/Tests"
    - "Actualizar AdminJsonDataSeeder.cs para propagar CancellationToken en todos los métodos Seed*Async"
    - "Actualizar la invocación de SeedAllAsync (u otros) en el código para que pase CancellationToken"
  out_scope:
    - "Refactorizar lógica de seed de datos fuera de añadir cancellation tokens"
    - "Cambiar algoritmos de hash"
---

# Especificación

## Implementación Técnica

1. **AdminAuthService.cs e IAdminAuthService.cs**
   - Modificar la firma del método: `Task<AdminUser?> AuthenticateAsync(string username, string password, CancellationToken cancellationToken = default);`
   - Modificar las llamadas asíncronas de EF Core (`FirstOrDefaultAsync(cancellationToken)`) en la implementación para recibir el token.
   - Encontrar todas las invocaciones a `AuthenticateAsync` (por ejemplo, en `AdminLoginHandler.cs` o sus tests) y asegurarse de que pasen el token adecuado.

2. **AdminJsonDataSeeder.cs**
   - El método `SeedAllAsync` y todos los métodos `Seed*Async` de la clase `AdminJsonDataSeeder` deben recibir `CancellationToken cancellationToken = default` y propagarlo a todas las llamadas de EF Core (como `ToListAsync`, `SaveChangesAsync`, etc.).
   - Actualizar las llamadas de consumo (por ejemplo, en program.cs, tests, controllers si las hay) para propagar el `CancellationToken`.
