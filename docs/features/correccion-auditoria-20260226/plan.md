# Plan de Ejecución: Corrección Auditoría 2026-02-26

1.  **Configuración y Documentación** (Completado)
    - Crear rama y estructura de documentación.
    - Definir objetivos y especificaciones.

2.  **Implementación KAIZEN-1 (Value Objects)**
    - Implementar `ProcessId` en `src/GesFer.Admin.Back.Domain/ValueObjects/ProcessId.cs`.
    - Implementar `ActionStatus` en `src/GesFer.Admin.Back.Domain/ValueObjects/ActionStatus.cs`.
    - Verificar compilación.

3.  **Implementación KAIZEN-2 (Fix Seeds)**
    - Modificar `src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs` para corregir la ruta de semillas.
    - Ejecutar tests de integración: `dotnet test src/GesFer.Admin.Back.IntegrationTests/`.
    - Verificar que `GeoControllerTests` pase.

4.  **Implementación KAIZEN-3 (Migración Rust)**
    - Analizar scripts existentes en `scripts/`.
    - Redactar `docs/analysis/MIGRACION_RUST_SCRIPTS.md`.

5.  **Verificación y Cierre**
    - Ejecutar suite completa de tests.
    - Generar `implementation.md` y `validacion.json`.
    - Realizar commit y push.
