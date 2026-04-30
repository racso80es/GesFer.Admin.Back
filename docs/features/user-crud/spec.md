---
feature_name: user-crud
created: "2026-04-29"
base:
  - docs/features/user-crud/objectives.md
  - c:\Proyectos\GesFer.Product.Back\docs\DocumentacionUsuarios.md
scope:
  in_scope:
    - Users: tabla + entidad (BaseEntity) con soft delete
    - Seeds de Users via JsonDataSeeder (compatibilidad admin123)
    - CRUD handlers + DTOs + endpoints /api/User
    - Multi-tenant por CompanyId extraído del token/contexto
  out_scope:
    - Cambios de arquitectura entre microservicios (solo integración existente)
functional_requirements:
  - Crear usuario validando existencia de CompanyId vía IAdminApiClient.GetCompanyAsync
  - Enforce unicidad de Username dentro de CompanyId
  - Hash de contraseña con BCrypt (work factor 11)
  - Actualizar usuario (hash solo si se provee Password; set UpdatedAt = UtcNow; soportar IsActive)
  - Borrado lógico (DeletedAt = UtcNow; IsActive = false)
  - Consultas limitadas al CompanyId del invocador (aislamiento multi-tenant)
non_functional_requirements:
  - Mantener compatibilidad con seed determinista para admin123
  - No exponer PasswordHash ni secretos en respuestas
validation_criteria:
  - No existen dos Users con el mismo (CompanyId, Username)
  - DELETE no elimina físicamente registros
  - GET /api/User retorna únicamente usuarios del CompanyId del token
---

## Modelo de datos: `Users`

Campos y restricciones según la especificación base:

- `Id` (GUID COMB, PK)
- `CompanyId` (obligatorio; FK conceptual a Admin)
- `Username` (nvarchar(100), obligatorio; **índice único compuesto** con `CompanyId`)
- `PasswordHash` (nvarchar(500), obligatorio; BCrypt)
- `FirstName`, `LastName` (nvarchar(100), obligatorios)
- `Email` (nvarchar(200), opcional; VO Email serializado como string)
- `Phone` (nvarchar(50), opcional)
- `Address` (nvarchar(500), opcional)
- FKs opcionales a catálogos locales: `PostalCodeId`, `CityId`, `StateId`, `CountryId`, `LanguageId` (**OnDelete: Restrict**)
- `CreatedAt` (UtcNow), `UpdatedAt` (nullable), `DeletedAt` (nullable), `IsActive` (default true)

Relaciones M:N:

- `UserGroups` (`{UserId, GroupId}` único; cascada en ambas FKs)
- `UserPermissions` (`{UserId, PermissionId}` único; cascada en ambas FKs)

## Seeds

Seeding de usuarios vía `JsonDataSeeder` con:

- Validación de existencia de `CompanyId`
- Re-activación por soft delete si el registro existe
- Contraseña seed `admin123` con hash determinista fijo (ver documento base)

## API: `/api/User`

Contratos mínimos:

- `GET /api/User` → `List<UserDto>`
- `GET /api/User/{id}` → `UserDto`
- `POST /api/User` (CreateUserDto) → `UserDto`
- `PUT /api/User/{id}` (UpdateUserDto) → `UserDto`
- `DELETE /api/User/{id}` → 204

Requisitos:

- `[Authorize]`
- `CompanyId` de **contexto/token** para limitar scope

## DTOs

DTOs según documento base: `UserDto`, `CreateUserDto`, `UpdateUserDto`.

## Touchpoints (pendiente en implementation.md)

- Handlers: `CreateUserCommandHandler`, `UpdateUserCommandHandler`, `DeleteUserCommandHandler`, `GetAllUsersCommandHandler`, `GetUserByIdCommandHandler`
- Integración: `IAdminApiClient.GetCompanyAsync`
