---
type: objectives
feature_name: correccion-2026-05-25
---

# Objectives
1. Add `CancellationToken` and `.AsNoTracking()` to `AdminAuthService.AuthenticateAsync`.
2. Add `CancellationToken` to all async Entity Framework operations in `AdminJsonDataSeeder.cs`.
3. Update `IAdminAuthService.cs` interface to accept `CancellationToken`.
4. Update `AdminLoginHandler.cs` to pass `CancellationToken` to `AuthenticateAsync`.
