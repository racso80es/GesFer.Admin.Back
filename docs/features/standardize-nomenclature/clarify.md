# SPEC-standardize-nomenclature_CLARIFICATIONS.md

**Fecha:** 2026-05-22
**Estado:** Clarificado
**Contexto:** `docs/features/standardize-nomenclature/`

## Clarificaciones Generales
- **Nombre Raíz:** `GesFer.Admin.Back` se ha confirmado como el nombre estándar para todos los proyectos, directorios y namespaces.
- **Herramienta Faltante:** La herramienta `GesFer.Console` no está presente en el código fuente. Se ha creado la deuda técnica `DT-2025-001` y se está realizando la documentación manualmente.
- **Carpeta 'Infra':** Se ha clarificado que la carpeta `Infrastructure` se debe renombrar a `GesFer.Admin.Back.Infrastructure`, manteniendo la consistencia con el nombre del proyecto.
- **Docker:** El archivo `Dockerfile` en `src/` debe ser actualizado para reflejar la nueva ruta del proyecto `GesFer.Admin.Back.Api` y el cambio de nombre del archivo `.csproj`.

## Preguntas y Respuestas
1.  **P:** ¿Se debe modificar el nombre de la solución `GesFer.Admin.Back.sln`?
    **R:** No, el nombre actual ya es correcto. Solo se deben actualizar las referencias internas a los proyectos renombrados.

2.  **P:** ¿Se debe modificar `docker-compose.yml`?
    **R:** Sí, para apuntar al nuevo `Dockerfile` (si cambia de ubicación o nombre interno) y para asegurar que los nombres de los servicios sean coherentes con la nueva nomenclatura.

## Gaps Identificados
- **Pruebas de Integración:** Las pruebas de integración (`GesFer.Product.IntegrationTests`) parecen estar obsoletas o fuera de lugar. Se renombrarán a `GesFer.Admin.Back.IntegrationTests` asumiendo que su contenido es relevante para `Admin`. (Verificar contenido real durante la implementación).
- **Scripts:** Los scripts `docker-start.ps1`, `docker-stop.ps1`, `reset-db.ps1` pueden tener referencias codificadas a los nombres antiguos de contenedores o servicios. Se deben revisar y actualizar.

## Decisiones
- Se renombrarán todas las carpetas en `src/` para coincidir con el nombre completo del namespace (e.g., `src/GesFer.Admin.Back.Api/`).
- Se mantendrá la estructura Clean Architecture existente (`Api`, `Application`, `Domain`, `Infrastructure`).
