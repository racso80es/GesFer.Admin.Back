---
title: "Especificación técnica para Kaizen AsNoTracking"
feature_id: "kaizen-asnotracking"
base:
  - "src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs"
  - "src/GesFer.Admin.Back.UnitTests/Services/AuditLogServiceTests.cs"
scope:
  in_scope:
    - "Añadir .AsNoTracking() a AuthenticateAsync en AdminAuthService"
    - "Añadir .AsNoTracking() a logs en AuditLogServiceTests"
  out_scope:
    - "Otras funciones no detectadas por grep"
---

# Especificación

## Modificaciones de código
1. En `src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs`, método `AuthenticateAsync`:
```csharp
        var adminUser = await _context.AdminUsers
            .AsNoTracking()
            .Where(u => u.Username == normalizedUsername
                && u.IsActive
                && u.DeletedAt == null)
            .FirstOrDefaultAsync();
```

2. En `src/GesFer.Admin.Back.UnitTests/Services/AuditLogServiceTests.cs`:
```csharp
        var log = await context.AuditLogs.AsNoTracking().FirstOrDefaultAsync();
```
```csharp
        var log = await context.AuditLogs.AsNoTracking().FirstAsync();
```
