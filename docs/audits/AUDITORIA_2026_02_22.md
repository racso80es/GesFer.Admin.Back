# Auditoría de Infraestructura Backend - 2026-02-22

## 1. Métricas de Salud (0-100%)
- **Arquitectura**: 85%
  - Estructura Clean Architecture respetada en general.
  - Violación de Dependency Inversion en `GesFer.Admin.Back.Api`: `DependencyInjection.cs` referencia directamente implementaciones de `Infrastructure` (`AdminAuthService`, etc.).
  - Lógica de negocio en `LogController` (PurgeLogs) en lugar de usar MediatR/Services.
- **Nomenclatura**: 95%
  - Convenciones PascalCase seguidas correctamente.
  - Warnings de compilación (CS8601) en `LogController`.
- **Estabilidad Async**: 90%
  - Uso correcto de async/await en general.
  - **Punto Crítico**: `PurgeLogs` carga entidades en memoria antes de eliminar, lo que es ineficiente y peligroso para la estabilidad bajo carga.

## 2. Pain Points (🔴 Críticos / 🟡 Medios)

### 🔴 Crítico: Credenciales Hardcodeadas (Fallo de Seguridad)
- **Hallazgo**: Se encontró una cadena de conexión con contraseña en texto plano como fallback en `DependencyInjection.cs`.
- **Ubicación**: `src/GesFer.Admin.Back.Api/DependencyInjection.cs` Línea ~19.

### 🔴 Crítico: Ineficiencia en PurgeLogs (Rendimiento)
- **Hallazgo**: El método `PurgeLogs` recupera todos los registros a eliminar a la memoria (`ToListAsync`) antes de eliminarlos uno a uno (o en rango), lo cual puede causar OutOfMemoryException con grandes volúmenes de datos.
- **Ubicación**: `src/GesFer.Admin.Back.Api/Controllers/LogController.cs` Línea ~198.

### 🟡 Medio: Violación de Capas (Arquitectura)
- **Hallazgo**: El proyecto `Api` referencia y registra servicios concretos de `Infrastructure` directamente.
- **Ubicación**: `src/GesFer.Admin.Back.Api/DependencyInjection.cs`.

### 🟡 Medio: Warnings de Nulabilidad
- **Hallazgo**: Múltiples warnings CS8601 en `LogController` indican posibles asignaciones nulas no controladas.
- **Ubicación**: `src/GesFer.Admin.Back.Api/Controllers/LogController.cs` Líneas 89-93.

## 3. Acciones Kaizen (Hoja de Ruta para el Executor)

### Acción 1: Eliminar Credenciales Hardcodeadas
**Instrucción**:
Modificar `src/GesFer.Admin.Back.Api/DependencyInjection.cs` para eliminar el string de conexión de fallback y lanzar una excepción si no se encuentra en la configuración.

```csharp
// ANTES
var connectionString = configuration.GetConnectionString("DefaultConnection")
    ?? "Server=localhost;...;Password=scrappassword;...";

// DESPUÉS
var connectionString = configuration.GetConnectionString("DefaultConnection")
    ?? throw new InvalidOperationException("Connection string 'DefaultConnection' not found.");
```

**Definition of Done (DoD)**:
- El código no contiene credenciales en texto plano.
- La aplicación falla al iniciar si no hay connection string configurada (Fail Fast).

### Acción 2: Optimizar PurgeLogs con ExecuteDeleteAsync
**Instrucción**:
Refactorizar el método `PurgeLogs` en `src/GesFer.Admin.Back.Api/Controllers/LogController.cs` para usar `ExecuteDeleteAsync` (disponible en EF Core 7+).

```csharp
// ANTES
var logsToDelete = await _context.Logs
    .Where(l => l.TimeStamp < dateLimit)
    .ToListAsync();

var count = logsToDelete.Count;

if (count > 0)
{
    _context.Logs.RemoveRange(logsToDelete);
    await _context.SaveChangesAsync();
}

// DESPUÉS
var count = await _context.Logs
    .Where(l => l.TimeStamp < dateLimit)
    .ExecuteDeleteAsync();
```

**Definition of Done (DoD)**:
- No se cargan entidades en memoria para la eliminación.
- Se utiliza `ExecuteDeleteAsync`.
- El rendimiento es O(1) en términos de memoria de aplicación.

### Acción 3: Corregir Warnings de Nulabilidad
**Instrucción**:
Asegurar que las propiedades en `LogController` manejan correctamente los nulos o suprimir los warnings si se garantiza que no son nulos por lógica de negocio (aunque es mejor manejarlo).

```csharp
// En LogController.cs
// Asegurar que las propiedades de LogDto (si son non-nullable) reciban valores no nulos.
// O cambiar LogDto para permitir nulos si la entidad lo permite.
```

**Definition of Done (DoD)**:
- El proyecto compila sin warnings (0 Warnings).
