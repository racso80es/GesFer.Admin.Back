---
type: objectives
feature_name: correccion-2026-06-15
status: in_progress
---

# Objetivos: Correcciones Auditoría 2026-06-15

## Hallazgos Priorizados
1. **Crítico:** Propagación de `CancellationToken` en métodos asíncronos de EF Core en `AdminJsonDataSeeder.cs`, `AdminAuthService.cs` y tests, para prevenir bloqueos de threads.
2. **Medio:** Uso de `.AsNoTracking()` en las consultas de lectura en `AdminAuthService.cs` para evitar pérdida de rendimiento en EF Core.

## Criterios de Cierre (DoD)
- `AdminAuthService.cs` utiliza `.AsNoTracking()` proactivamente en sus queries.
- Métodos como `AuthenticateAdminAsync` en `AdminAuthService.cs` y métodos asíncronos en `AdminJsonDataSeeder.cs` (y tests) implementan explícitamente `CancellationToken`.
- El proyecto compila y los tests pasan sin errores (`dotnet test src/GesFer.Admin.Back.sln`).
- Documentación de tarea (`spec.md`, `clarify.md`, `validacion.md`, `finalize-process.md`) generada.
