---
feature_name: unificacion-geolocation-lectura-product
created: 2026-03-20
purpose: Decisiones de negocio y contrato API con Product (geolocalización lectura).
decisions:
  - id: C-01
    topic: Prefijo de ruta
    resolution: "Todas las rutas bajo `api/geolocation`."
  - id: C-02
    topic: Controladores legacy
    resolution: "Eliminar `CountriesController` y `StatesController`; solo el controlador unificado. Sin periodo de deprecación ni alias."
  - id: C-03
    topic: Autenticación
    resolution: "Mismo mecanismo que el resto de catálogos protegidos: `AuthorizeSystemOrAdmin` (p. ej. `X-Internal-Secret` en entornos de integración)."
  - id: C-04
    topic: Códigos postales
    resolution: "Único modo de lectura: listado bajo una ciudad ya elegida (`GET .../cities/{cityId}/postal-codes`). Sin query `?code=`, sin listado global de CP, sin autocompletado en esta entrega. Ver cuerpo § Aclaración punto 4."
  - id: C-05
    topic: Paginación y filtros
    resolution: "Sin paginación. Filtros únicamente por padre en la jerarquía: states por country, cities por state, postal codes por city."
  - id: C-06
    topic: Forma de los DTOs
    resolution: "DTOs acordes al dominio por entidad: Country (Id, Name, Code); State (Id, CountryId, Name, Code); City (Id, StateId, Name) — sin Code; PostalCode (Id, CityId, Code) — sin Name. Nombres de propiedades alineados al modelo (CountryId, StateId, CityId), no campo genérico ParentId."
  - id: C-07
    topic: Nombre del controlador y estilo de rutas
    resolution: 'Clase GeolocationController; ruta explícita [Route("api/geolocation")] sin api/[controller]. Segmento postal-codes en kebab-case.'
  - id: C-08
    topic: Visibilidad en lectura geo
    resolution: "Filtrar únicamente por `IsActive == true` y, según endpoint, por clave foránea del padre (`CountryId`, `StateId`, `CityId`). Sin predicado `DeletedAt`. Para no acoplar visibilidad al `HasQueryFilter` global de soft-delete, usar `IgnoreQueryFilters()` en todas las consultas de estos handlers geo y mantener solo `IsActive` + padre."
---

# Clarificación — Unificación geolocalización (lectura)

## Decisiones cerradas

Resumen acordado con el usuario (2026-03-20):

| # | Tema | Decisión |
|---|------|----------|
| 1 | Prefijo | `api/geolocation` |
| 2 | Legacy | Eliminar `CountriesController` y `StatesController` |
| 3 | Auth | Sí — misma política que hoy (`AuthorizeSystemOrAdmin`) |
| 4 | Postales | Solo por `cityId` |
| 5 | Listados | Sin paginación; solo filtros por padre |
| 6 | DTOs | Por entidad: Country/State con Code; City sin Code; PostalCode sin Name (ver SPEC) |
| 7 | Convenciones | `GeolocationController`, ruta base `api/geolocation`, `postal-codes` en kebab-case |
| 8 | Filtro lectura | Solo `IsActive` + padre jerárquico; sin `DeletedAt`; `IgnoreQueryFilters` en handlers geo (ver plan) |

### Aclaración punto 4 (códigos postales, C-04)

**Qué cubre:** Product (u otro cliente) debe obtener códigos postales **solo** después de seleccionar una **ciudad** conocida (`cityId`). La ruta acordada es:

`GET /api/geolocation/cities/{cityId}/postal-codes`

**Qué queda fuera de alcance en esta feature:**

- Resolver un CP **sin** conocer ciudad (p. ej. `GET /api/geolocation/postal-codes?code=28001`).
- Listar todos los códigos postales del sistema.
- Autocompletado o búsqueda parcial por texto sobre CP.

**Comportamiento esperado del endpoint:**

- Entrada: `cityId` en la ruta (padre jerárquico; mismo criterio que estados por país o ciudades por estado).
- Salida: colección de `PostalCodeGeoReadDto` (`Id`, `CityId`, `Code`), solo `IsActive == true`, orden sugerido por `Code` (spec).
- Si `cityId` no existe o no hay CP activos para esa ciudad, respuesta coherente con el resto de la API (típicamente lista vacía o 404 según convención que se unifique en implementación; dejar explícito en `implementation.md` si hace falta).

**Flujo típico en Product:** país → estados del país → ciudades del estado → **códigos postales de la ciudad**.

*Nota:* La **fase P4** del `plan.md` (tests DDD de **arquitectura**, no funcional) está aclarada en **`plan.md` → «Aclaración detallada — Fase P4»**; es independiente de C-04 (postales).

### Aclaración punto 7 (nombre y rutas)

En ASP.NET Core el **nombre de la clase** del controlador es `GeolocationController`; la **URL no depende** del sufijo `Controller` porque fijamos **`[Route("api/geolocation")]`** de forma explícita. Así Product usa siempre el mismo prefijo aunque en el futuro se renombre la clase. Los sub-recursos siguen el patrón REST ya propuesto (`countries`, `states`, `cities`, `postal-codes`).

---

## Preguntas originales (histórico)

Cerradas; ver frontmatter `decisions` y tabla anterior.

---

## Notas para el implementador

Siguiente paso: **`plan.md`** → **`implementation.md`** / ejecución. Incluir como **paso explícito del mismo proceso**: (1) DTOs por entidad (City sin `Code`, PostalCode sin `Name`); (2) handlers geo: `IgnoreQueryFilters()` + solo `IsActive` y FK de padre; (3) tests alineados a ese criterio (sin depender de `DeletedAt`) y DTOs §3 de la spec.
