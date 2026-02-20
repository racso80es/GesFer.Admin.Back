# Objetivo: Gestión de Zonas Geográficas (Geo-Location Context) y Runner de Skills en Rust

## Objetivo Principal
Reproducir de la solución GesFer lo relacionado con la gestión de zonas geográficas (seeds, EF, etc.) para que sea **GesFer.Admin.Back** el responsable de ese contexto, ofreciendo endpoints de consumo con la correspondiente seguridad (`[AuthorizeSystemOrAdmin]`, igual que la petición de `Company`).

## Objetivos Secundarios
1.  **Skills Runner en Rust:** Crear una herramienta en Rust (`scripts/tools-rs`) que actúe como ejecutable para las skills definidas en `SddIA/skills/`, desacoplando el dominio de la implementación técnica.
2.  **Seed Data:** Implementar la carga inicial de datos geográficos (Países, Provincias, Ciudades) mediante archivos JSON seed.
3.  **Seguridad:** Asegurar que los endpoints de consumo estén protegidos con la misma política que `CompanyController`.

## Alcance
*   **Backend:** `GesFer.Admin.Back` (Domain, Infrastructure, Application, Api).
*   **Tools:** Nuevo proyecto Rust en `scripts/tools-rs/`.
*   **Documentación:** `SddIA/skills/` (actualización de contrato/readme).

## Ley Aplicada
*   **SddIA/Process/feature.md**: Ciclo completo de feature.
*   **Clean Architecture**: Segregación de capas (Domain, Infra, App, Api).
*   **CQRS**: Uso de MediatR para las queries de lectura.

## Resumen del Proceso
1.  **Fase 0:** Preparar entorno (Rama `feat/geo-location-context`).
2.  **Fase 1:** Documentación con objetivos (este archivo).
3.  **Fase 2-4:** Spec, Clarify, Plan.
4.  **Fase 5:** Implementación (Código Rust y C#).
5.  **Fase 6-7:** Ejecución y Validación.
6.  **Fase 8:** Finalizar y PR.

## Referencias
*   `SddIA/process/feature.md`
*   `SddIA/skills/skills-contract.json`
*   `src/GesFer.Admin.Back.Api/Controllers/CompanyController.cs`
