---
feature_name: unificacion-geolocation-lectura-product
created: 2026-03-20
updated: 2026-03-20
base:
  - objectives.md
  - clarify.md
scope:
  in_scope:
    - Controlador único `GeolocationController` bajo `api/geolocation`.
    - Eliminación de `CountriesController` y `StatesController`.
    - DTOs de lectura por entidad (estructura de dominio; City sin Code; PostalCode sin Name).
    - Visibilidad geo: `IgnoreQueryFilters` + solo `IsActive` y FK de padre (§4).
    - Tests de integración y unitarios al nuevo contrato.
  out_scope:
    - Paginación y búsqueda textual de códigos postales.
    - CRUD administrativo geo.
    - Exposición de LanguageId en Country para Product.
functional_requirements:
  - id: FR-01
    text: GET países con IsActive true; orden por nombre.
  - id: FR-02
    text: GET país por Id con IsActive true; 404 si no cumple.
  - id: FR-03
    text: GET states por countryId e IsActive true.
  - id: FR-04
    text: GET cities por stateId e IsActive true.
  - id: FR-05
    text: GET postal-codes por cityId e IsActive true.
  - id: FR-06
    text: Respuestas JSON con DTOs según §3; Swagger actualizado.
non_functional_requirements:
  - id: NFR-01
    text: AsNoTracking en consultas de lectura.
  - id: NFR-02
    text: "[AuthorizeSystemOrAdmin] en el controlador."
touchpoints:
  - path: src/GesFer.Admin.Back.Api/Controllers/GeolocationController.cs
    note: Controlador único; rutas bajo api/geolocation.
  - path: src/GesFer.Admin.Back.ArchitectureTests/
    note: Tests de dependencias entre capas (P4).
  - path: src/GesFer.Admin.Back.Application/DTOs/Geo/GeoDtos.cs
    note: DTOs lectura por nivel (§3); eliminar shapes homogéneos duplicados si existían.
  - path: src/GesFer.Admin.Back.Application/Commands/Geo/GeoQueries.cs
    note: Comando postal por CityId.
  - path: src/GesFer.Admin.Back.Application/Handlers/Geo/GeoHandlers.cs
    note: IgnoreQueryFilters + IsActive + FK padre; sin DeletedAt; handler postal.
  - path: src/GesFer.Admin.Back.IntegrationTests/GeoControllerTests.cs
    note: Rutas /api/geolocation/...
  - path: src/GesFer.Admin.Back.UnitTests/Handlers/Geo/
    note: Ajustar escenarios (IsActive; no basar exclusión solo en DeletedAt).
validation_criteria:
  - Build y tests verdes.
  - clarify.md C-01..C-08 y ampliaciones DTO/filtrado de este documento.
---

# Especificación técnica — Unificación controlador geolocalización (lectura)

## 1. Estado previo (reemplazado por esta feature)

Antes existían `CountriesController` y `StatesController` bajo `/api/countries` y `/api/states`. **Se eliminan.** Los handlers actuales filtran con `DeletedAt == null`; esta feature **cambia** el criterio a **solo `IsActive`** en los predicados explícitos (§4).

## 2. API definitiva

**Ruta base:** `api/geolocation`  
**Autorización:** `[AuthorizeSystemOrAdmin]`.

| Método | Ruta | Respuesta |
|--------|------|-----------|
| GET | `/api/geolocation/countries` | `List<CountryGeoReadDto>` |
| GET | `/api/geolocation/countries/{countryId}` | `CountryGeoReadDto` o 404 |
| GET | `/api/geolocation/countries/{countryId}/states` | `List<StateGeoReadDto>` |
| GET | `/api/geolocation/states/{stateId}/cities` | `List<CityGeoReadDto>` |
| GET | `/api/geolocation/cities/{cityId}/postal-codes` | `List<PostalCodeGeoReadDto>` |

*Nombres de tipo* (`CountryGeoReadDto`, etc.) son orientativos; pueden coincidir con nombres existentes si el ensamblado ya los define, siempre que la forma pública sea la de §3.

## 3. Contrato de DTOs (por estructura de entidad)

Propiedades **solo si existen** en el dominio o son necesarias para la jerarquía; **sin** campo genérico `ParentId`.

### 3.1 `CountryGeoReadDto`

| Campo | Tipo |
|-------|------|
| `Id` | Guid |
| `Name` | string |
| `Code` | string |

### 3.2 `StateGeoReadDto`

| Campo | Tipo |
|-------|------|
| `Id` | Guid |
| `CountryId` | Guid |
| `Name` | string |
| `Code` | string? |

### 3.3 `CityGeoReadDto`

| Campo | Tipo |
|-------|------|
| `Id` | Guid |
| `StateId` | Guid |
| `Name` | string |

**Sin** propiedad `Code` (la entidad `City` no la tiene).

### 3.4 `PostalCodeGeoReadDto`

| Campo | Tipo |
|-------|------|
| `Id` | Guid |
| `CityId` | Guid |
| `Code` | string |

**Sin** propiedad `Name` (la entidad `PostalCode` no la tiene).

## 4. Reglas de filtrado (handlers de geolocalización)

**Cerrado en clarify (C-08):** el conjunto visible lo define **solo** `IsActive == true` y la **clave del padre** en cada listado; **no** se filtra por `DeletedAt`.

1. Invocar **`IgnoreQueryFilters()`** en el `DbSet` usado por cada handler geo (las entidades heredan filtro global por `DeletedAt` en `DbContextExtensions.ConfigureAdminEntities`; sin `IgnoreQueryFilters`, ese filtro seguiría actuando de facto como filtro por borrado).
2. **Predicados por caso:**
   - Países (lista / por id): `IsActive == true` (+ `Id` en el by-id).
   - Estados por país: `CountryId == request` **y** `IsActive == true`.
   - Ciudades por estado: `StateId == request` **y** `IsActive == true`.
   - Códigos postales por ciudad: `CityId == request` **y** `IsActive == true`.
3. **No** añadir `DeletedAt == null` (ni enlazar visibilidad al soft-delete global).

Orden: por `Name` (countries, states, cities); postal codes por `Code`.

## 5. Capa de aplicación

- Comandos existentes de países / estados / ciudades: actualizar proyección a los DTOs §3 y filtros §4.
- Nuevo comando: listado de `PostalCode` por `CityId` con DTO §3.4 y filtros §4.

## 6. Controlador

- Clase **`GeolocationController`**, `[Route("api/geolocation")]`, `[AuthorizeSystemOrAdmin]`.

## 7. Pruebas

- **Integración:** rutas bajo `/api/geolocation/...`.
- **Unitarias:** exclusión/inclusión basada en **`IsActive`**; no depender de que el ejemplo “borrado” use solo `DeletedAt` sin acotar `IsActive`. Ajustar datos semilla en tests si hace falta.

## 8. Riesgo / migración

**Breaking change** en rutas y en forma de JSON respecto a clientes que esperaban un DTO homogéneo o `LanguageId` en país. **Cambio de semántica:** registros con `DeletedAt != null` pero `IsActive == true` podrían exponerse (antes el filtro global los ocultaba).

---

*SPEC alineada con `clarify.md` (C-01..C-08). Filtrado geo: §4 + `plan.md`.*
