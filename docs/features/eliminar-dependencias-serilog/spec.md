---
feature_name: eliminar-dependencias-serilog
created: "2026-03-16"
base: objectives.md
scope:
  in_scope:
    - Eliminar paquetes Serilog de GesFer.Admin.Back.Infrastructure.csproj
    - Reemplazar Serilog por Microsoft.Extensions.Logging
    - Crear provider/sink que escriba a ILogQueue (equivalente a MediatRLogSink)
    - Actualizar Program.cs y SerilogConfiguration (o reemplazar por configuración MEL)
    - Actualizar AdminWebAppFactory y E2EFixture (eliminar Serilog.Log.CloseAndFlushAsync)
    - Eliminar configuración Serilog de appsettings (UseMySql)
  out_scope:
    - Cambiar estructura de ILogQueue, LogQueue, Channel
    - Modificar tabla Logs o migraciones
    - Cambiar CreateLogCommand, CreateLogHandler, LogDispatcherBackgroundService
functional_requirements:
  - FR1: Los logs de la aplicación deben llegar a la cola ILogQueue
  - FR2: Los logs deben persistirse en BD vía CreateLogCommand (flujo existente)
  - FR3: En Development se debe mostrar logs en consola
  - FR4: Niveles mínimos configurables (Verbose en dev, Information en prod)
non_functional_requirements:
  - NFR1: Sin dependencias de Serilog ni paquetes relacionados
  - NFR2: Build y tests deben pasar
touchpoints:
  - path: src/GesFer.Admin.Back.Infrastructure/GesFer.Admin.Back.Infrastructure.csproj
    action: eliminar PackageReference Serilog
  - path: src/GesFer.Admin.Back.Infrastructure/Logging/MediatRLogSink.cs
    action: reemplazar por LogQueueLoggerProvider
  - path: src/GesFer.Admin.Back.Infrastructure/Logging/SerilogConfiguration.cs
    action: eliminar; reemplazar por AddLogging con provider custom
  - path: src/GesFer.Admin.Back.Api/Program.cs
    action: eliminar ConfigureInfrastructureLogging; usar AddLogging con provider
  - path: src/GesFer.Admin.Back.IntegrationTests/AdminWebAppFactory.cs
    action: eliminar Serilog.Log.CloseAndFlushAsync
  - path: src/GesFer.Admin.Back.E2ETests/E2EFixture.cs
    action: eliminar Serilog.Log.CloseAndFlushAsync
  - path: src/GesFer.Admin.Back.Api/appsettings.Development.json
    action: eliminar sección Serilog
validation_criteria:
  - Build exitoso
  - Unit tests pasan
  - Integration tests pasan
  - E2E tests pasan
  - Logs llegan a la tabla Logs en BD
---

# Especificación: Eliminar dependencias de Serilog

## 1. Análisis de dependencias Serilog

### 1.1 Paquetes en GesFer.Admin.Back.Infrastructure.csproj

| Paquete | Versión | Uso actual |
|---------|---------|------------|
| Serilog.AspNetCore | 8.0.0 | `UseSerilog`, integración con Host |
| Serilog.Sinks.Async | 2.0.0 | **No usado** en el código |
| Serilog.Sinks.Console | 5.0.1 | `WriteTo.Console()` en SerilogConfiguration |
| Serilog.Sinks.MySQL | 5.0.0 | **No usado** (logs van por cola + CreateLogCommand) |

**Transitivos:** Serilog, Serilog.Extensions.Hosting, Serilog.Extensions.Logging, Serilog.Formatting.Compact, Serilog.Settings.Configuration, Serilog.Sinks.Debug, Serilog.Sinks.File.

### 1.2 Archivos que usan Serilog

| Archivo | Uso |
|---------|-----|
| `MediatRLogSink.cs` | Implementa `ILogEventSink` (Serilog). Recibe `LogEvent`, convierte a `CreateLogDto`, escribe en `ILogQueue` con `TryWrite`. |
| `SerilogConfiguration.cs` | `ConfigureInfrastructureLogging`: usa `UseSerilog`, `ReadFrom.Services`, `Enrich.FromLogContext`, `WriteTo.Console`, `WriteTo.Sink(MediatRLogSink)`. |
| `Program.cs` | `builder.Host.ConfigureInfrastructureLogging()`. |
| `AdminWebAppFactory.cs` | `await Serilog.Log.CloseAndFlushAsync()` en `DisposeAsync`. |
| `E2EFixture.cs` | `await Serilog.Log.CloseAndFlushAsync()` en `DisposeAsync`. |

### 1.3 Flujo actual de logs

```
ILogger → Serilog (pipeline) → MediatRLogSink.Emit(LogEvent)
  → ILogQueue.TryWrite(CreateLogDto)
  → LogDispatcherBackgroundService.ReadAllAsync
  → CreateLogCommand → CreateLogHandler → BD (tabla Logs)
```

### 1.4 Flujo objetivo (sin Serilog)

```
ILogger → LogQueueLoggerProvider (ILoggerProvider)
  → LogQueueLogger.Log() → ILogQueue.TryWrite(CreateLogDto)
  → LogDispatcherBackgroundService.ReadAllAsync
  → CreateLogCommand → CreateLogHandler → BD (tabla Logs)
```

## 2. Estructura de datos a preservar

- **CreateLogDto:** `Level`, `Message`, `Exception`, `TimeStamp`, `Properties`
- **ILogQueue / LogQueue:** `TryWrite`, `WriteAsync`, `ReadAllAsync`
- **LogDispatcherBackgroundService:** Sin cambios
- **CreateLogCommand / CreateLogHandler:** Sin cambios
- **Tabla Logs:** Sin cambios

## 3. Implementación del reemplazo

### 3.1 LogQueueLoggerProvider

Crear `LogQueueLoggerProvider : ILoggerProvider` que:

- Recibe `ILogQueue` por constructor
- Crea `LogQueueLogger` con categoría
- Mapea `LogLevel` a string `Level` (Debug→Debug, Information→Information, etc.)
- Convierte `LogEntry` a `CreateLogDto` con `Properties` (scope, state, exception)
- Usa `TryWrite` para no bloquear (sincrónico, como MediatRLogSink)

### 3.2 LogQueueLogger

- Implementa `ILogger`
- En `Log<TState>`: si `IsEnabled(level)`, construir `CreateLogDto` y `TryWrite`
- `Properties`: extraer de `state` (IReadOnlyList<KeyValuePair<string, object>>) y `scope` si aplica

### 3.3 Registrar en DI

```csharp
// Program.cs
builder.Services.AddSingleton<ILogQueue, LogQueue>();
builder.Logging.AddProvider(new LogQueueLoggerProvider(logQueue));
builder.Logging.AddConsole(); // en Development
builder.Logging.SetMinimumLevel(isDevelopment ? LogLevel.Debug : LogLevel.Information);
builder.Logging.AddFilter("Microsoft", LogLevel.Warning);
builder.Logging.AddFilter("Microsoft.AspNetCore", LogLevel.Warning);
```

### 3.4 Eliminar

- `SerilogConfiguration.cs` (archivo completo)
- `MediatRLogSink.cs` (archivo completo)
- `builder.Host.ConfigureInfrastructureLogging()`
- `Serilog.Log.CloseAndFlushAsync()` en tests
- `"Serilog": { "UseMySql": false }` en appsettings.Development.json

## 4. Consideraciones

- **Orden de registro:** `ILogQueue` debe registrarse antes del provider (ya se hace en Program.cs).
- **Console:** `AddConsole()` ya está en el host por defecto; verificar que en Development se muestren logs.
- **Properties:** El formato de `Properties` en Serilog es `LogEvent.Properties` (ScalarValue, etc.). En MEL se usa `state` y `scope`; el formato puede ser un `Dictionary<string, object>` serializado a JSON como hace `CreateLogHandler`.

## 5. Criterios de validación

- [ ] `dotnet build` exitoso
- [ ] No hay referencias a `Serilog` en el código fuente
- [ ] Unit tests pasan
- [ ] Integration tests pasan
- [ ] E2E tests pasan
- [ ] Logs en consola en Development
- [ ] Logs persisten en tabla `Logs` vía cola + CreateLogCommand
