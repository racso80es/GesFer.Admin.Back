# Especificación Técnica: Geo-Location Context & Rust Skills Runner

## Contexto
Actualmente, la gestión de zonas geográficas (País -> Provincia -> Ciudad) no está centralizada en `GesFer.Admin.Back`. Se requiere migrar esta responsabilidad, asegurando que los datos maestros se carguen vía Seeds y se expongan mediante una API segura. Adicionalmente, se debe estandarizar la ejecución de skills mediante una herramienta en Rust.

## Requerimientos Funcionales
1.  **API de Geografía:**
    *   Endpoint `GET /api/countries`: Listar todos los países.
    *   Endpoint `GET /api/countries/{id}`: Obtener detalle de un país.
    *   Endpoint `GET /api/states/by-country/{countryId}`: Listar provincias de un país.
    *   Endpoint `GET /api/cities/by-state/{stateId}`: Listar ciudades de una provincia.
    *   Todos los endpoints deben retornar DTOs (`CountryDto`, `StateDto`, `CityDto`).
    *   Seguridad: `[AuthorizeSystemOrAdmin]`.

2.  **Persistencia y Datos:**
    *   Entidades: `Country`, `State`, `City` (ya existentes en Domain).
    *   Seeds: Archivos JSON en `Infrastructure/Data/Seeds/` (`countries.json`, etc.).
    *   Carga: Al inicio de la aplicación (`AdminDbContextInitialiser`).

3.  **Rust Skills Runner:**
    *   Crear proyecto Rust en `scripts/tools-rs/sddia-skills`.
    *   Compilar un ejecutable capaz de ser invocado.
    *   Actualizar documentación de `SddIA/skills/` para reflejar que la implementación debe ser en Rust.

## Requerimientos No Funcionales
*   **Performance:** Uso de `AsNoTracking()` para consultas de lectura.
*   **Seguridad:** Validación de JWT vía `AuthorizeSystemOrAdmin`.
*   **Arquitectura:** Clean Architecture + CQRS (MediatR).
*   **Testing:** Unit Tests para Handlers, Integration Tests para Controllers.

## Entregables
*   Código fuente C# (`Api`, `Application`, `Infrastructure`).
*   Código fuente Rust (`scripts/tools-rs`).
*   Tests unitarios e integración.
*   Documentación actualizada.
