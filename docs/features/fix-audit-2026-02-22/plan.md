# Plan de Ejecución: Correcciones Auditoría 2026-02-22

## 1. Documentación SddIA (Actual)
- Crear estructura en `docs/features/fix-audit-2026-02-22/`.
- Definir Objetivos, Especificación, Clarificaciones y Plan.
- Listar archivos de Implementación (`implementation.md`).

## 2. Implementación de Acciones Kaizen

### Acción 1: Eliminar Credenciales Hardcodeadas (Seguridad)
- **Archivo**: `src/GesFer.Admin.Back.Api/DependencyInjection.cs`
- **Tarea**: Reemplazar string de conexión fallback con `throw new InvalidOperationException()`.
- **Verificación**: `dotnet build` - Debe compilar correctamente.
- **Commit**: `fix(security): remove hardcoded connection string fallback`.

### Acción 2: Optimizar PurgeLogs (Rendimiento)
- **Archivo**: `src/GesFer.Admin.Back.Api/Controllers/LogController.cs`
- **Tarea**: Refactorizar método `PurgeLogs`.
- **Detalle**: Usar `_context.Logs.Where(...).ExecuteDeleteAsync()`.
- **Verificación**: `dotnet build` - Debe compilar correctamente.
- **Commit**: `perf(logs): optimize PurgeLogs using ExecuteDeleteAsync`.

### Acción 3: Corregir Warnings de Nulabilidad (Estabilidad)
- **Archivo**: `src/GesFer.Admin.Back.Api/Controllers/LogController.cs`
- **Tarea**: Resolver warnings CS8601 en `GetLogs` (asignaciones a `LogDto`).
- **Detalle**: Usar operador coalescente `?? string.Empty` para propiedades obligatorias.
- **Verificación**: `dotnet build` - Debe reportar 0 warnings en `LogController`.
- **Commit**: `fix(logs): resolve nullability warnings in LogController`.

## 3. Finalización SddIA
- Generar `execution.json`.
- Generar `validacion.json`.
- Verificar solución completa.

## 4. Entrega
- Realizar Pre-Commit Checks.
- Submit a la rama `fix/audit-corrections-2026-02-22`.
