---
feature_name: eliminar-dependencias-serilog
created: "2026-03-16"
process: feature
---

# Objetivos: Eliminar dependencias de Serilog

## Objetivo principal

Disponer de un sistema de logs **sin depender de terceros** (Serilog). La estructura actual de colas (`ILogQueue`, `LogQueue`, `Channel<CreateLogDto>`) y la tabla `Logs` en base de datos son válidas y se mantienen.

## Alcance

- **In scope:** Eliminar todas las dependencias de Serilog del proyecto; reemplazar por el sistema de logging estándar de .NET (`Microsoft.Extensions.Logging`).
- **Out of scope:** Cambiar la estructura de colas, la tabla `Logs`, el flujo MediatR (`CreateLogCommand`), ni el `LogDispatcherBackgroundService`.

## Ley aplicada

- **Ley COMPILACIÓN:** El código roto es inaceptable. Verificar localmente.
- **Ley SOBERANÍA:** docs/ y SddIA/ son la verdad absoluta.

## Resumen del proceso

1. Analizar dependencias Serilog en el codebase.
2. Definir reemplazo: `ILogger` + provider personalizado que escriba a `ILogQueue`.
3. Eliminar paquetes Serilog del `.csproj`.
4. Actualizar tests (AdminWebAppFactory, E2EFixture) que invocan `Serilog.Log.CloseAndFlushAsync()`.
5. Validar build, tests y flujo de logs end-to-end.
