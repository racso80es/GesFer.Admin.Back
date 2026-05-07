---
feature_name: kaizen-ef-asnotracking
---
# Objetivos: Optimización Termodinámica EF Core (AsNoTracking)

## Objetivo
Resolver la fuga de rendimiento detectada en la capa de aplicación, agregando explícitamente `.AsNoTracking()` a todos los Query Handlers para evitar cargar entidades de solo lectura en el Change Tracker de Entity Framework Core.

## Alcance
- Modificar `GetAllCompaniesHandler`, `GetCompanyByIdHandler`, `GetCompanyByNameHandler`, `GetAllUsersHandler` y `GetUserByIdHandler`.
- Mantener actualizado el log de evolución.
