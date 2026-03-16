---
feature_name: eliminar-dependencias-serilog
created: "2026-03-16"
phases:
  - id: "1"
    name: Crear LogQueueLoggerProvider y LogQueueLogger
  - id: "2"
    name: Actualizar Program.cs (reemplazar Serilog por MEL)
  - id: "3"
    name: Eliminar archivos y paquetes Serilog
  - id: "4"
    name: Actualizar tests y appsettings
  - id: "5"
    name: Validar build y tests
tasks:
  - Crear LogQueueLogger.cs
  - Crear LogQueueLoggerProvider.cs
  - Actualizar Program.cs
  - Eliminar MediatRLogSink.cs
  - Eliminar SerilogConfiguration.cs
  - Eliminar paquetes Serilog del csproj
  - Actualizar AdminWebAppFactory.cs
  - Actualizar E2EFixture.cs
  - Actualizar appsettings.Development.json
  - Ejecutar dotnet build
  - Ejecutar tests
---

# Plan: Eliminar dependencias Serilog

## Fase 1: Crear LogQueueLoggerProvider y LogQueueLogger

- Crear `LogQueueLogger` que implemente `ILogger`
- Crear `LogQueueLoggerProvider` que implemente `ILoggerProvider`
- Mapear `LogLevel` → string, `state` → Properties (Dictionary)

## Fase 2: Actualizar Program.cs

- Reemplazar `ConfigureInfrastructureLogging()` por `AddProvider(LogQueueLoggerProvider)`
- Configurar niveles mínimos y filtros (Microsoft, Microsoft.AspNetCore)
- Mantener `ILogQueue` como singleton (crear instancia explícita para pasarla al provider)

## Fase 3: Eliminar Serilog

- Eliminar `MediatRLogSink.cs`
- Eliminar `SerilogConfiguration.cs`
- Eliminar PackageReference de Serilog.* en Infrastructure.csproj

## Fase 4: Actualizar tests y configuración

- Eliminar `Serilog.Log.CloseAndFlushAsync()` en AdminWebAppFactory y E2EFixture
- Eliminar sección Serilog de appsettings.Development.json

## Fase 5: Validar

- `dotnet build`
- Unit tests, Integration tests, E2E tests
