---
type: objectives
feature_name: correccion-2026-06-21
priority: high
status: pending
---
# Objetivos: Corrección de Auditoría 2026-06-21

## Hallazgos Priorizados
1. [Crítico] Posible bloqueo de hilos (thread pool starvation) por falta de propagación del CancellationToken en operaciones asíncronas de Entity Framework Core.
2. [Medio] Fugas de memoria potenciales por falta de `.AsNoTracking()` en consultas de sólo lectura.

## Alcance
- Modificar `AdminJsonDataSeeder` y `AdminAuthService` para incluir `.AsNoTracking()` donde sea necesario (operaciones de solo lectura).
- Propagar `CancellationToken` en operaciones asíncronas (como `FirstOrDefaultAsync()`, `ToListAsync()`, etc.) en `AdminAuthService` y métodos de semilla de `AdminJsonDataSeeder`.

## Criterios de Cierre (DoD)
- Código modificado con las mitigaciones aplicadas en los dos archivos detectados.
- El proyecto compila correctamente.
- Los tests pasan sin regresiones.
