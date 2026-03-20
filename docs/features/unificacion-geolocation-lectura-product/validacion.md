---
feature_name: unificacion-geolocation-lectura-product
branch: feat/unificacion-geolocation-lectura-product
base_branch: main
global: pass
blocking: false
checks:
  - name: dotnet build
    result: pass
    message: GesFer.Admin.Back.sln sin errores ni advertencias (última ejecución local).
  - name: dotnet test
    result: pass
    message: Unit (60), Integration (30 activos, 1 omitido), E2E (4), Architecture (5).
  - name: swagger_schemas_manual
    result: pending
    message: "Comprobación manual (plan T11): en Swagger, esquemas CityGeoReadDto sin Code; PostalCodeGeoReadDto sin Name; CountryGeoReadDto sin languageId."
git_changes:
  files_added:
    - src/GesFer.Admin.Back.Api/Controllers/GeolocationController.cs
    - src/GesFer.Admin.Back.ArchitectureTests/GesFer.Admin.Back.ArchitectureTests.csproj
    - src/GesFer.Admin.Back.ArchitectureTests/LayerDependencyTests.cs
    - src/GesFer.Admin.Back.ArchitectureTests/README.md
    - docs/features/unificacion-geolocation-lectura-product/implementation.md
    - docs/features/unificacion-geolocation-lectura-product/execution.md
    - docs/features/unificacion-geolocation-lectura-product/validacion.md
  files_modified:
    - src/GesFer.Admin.Back.sln
    - src/GesFer.Admin.Back.IntegrationTests/GeoControllerTests.cs
    - src/GesFer.Admin.Back.UnitTests/Handlers/Geo/GetAllCountriesHandlerTests.cs
    - src/GesFer.Admin.Back.UnitTests/Handlers/Geo/GetStatesByCountryIdHandlerTests.cs
    - src/GesFer.Admin.Back.UnitTests/Handlers/Geo/GetCitiesByStateIdHandlerTests.cs
    - src/GesFer.Admin.Back.UnitTests/Handlers/Geo/GetPostalCodesByCityIdHandlerTests.cs
    - docs/postman/GesFer.Admin.Back.API.postman_collection.json
    - scripts/README-E2E.md
    - docs/features/unificacion-geolocation-lectura-product/spec.md
    - docs/evolution/EVOLUTION_LOG.md
  files_deleted:
    - src/GesFer.Admin.Back.Api/Controllers/CountriesController.cs
    - src/GesFer.Admin.Back.Api/Controllers/StatesController.cs
---

# Validación — Unificación geolocalización

## Resumen

La solución compila y la batería de tests automatizados pasa. Queda explícito como **pendiente manual** el repaso de esquemas en Swagger (T11), coherente con el plan de la feature.

## Aviso a consumidores

- Rutas antiguas `/api/countries` y `/api/states/...` **eliminadas**.
- Respuestas JSON de país **sin** `languageId`.

Tras merge, actualizar clientes **Product** y cualquier herramienta (Postman ya referenciada en `docs/postman/`).
