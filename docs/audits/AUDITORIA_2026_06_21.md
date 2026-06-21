# Auditoría S+ - Guardián de la Infraestructura
**Fecha:** 2026-06-21

## 1. Métricas de Salud (0-100%)
*   **Arquitectura:** 90%
*   **Nomenclatura:** 95%
*   **Estabilidad Async:** 85%

## 2. Pain Points (🔴 Críticos / 🟡 Medios)

*   **Hallazgo:** [🟡 Medio] Fugas de memoria potenciales por falta de `.AsNoTracking()` en consultas de sólo lectura.
    *   **Ubicación:** `src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs` (múltiples líneas como 204, 208, 371, 419, etc.)
    *   **Ubicación:** `src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs` (línea 37)
*   **Hallazgo:** [🔴 Crítico] Posible bloqueo de hilos (thread pool starvation) por falta de propagación del `CancellationToken` en operaciones asíncronas de Entity Framework Core.
    *   **Ubicación:** `src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs` (múltiples llamadas a `ToListAsync()`)
    *   **Ubicación:** `src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs` (línea 37 - `FirstOrDefaultAsync()`)

## 3. Acciones Kaizen (Hoja de Ruta para el Executor)

### Acción 1: Aplicar `.AsNoTracking()` en operaciones de sólo lectura
*   **Instrucciones:** Modificar los métodos en `AdminJsonDataSeeder` y `AdminAuthService` para incluir `.AsNoTracking()` cuando se recuperan entidades para sólo lectura. En el caso del Seeder, si se van a modificar entidades existentes, NO aplicar `AsNoTracking`, pero para `Select(x => x.Id)` u otras consultas donde no se rastrean cambios, sí es necesario. Dado que `AdminJsonDataSeeder` sólo lee para verificar existencia, debería usar `AsNoTracking()` para esas comprobaciones. Para `AdminAuthService`, es una consulta pura de lectura para validación, por lo que es mandatorio.
*   **DoD:** Las consultas LINQ en `AdminJsonDataSeeder.cs` y `AdminAuthService.cs` incluyen `AsNoTracking()` cuando corresponde. El proyecto compila y los tests pasan.

### Acción 2: Propagar `CancellationToken`
*   **Instrucciones:** Modificar la interfaz `IAdminAuthService` y su implementación en `AdminAuthService.cs` para aceptar y propagar un `CancellationToken`. De igual forma, en `AdminJsonDataSeeder`, aceptar `CancellationToken` (si es posible inyectarlo/pasarlo, aunque al ser un Seeder puede ser menos crítico, pero `AdminAuthService` es usado en request-response y DEBE tenerlo).
*   **DoD:** Los métodos asíncronos en `AdminAuthService.cs` (y su interfaz) y donde sea pertinente reciben y pasan el `CancellationToken` a métodos como `FirstOrDefaultAsync()`.
