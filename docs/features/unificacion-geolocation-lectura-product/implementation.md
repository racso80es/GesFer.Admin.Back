---
feature_name: unificacion-geolocation-lectura-product
created: 2026-03-20
updated: 2026-03-20
items:
  - id: P3-controller
    action: create
    path: src/GesFer.Admin.Back.Api/Controllers/GeolocationController.cs
    location: Api
    proposal: "Cinco GET bajo Route(\"api/geolocation\"), AuthorizeSystemOrAdmin, IMediator, manejo de errores alineado al resto de controladores."
    dependencies: []
  - id: P3-remove-legacy
    action: delete
    path: src/GesFer.Admin.Back.Api/Controllers/CountriesController.cs
    location: Api
    proposal: Sustituido por GeolocationController.
    dependencies: [P3-controller]
  - id: P3-remove-states
    action: delete
    path: src/GesFer.Admin.Back.Api/Controllers/StatesController.cs
    location: Api
    proposal: Sustituido por GeolocationController.
    dependencies: [P3-controller]
  - id: P4-project
    action: create
    path: src/GesFer.Admin.Back.ArchitectureTests/
    location: tests
    proposal: "Proyecto xUnit con reglas de referencia entre ensamblados Domain, Application, Infrastructure, Api."
    dependencies: [P3-controller]
  - id: P5-integration
    action: modify
    path: src/GesFer.Admin.Back.IntegrationTests/GeoControllerTests.cs
    location: IntegrationTests
    proposal: Rutas /api/geolocation/... y test de postal-codes por ciudad (seed Madrid).
    dependencies: [P3-controller]
  - id: P5-unit-geo
    action: modify
    path: src/GesFer.Admin.Back.UnitTests/Handlers/Geo/
    location: UnitTests
    proposal: Cobertura IsActive, país con DeletedAt pero activo, exclusión inactivos en states/cities/postal.
    dependencies: []
  - id: P5-postman-readme
    action: modify
    path: docs/postman/GesFer.Admin.Back.API.postman_collection.json
    location: docs
    proposal: Carpeta Geolocation con URLs nuevas.
    dependencies: [P3-controller]
---

# Implementación — Unificación geolocalización (lectura)

## Resumen técnico

- **P3:** `GeolocationController` con rutas del spec §2; eliminación de controladores legacy.
- **P4:** `GesFer.Admin.Back.ArchitectureTests` + `LayerDependencyTests` (reflexión) + README.
- **P5:** Tests integración y unitarios ampliados; Postman y `scripts/README-E2E.md` alineados a `/api/geolocation`.
- **P6:** Artefactos `implementation.md`, `execution.md`, `validacion.md`.

## Rutas definitivas (recordatorio)

| Método | Ruta |
|--------|------|
| GET | `/api/geolocation/countries` |
| GET | `/api/geolocation/countries/{countryId}` |
| GET | `/api/geolocation/countries/{countryId}/states` |
| GET | `/api/geolocation/states/{stateId}/cities` |
| GET | `/api/geolocation/cities/{cityId}/postal-codes` |

## Migración consumidores (Product / clientes)

**Breaking:** ya no existen `/api/countries`, `/api/states/...`. JSON de país sin `languageId`. Conv Coord despliegue con front Product.
