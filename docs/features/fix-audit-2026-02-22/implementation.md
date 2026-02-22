# Archivos de Implementación: Correcciones Auditoría 2026-02-22

## Archivos a Modificar

1. **`src/GesFer.Admin.Back.Api/DependencyInjection.cs`**
   - **Acción**: Eliminar string de conexión de fallback con credenciales.

2. **`src/GesFer.Admin.Back.Api/Controllers/LogController.cs`**
   - **Acción**: Optimizar `PurgeLogs` (usar `ExecuteDeleteAsync`).
   - **Acción**: Corregir warnings de nulabilidad (CS8601) en `GetLogs`.

## Archivos a Consultar (Solo Lectura)

- `src/GesFer.Admin.Back.Application/DTOs/Logs/LogDto.cs`
- `src/GesFer.Admin.Back.Domain/Entities/Log.cs`
- `src/GesFer.Admin.Back.Infrastructure/Data/AdminDbContext.cs`

## Nuevos Archivos (Documentación SddIA)

- `docs/features/fix-audit-2026-02-22/objectives.md`
- `docs/features/fix-audit-2026-02-22/spec.md`
- `docs/features/fix-audit-2026-02-22/clarify.md`
- `docs/features/fix-audit-2026-02-22/plan.md`
- `docs/features/fix-audit-2026-02-22/implementation.md`
- `docs/features/fix-audit-2026-02-22/execution.json`
- `docs/features/fix-audit-2026-02-22/validacion.json`
