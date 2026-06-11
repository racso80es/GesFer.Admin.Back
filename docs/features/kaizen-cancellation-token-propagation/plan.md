---
type: plan
---
# Plan de Ejecución

1. Usar `replace_with_git_merge_diff` en `IAdminAuthService.cs` para añadir el parámetro `CancellationToken cancellationToken = default` a `AuthenticateAsync`.
<<<<<<< SEARCH
    Task<AdminUser?> AuthenticateAsync(string username, string password);
=======
    Task<AdminUser?> AuthenticateAsync(string username, string password, CancellationToken cancellationToken = default);
>>>>>>> REPLACE

2. Usar bash (`cat`) para verificar la modificación en `IAdminAuthService.cs`.

3. Usar `replace_with_git_merge_diff` en `AdminAuthService.cs` para añadir el parámetro `CancellationToken cancellationToken = default` al método `AuthenticateAsync` y propagar `cancellationToken` a la llamada de `FirstOrDefaultAsync`.
<<<<<<< SEARCH
    public async Task<AdminUser?> AuthenticateAsync(string username, string password)
    {
        if (string.IsNullOrWhiteSpace(username) || string.IsNullOrWhiteSpace(password))
            return null;

        // Normalizar username
        var normalizedUsername = username.Trim();

        // Buscar el usuario administrativo
        var adminUser = await _context.AdminUsers
            .Where(u => u.Username == normalizedUsername
                && u.IsActive
                && u.DeletedAt == null)
            .FirstOrDefaultAsync();

        if (adminUser == null)
=======
    public async Task<AdminUser?> AuthenticateAsync(string username, string password, CancellationToken cancellationToken = default)
    {
        if (string.IsNullOrWhiteSpace(username) || string.IsNullOrWhiteSpace(password))
            return null;

        // Normalizar username
        var normalizedUsername = username.Trim();

        // Buscar el usuario administrativo
        var adminUser = await _context.AdminUsers
            .Where(u => u.Username == normalizedUsername
                && u.IsActive
                && u.DeletedAt == null)
            .FirstOrDefaultAsync(cancellationToken);

        if (adminUser == null)
>>>>>>> REPLACE

4. Usar bash (`cat`) para verificar la modificación en `AdminAuthService.cs`.

5. Usar `replace_with_git_merge_diff` en `AdminLoginHandler.cs` para propagar el `cancellationToken` a la llamada `await _authService.AuthenticateAsync`.
<<<<<<< SEARCH
            }

            var adminUser = await _authService.AuthenticateAsync(request.UserName, request.Password);

            if (adminUser == null)
=======
            }

            var adminUser = await _authService.AuthenticateAsync(request.UserName, request.Password, cancellationToken);

            if (adminUser == null)
>>>>>>> REPLACE

6. Usar bash (`cat`) para verificar la modificación en `AdminLoginHandler.cs`.

7. Ejecutar `git add .` en bash session.

8. Ejecutar `git commit -m "feat: propagate CancellationToken in EF Core and Async operations"` en bash session.

9. Ejecutar `dotnet build src/GesFer.Admin.Back.sln` en bash session.

10. Ejecutar `dotnet test src/GesFer.Admin.Back.sln --filter Category!=E2E` en bash session para comprobar que no haya test rotos.

11. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

12. Submit the change.
