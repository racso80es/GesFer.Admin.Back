# Auditoría S+ - 2026-02-21

## 1. Métricas de Salud (0-100%)
*   **Arquitectura: 50%** (La estructura base es correcta, pero el proyecto no compila debido a archivos faltantes críticos en la capa de Application. Esto impide cualquier despliegue o prueba real.)
*   **Nomenclatura: 90%** (El uso de `GesFer.Admin.Back.*` es consistente. Los nombres de carpetas y espacios de nombres están alineados.)
*   **Estabilidad Async: 100%** (No se detectaron llamadas bloqueantes como `.Result` o `.Wait()` en el código asíncrono revisado.)

## 2. Pain Points (🔴 Críticos / 🟡 Medios)

### 🔴 Crítico: El proyecto no compila
**Hallazgo:** Faltan los DTOs de Logs en `GesFer.Admin.Back.Application`. El controlador `LogController` hace referencia a un espacio de nombres y clases que no existen.
**Ubicación:** `src/GesFer.Admin.Back.Api/Controllers/LogController.cs` (Líneas 3, 36, 80, 112, 186)
**Impacto:** Bloqueo total del pipeline de CI/CD.

### 🔴 Crítico: Credenciales Hardcoded
**Hallazgo:** La cadena de conexión a base de datos contiene credenciales en texto plano como fallback en `Program.cs`.
**Ubicación:** `src/GesFer.Admin.Back.Api/Program.cs` (Línea 29)
**Impacto:** Riesgo de seguridad alto. Exposición de credenciales.

### 🟡 Medio: Versiones de Dependencias
**Hallazgo:** `GesFer.Admin.Back.Infrastructure` utiliza `Microsoft.EntityFrameworkCore` 8.0.0, mientras que `Api` y `Application` estandarizan `MediatR` en 12.4.1. Aunque compatible, se recomienda mantener la coherencia en las versiones de EF Core y asegurar que no haya conflictos transitivos.
**Ubicación:** `src/GesFer.Admin.Back.Infrastructure/GesFer.Admin.Back.Infrastructure.csproj`

## 3. Acciones Kaizen (Hoja de Ruta para el Executor)

### Acción 1: Restaurar DTOs de Logs
**Prioridad:** Alta (Bloqueante)
**DoD:** El proyecto debe compilar sin errores `dotnet build`.

**Instrucciones:**
Crear la carpeta `src/GesFer.Admin.Back.Application/DTOs/Logs/` y añadir los siguientes archivos:

**File: `CreateLogDto.cs`**
```csharp
namespace GesFer.Admin.Back.Application.DTOs.Logs;

public class CreateLogDto
{
    public string Level { get; set; } = string.Empty;
    public string Message { get; set; } = string.Empty;
    public string? Exception { get; set; }
    public DateTime TimeStamp { get; set; }
    public Dictionary<string, object>? Properties { get; set; }
}
```

**File: `CreateAuditLogDto.cs`**
```csharp
namespace GesFer.Admin.Back.Application.DTOs.Logs;

public class CreateAuditLogDto
{
    public string? CursorId { get; set; }
    public string? Username { get; set; }
    public string? Action { get; set; }
    public string? HttpMethod { get; set; }
    public string? Path { get; set; }
    public string? AdditionalData { get; set; }
    public DateTime ActionTimestamp { get; set; }
}
```

**File: `LogDto.cs`**
```csharp
namespace GesFer.Admin.Back.Application.DTOs.Logs;

public class LogDto
{
    public int Id { get; set; }
    public string Level { get; set; } = string.Empty;
    public string Message { get; set; } = string.Empty;
    public string? Exception { get; set; }
    public DateTime TimeStamp { get; set; }
    public string? Source { get; set; }
    public Guid? CompanyId { get; set; }
    public Guid? UserId { get; set; }
}
```

**File: `LogsPagedResponseDto.cs`**
```csharp
namespace GesFer.Admin.Back.Application.DTOs.Logs;

public class LogsPagedResponseDto
{
    public List<LogDto> Logs { get; set; } = new();
    public int TotalCount { get; set; }
    public int PageNumber { get; set; }
    public int PageSize { get; set; }
    public int TotalPages { get; set; }
}
```

**File: `PurgeLogsResponseDto.cs`**
```csharp
namespace GesFer.Admin.Back.Application.DTOs.Logs;

public class PurgeLogsResponseDto
{
    public int DeletedCount { get; set; }
    public DateTime DateLimit { get; set; }
}
```

### Acción 2: Eliminar Credenciales Hardcoded
**Prioridad:** Alta (Seguridad)
**DoD:** No deben existir contraseñas en `Program.cs`.

**Instrucciones:**
Modificar `src/GesFer.Admin.Back.Api/Program.cs` para eliminar el string de fallback.

```csharp
// Antes
var connectionString = builder.Configuration.GetConnectionString("DefaultConnection")
    ?? "Server=localhost;Port=3306;Database=GesFer_Admin;User=admin;Password=GesFerAdmin@pthrjkl;CharSet=utf8mb4;AllowUserVariables=True;AllowLoadLocalInfile=True;";

// Después
var connectionString = builder.Configuration.GetConnectionString("DefaultConnection")
    ?? throw new InvalidOperationException("Connection string 'DefaultConnection' not found.");
```
