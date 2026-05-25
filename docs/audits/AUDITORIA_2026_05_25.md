1. Métricas de Salud (0-100%)
Arquitectura: 90% | Nomenclatura: 100% | Estabilidad Async: 85%

2. Pain Points (🔴 Críticos / 🟡 Medios)
Hallazgo: [Falta CancellationToken y AsNoTracking en AdminAuthService]
Ubicación: src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs:37

Hallazgo: [Falta CancellationToken en llamadas en AdminJsonDataSeeder]
Ubicación: src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs:204,208,371,419,472,525,580,645,769

3. Acciones Kaizen (Hoja de Ruta para el Executor)
- **AdminAuthService.cs:**
  Añadir `cancellationToken` como parámetro a `AuthenticateAsync` y pasarlo a `FirstOrDefaultAsync`. Añadir `AsNoTracking()` ya que es una consulta de solo lectura.
  ```csharp
    public async Task<AdminUser?> AuthenticateAsync(string username, string password, CancellationToken cancellationToken = default)
    {
        // ...
        var adminUser = await _context.AdminUsers
            .AsNoTracking()
            .Where(u => u.Username == normalizedUsername
                && u.IsActive
                && u.DeletedAt == null)
            .FirstOrDefaultAsync(cancellationToken);
        // ...
    }
  ```
  **DoD:** La consulta de autenticación utiliza `AsNoTracking` y el `cancellationToken` se propaga.

- **AdminJsonDataSeeder.cs:**
  Añadir `CancellationToken` a los métodos de seedeo y a todas las llamadas asíncronas de Entity Framework (`ToListAsync`, `SaveChangesAsync`, etc.).

  **DoD:** Todas las llamadas a la base de datos en `AdminJsonDataSeeder` reciben y propagan un `CancellationToken`.

- **IAdminAuthService.cs:**
  Actualizar la interfaz `IAdminAuthService` para incluir el parámetro `CancellationToken` en `AuthenticateAsync`.
  ```csharp
  Task<AdminUser?> AuthenticateAsync(string username, string password, CancellationToken cancellationToken = default);
  ```
  **DoD:** La interfaz refleja la firma actualizada con `CancellationToken`.

- **AdminLoginHandler.cs:**
  Pasar `cancellationToken` a la llamada a `AuthenticateAsync` de `_authService`.
  ```csharp
  var adminUser = await _authService.AuthenticateAsync(request.UserName, request.Password, cancellationToken);
  ```
  **DoD:** `AdminLoginHandler` pasa el `cancellationToken` a `AuthenticateAsync`.
