# Plan de Implementación

1.  **Initialize Feature Documentation** (Completado)
    *   Crear `docs/features/geo-location-context/` con `objectives.md`, `spec.md`, `clarify.md`.

2.  **Implement Rust Skills Runner**
    *   Crear proyecto `scripts/tools-rs/sddia-skills`.
    *   Implementar CLI básico.
    *   Actualizar `SddIA/skills/skills-contract.json`.

3.  **Implement Geographic Domain & Persistence**
    *   Crear `src/GesFer.Admin.Back.Infrastructure/Data/Seeds/countries.json` (con estados y ciudades anidados o planos, según diseño de entidad). *Nota: EF Core suele preferir grafos o claves foráneas. JSON plano con IDs es más fácil de mantener.*
    *   Implementar Seeder en `Infrastructure`.

4.  **Implement Application Layer (CQRS)**
    *   DTOs (`CountryDto`, `StateDto`, `CityDto`).
    *   Queries (`GetAllCountries`, `GetCountryById`, `GetStatesByCountry`, `GetCitiesByState`).
    *   Mappings (AutoMapper).

5.  **Implement API Layer**
    *   Controladores: `CountriesController`, `StatesController`, `CitiesController` (o unificado).
    *   Seguridad `[AuthorizeSystemOrAdmin]`.

6.  **Testing & Verification**
    *   Unit Tests (Application).
    *   Integration Tests (Api).

7.  **Finalize**
    *   Pre-commit checks.
    *   Submit.
