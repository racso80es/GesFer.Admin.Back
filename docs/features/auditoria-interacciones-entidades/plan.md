# PLAN: Auditoría de Interacciones entre Entidades

**Feature:** auditoria-interacciones-entidades
**Fecha:** 2026-02-24

## Objetivo
Implementar la funcionalidad de auditoría y corregir errores críticos de arquitectura y compilación en `GesFer.Admin.Back`.

## Pasos de Ejecución

1.  **Documentación SddIA:**
    -   Crear `docs/features/auditoria-interacciones-entidades/clarify.md`.
    -   Crear `docs/features/auditoria-interacciones-entidades/plan.md`.

2.  **Implementación CQRS (Application Layer):**
    -   Implementar `CreateLogCommand` y `CreateLogHandler`.
    -   Implementar `CreateAuditLogCommand` y `CreateAuditLogHandler`.
    -   Implementar `GetLogsQuery` y `GetLogsHandler`.
    -   Implementar `PurgeLogsCommand` y `PurgeLogsHandler`.
    -   Asegurar existencia de DTOs necesarios en `GesFer.Admin.Back.Application.DTOs.Logs`.

3.  **Refactorización API (Api Layer):**
    -   Limpiar `LogController`: eliminar lógica de negocio y usar `IMediator`.
    -   Eliminar referencia a `GesFer.Admin.Back.Infrastructure` en `GesFer.Admin.Back.Api.csproj`.
    -   Eliminar paquetes de implementación (`Serilog.Sinks.MySQL`, `Pomelo`) de `Api`.

4.  **Refactorización Infraestructura (Infrastructure Layer):**
    -   Mover configuración de Serilog a `Infrastructure`.
    -   Exponer método de configuración en `DependencyInjection` para ser consumido por `Program.cs`.

5.  **Verificación:**
    -   Compilar solución (`dotnet build`).
    -   Ejecutar pruebas (`dotnet test`).

6.  **Cierre SddIA:**
    -   Generar `docs/features/auditoria-interacciones-entidades/implementation.md`.
    -   Generar `docs/features/auditoria-interacciones-entidades/validacion.json`.
