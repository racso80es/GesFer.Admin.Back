1. **Modificar firmas en `AdminJsonDataSeeder.cs` para soportar `CancellationToken`**
   - Actualizar los métodos `SeedAllAsync`, `SeedUsersAsync`, `SeedLanguagesAsync`, `SeedCountriesAsync`, `SeedStatesAsync`, `SeedCitiesAsync`, `SeedPostalCodesAsync`, `SeedAdminUsersAsync`, `SeedCompaniesAsync` para aceptar `CancellationToken cancellationToken = default`.
2. **Añadir `CancellationToken` y `AsNoTracking` a las operaciones de Entity Framework en `AdminJsonDataSeeder.cs`**
   - Modificar las llamadas de `ToListAsync()` para incluir `cancellationToken` y `AsNoTracking()`.
   - Modificar las llamadas de `SaveChangesAsync()` para incluir `cancellationToken`.
3. **Modificar firmas en `WebAppExtensions.cs` para propagar el `CancellationToken` si es posible**
   - Verificar si `SeedAllAsync()` es llamado con `CancellationToken` y propagarlo.
4. **Modificar `AdminAuthService.cs` y `AuditLogService.cs`**
   - Actualizar los métodos que realizan llamadas a EF Core para propagar `CancellationToken`.
5. **Comprobar si hay otras llamadas a EF Core sin `CancellationToken`**
   - Analizar el resto de la capa Application/Infrastructure y añadir la propagación donde falte.
6. **Ejecutar pruebas unitarias**
   - Usar `dotnet test src/GesFer.Admin.Back.sln --filter Category!=E2E` para asegurar que todo compila y pasa correctamente.
7. **Completar pasos pre-commit**
   - Ejecutar `pre_commit_instructions` para asegurar las validaciones y creación de archivos necesarios.
   - Guardar los cambios.
