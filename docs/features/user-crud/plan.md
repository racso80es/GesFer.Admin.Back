---
feature_name: user-crud
created: "2026-04-29"
phases:
  - id: P0
    name: Preparar rama
    goal: "Crear/checkout `feat/user-crud` desde troncal actualizada."
  - id: P1
    name: Persistencia (EF Core)
    goal: "Agregar entidad/configuración `Users` + constraints/índices + soft delete."
  - id: P2
    name: Seeds
    goal: "Extender JsonDataSeeder para `Users` y compatibilidad admin123 determinista."
  - id: P3
    name: Aplicación (CQRS/Handlers)
    goal: "Crear handlers Create/Update/Delete/GetAll/GetById con reglas."
  - id: P4
    name: API (Controllers/Endpoints)
    goal: "Exponer `/api/User` CRUD con `[Authorize]` y scope por CompanyId."
  - id: P5
    name: Validación
    goal: "Build + tests + verificación multi-tenant y soft delete."
tasks:
  - "Localizar estructura actual (carpetas/proyectos) donde viven Entities, DTOs, Handlers y Controllers."
  - "Implementar tabla Users y su configuración (incluye índice único CompanyId+Username)."
  - "Implementar M:N (UserGroups, UserPermissions) si aplica en este microservicio; si ya existen, integrar."
  - "Integrar `IAdminApiClient.GetCompanyAsync` en Create/Update (validación CompanyId)."
  - "Asegurar que GET/POST/PUT/DELETE aplican CompanyId del token/contexto."
  - "Asegurar que PasswordHash no se devuelve y que Update solo rehashea si hay Password."
---

## Notas de ejecución

- Este plan se completará con rutas concretas en `implementation.md` una vez identificados los touchpoints reales en el código.
