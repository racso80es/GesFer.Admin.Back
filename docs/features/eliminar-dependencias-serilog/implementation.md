---
feature_name: eliminar-dependencias-serilog
created: "2026-03-16"
items:
  - id: "1"
    action: create
    path: src/GesFer.Admin.Back.Infrastructure/Logging/LogQueueLogger.cs
    proposal: Implementar ILogger que convierta LogLevel/state/exception a CreateLogDto y llame ILogQueue.TryWrite
  - id: "2"
    action: create
    path: src/GesFer.Admin.Back.Infrastructure/Logging/LogQueueLoggerProvider.cs
    proposal: Implementar ILoggerProvider que cree LogQueueLogger con ILogQueue inyectado
  - id: "3"
    action: modify
    path: src/GesFer.Admin.Back.Api/Program.cs
    proposal: Reemplazar ConfigureInfrastructureLogging por AddProvider; crear LogQueue explícito para pasarlo al provider
  - id: "4"
    action: delete
    path: src/GesFer.Admin.Back.Infrastructure/Logging/MediatRLogSink.cs
  - id: "5"
    action: delete
    path: src/GesFer.Admin.Back.Infrastructure/Logging/SerilogConfiguration.cs
  - id: "6"
    action: modify
    path: src/GesFer.Admin.Back.Infrastructure/GesFer.Admin.Back.Infrastructure.csproj
    proposal: Eliminar PackageReference Serilog.AspNetCore, Serilog.Sinks.Async, Serilog.Sinks.Console, Serilog.Sinks.MySQL
  - id: "7"
    action: modify
    path: src/GesFer.Admin.Back.IntegrationTests/AdminWebAppFactory.cs
    proposal: Eliminar await Serilog.Log.CloseAndFlushAsync() en DisposeAsync
  - id: "8"
    action: modify
    path: src/GesFer.Admin.Back.E2ETests/E2EFixture.cs
    proposal: Eliminar await Serilog.Log.CloseAndFlushAsync() en DisposeAsync
  - id: "9"
    action: modify
    path: src/GesFer.Admin.Back.Api/appsettings.Development.json
    proposal: Eliminar "Serilog": { "UseMySql": false }
---

# Implementación: Eliminar dependencias Serilog

## Ítems de implementación

| Id | Acción | Ruta | Propuesta |
|----|--------|------|-----------|
| 1 | create | Logging/LogQueueLogger.cs | ILogger con Log<TState>, IsEnabled, BeginScope; convertir a CreateLogDto |
| 2 | create | Logging/LogQueueLoggerProvider.cs | ILoggerProvider que crea LogQueueLogger |
| 3 | modify | Program.cs | logQueue explícito, AddProvider, filtros de nivel |
| 4 | delete | MediatRLogSink.cs | Eliminar |
| 5 | delete | SerilogConfiguration.cs | Eliminar |
| 6 | modify | Infrastructure.csproj | Eliminar 4 PackageReference Serilog |
| 7 | modify | AdminWebAppFactory.cs | Eliminar CloseAndFlushAsync |
| 8 | modify | E2EFixture.cs | Eliminar CloseAndFlushAsync |
| 9 | modify | appsettings.Development.json | Eliminar sección Serilog |
