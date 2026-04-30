---
feature_name: user-crud
created: "2026-04-29"
items:
  - id: I0
    action: "Crear rama feat/user-crud"
    path: "scripts/skills/iniciar-rama/"
    location: "iniciar_rama.exe (cápsula) o Iniciar-Rama.bat (humano)"
    proposal: "Ejecutar skill iniciar-rama con BranchType=feat, BranchName=user-crud."
    dependencies:
      - "Ejecutables instalados vía scripts/skills-rs/install.ps1"
  - id: I1
    action: "Entidad EF + configuración Users"
    path:
      - "src/GesFer.Admin.Back.Domain/Entities/User.cs"
      - "src/GesFer.Admin.Back.Infrastructure/Data/Configurations/UserConfiguration.cs"
      - "src/GesFer.Admin.Back.Infrastructure/Data/AdminDbContext.cs"
      - "src/GesFer.Admin.Back.Application/Common/Interfaces/IApplicationDbContext.cs"
    location:
      - "AdminDbContext.DbSet<User>"
      - "UserConfiguration (índice único CompanyId+Username)"
      - "QueryFilter soft delete por BaseEntity.DeletedAt (DbContextExtensions)"
    proposal: "Añadir entidad `User` basada en `BaseEntity`, configuración EF en `UserConfiguration` y exponer `DbSet<User>` vía `AdminDbContext`/`IApplicationDbContext`."
    dependencies: []
  - id: I2
    action: "Seeds Users via JsonDataSeeder"
    path:
      - "src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs"
      - "src/GesFer.Admin.Back.Infrastructure/Data/Seeds/*.json"
    location:
      - "AdminJsonDataSeeder.SeedAllAsync()"
      - "WebAppExtensions.RunMigrationsAndSeeds*"
    proposal: "En este repo existen seeds solo para el dominio Admin (Companies, AdminUsers, geo). No se definieron seeds para `Users` en esta feature; requiere decisión explícita (y/o implementación en Product)."
    dependencies:
      - "Decisión de alcance: ¿Users pertenece a Admin.Back o Product.Back?"
  - id: I3
    action: "Handlers CRUD"
    path:
      - "src/GesFer.Admin.Back.Application/Commands/User/*"
      - "src/GesFer.Admin.Back.Application/Handlers/User/*"
      - "src/GesFer.Admin.Back.Application/DTOs/User/UserDtos.cs"
    location:
      - "CreateUserHandler, UpdateUserHandler, DeleteUserHandler, GetAllUsersHandler, GetUserByIdHandler"
    proposal: "Implementar commands/handlers MediatR para CRUD de `User` con unicidad de `Username` por `CompanyId`."
    dependencies:
      - "Validación CompanyId: en Admin.Back se valida contra tabla Companies (no IAdminApiClient)."
  - id: I4
    action: "Endpoints /api/User"
    path:
      - "src/GesFer.Admin.Back.Api/Controllers/UserController.cs"
      - "src/GesFer.Admin.Back.Api/Security/ClaimsExtensions.cs"
    location:
      - "Route: /api/User"
      - "Auth: [AuthorizeSystemOrAdmin]"
      - "CompanyId: claim (CompanyId/companyId) con fallback query para llamadas System"
    proposal: "Exponer CRUD en `UserController` usando MediatR y resolución de CompanyId desde claims."
    dependencies: []
  - id: I5
    action: "Validación pre-PR"
    path: "docs/features/user-crud/validacion.md"
    location: "checks + git_changes"
    proposal: "Build/tests + criterios de validación de spec."
    dependencies: []
---

## Touchpoints descubiertos

- **DbContext EF**: `src/GesFer.Admin.Back.Infrastructure/Data/AdminDbContext.cs`
- **Soft delete global**: `src/GesFer.Admin.Back.Infrastructure/Repository/DbContextExtensions.cs` (query filter por `DeletedAt`)
- **Seeder Admin**: `src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs` + `src/GesFer.Admin.Back.Infrastructure/Data/Seeds/`
- **Estructura CQRS**: `src/GesFer.Admin.Back.Application/Commands/*` + `src/GesFer.Admin.Back.Application/Handlers/*`
