# Especificación Técnica: Correcciones Auditoría 2026-02-22

## Resumen
Implementar las tres correcciones Kaizen identificadas en la auditoría del 2026-02-22: eliminación de credenciales hardcodeadas, optimización de `PurgeLogs` con `ExecuteDeleteAsync`, y resolución de warnings de nulabilidad en `LogController`.

## Modificaciones

### 1. `src/GesFer.Admin.Back.Api/DependencyInjection.cs`
- **Cambio**: Eliminar la cadena de conexión de fallback con contraseña en texto plano.
- **Implementación**: Usar `configuration.GetConnectionString("DefaultConnection") ?? throw new InvalidOperationException(...)`.
- **Justificación**: Seguridad (No exponer credenciales en el código fuente).

### 2. `src/GesFer.Admin.Back.Api/Controllers/LogController.cs` - `PurgeLogs`
- **Cambio**: Reemplazar la recuperación de logs (`ToListAsync`) y eliminación (`RemoveRange`) por una operación directa en base de datos.
- **Implementación**: Usar `ExecuteDeleteAsync()` de EF Core 7+.
- **Justificación**: Rendimiento y escalabilidad. Evitar cargar miles de registros en memoria antes de eliminarlos.
- **Impacto**: Reducción drástica del uso de memoria durante operaciones de purga masiva.

### 3. `src/GesFer.Admin.Back.Api/Controllers/LogController.cs` - Nulabilidad
- **Cambio**: Corregir warnings CS8601 (Possible null reference assignment) en las asignaciones de propiedades de `LogDto`.
- **Implementación**: Asegurar que las propiedades de `LogDto` (Level, Message, Exception) no se asignen con valores nulos, o modificar `LogDto` para permitir nulos si es apropiado (en este caso, `Level` y `Message` son obligatorios, así que debemos asegurar que no son nulos).
- **Estrategia**: Usar operador coalescente nulo (`?? string.Empty`) o validar antes de asignar. En el caso de `Exception`, puede ser nulo, así que `LogDto.Exception` debería permitir null (ya lo hace: `string?`).
- **Justificación**: Estabilidad del código y cumplimiento de la convención de 0 warnings.

## Pruebas
- Verificar compilación exitosa (`dotnet build`).
- Verificar ausencia de warnings en `LogController`.
- Probar localmente que `PurgeLogs` (si es posible testear) funciona correctamente, aunque la validación principal será la compilación y revisión de código.

## Riesgos
- **PurgeLogs**: Asegurar que la query LINQ sea compatible con `ExecuteDeleteAsync`. `Where(l => l.TimeStamp < dateLimit)` es compatible.
- **DependencyInjection**: Si no existe la configuración, la aplicación fallará al inicio (comportamiento deseado: Fail Fast).
