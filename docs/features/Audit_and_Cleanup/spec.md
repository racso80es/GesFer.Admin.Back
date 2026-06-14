---
feature_name: Audit_and_Cleanup
created: 2026-06-14
base:
  - src/GesFer.Admin.Back.IntegrationTests/AdminAuthIntegrationTests.cs
  - src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs
  - src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs
scope:
  in_scope:
    - Adding CancellationToken to ToListAsync() in AdminAuthIntegrationTests.cs
    - Adding cancellationToken to ToListAsync() in AdminJsonDataSeeder.cs
    - Adding cancellationToken to FirstOrDefaultAsync() in AdminAuthService.cs
  out_scope:
    - Adding cancellation tokens to methods where they are not available in the context (though Seeder and Service methods usually accept one).
---

# Specification: Audit and Cleanup

We will update specific files to pass `CancellationToken` to EF Core async operations.
