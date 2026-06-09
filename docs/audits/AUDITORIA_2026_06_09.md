1. Métricas de Salud (0-100%)
Arquitectura: 90% | Nomenclatura: 95% | Estabilidad Async: 85%

2. Pain Points (🔴 Críticos / 🟡 Medios)
Hallazgo: [Estabilidad Async] Falta de CancellationToken en consultas asíncronas
Ubicación: src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs línea 37
Descripción: El método `AuthenticateAsync` no acepta ni propaga un `CancellationToken` hacia la llamada a `.FirstOrDefaultAsync()`, lo que puede llevar a bloqueos de hilos en la base de datos si la solicitud HTTP es cancelada.

Hallazgo: [Estabilidad Async] Falta de propagación de CancellationToken en AdminJsonDataSeeder
Ubicación: src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs (múltiples líneas)
Descripción: El servicio `AdminJsonDataSeeder` utiliza llamadas a `.ToListAsync()` y `.SaveChangesAsync()` en Entity Framework sin propagar un `CancellationToken`, lo que reduce la estabilidad de operaciones de larga duración.

3. Acciones Kaizen (Hoja de Ruta para el Executor)
**Acción 1: Propagación de CancellationToken en IAdminAuthService y AdminAuthService**
- **Instrucciones:** Modificar la interfaz `IAdminAuthService` y su implementación `AdminAuthService` para aceptar un `CancellationToken` (con valor por defecto `default`). Propagar este token a la llamada `FirstOrDefaultAsync`.
- **Snippet de código:**
```csharp
// IAdminAuthService.cs
Task<AdminUser?> AuthenticateAsync(string username, string password, CancellationToken cancellationToken = default);

// AdminAuthService.cs
public async Task<AdminUser?> AuthenticateAsync(string username, string password, CancellationToken cancellationToken = default)
{
    // ...
    var adminUser = await _context.AdminUsers
        // ...
        .FirstOrDefaultAsync(cancellationToken);
    // ...
}
```
- **DoD:** El servicio de autenticación soporta cancelación. Las pruebas de integración se ejecutan correctamente pasando `CancellationToken`.

**Acción 2: Propagación de CancellationToken en AdminJsonDataSeeder**
- **Instrucciones:** Añadir `CancellationToken cancellationToken = default` a todos los métodos públicos y privados relevantes en `AdminJsonDataSeeder` (ej. `SeedAllAsync`, `SeedCountriesAsync`, etc.) y propagarlo a todas las llamadas `.ToListAsync(cancellationToken)` y `.SaveChangesAsync(cancellationToken)`.
- **DoD:** Todas las llamadas asíncronas de EF Core en `AdminJsonDataSeeder` reciben un `CancellationToken`. El proyecto compila y los tests pasan sin errores.
