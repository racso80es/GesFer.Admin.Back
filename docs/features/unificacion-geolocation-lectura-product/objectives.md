---
feature_name: unificacion-geolocation-lectura-product
created: 2026-03-20
process: feature
---

# Objetivos — Unificación API de geolocalización (lectura)

## Objetivo principal

Exponer en **un único controlador** los endpoints de **solo lectura** para **Country, State, City y PostalCode**, de forma coherente y consumible por la **API de Product**, sustituyendo o consolidando el diseño actual (disperso entre `CountriesController` y `StatesController`, sin lectura pública de códigos postales).

## Alcance previsto (alto nivel)

- **Incluido:** rutas bajo `api/geolocation`, DTOs por entidad (City sin `Code`, PostalCode sin `Name`), handlers con criterio **solo `IsActive`** (sin predicado `DeletedAt`; ver spec §4 y decisión `IgnoreQueryFilters`), eliminación de controladores legacy, tests y OpenAPI.

## Ley / normas aplicadas

- Proceso **feature** (`paths.processPath/feature`).
- Clean Architecture + MediatR ya usados en el proyecto.
- Autorización: misma línea que el resto de catálogos expuestos a sistema/admin (`AuthorizeSystemOrAdmin`), salvo que **clarify** cierre otra política para Product.

## Relación con documentación previa

- Existe `paths.featurePath/geo-location-context` (contexto geo + otras metas). Esta tarea se centra en **unificación del controlador y postal codes para consumo Product**; no sustituye automáticamente aquel documento, pero puede referenciar entidades y seeds ya descritos allí.

## Siguiente fase

- **plan.md:** roadmap (incl. paso dedicado a filtrado IsActive / EF query filters).
- **implementation.md** y ejecución en código cuando corresponda.
