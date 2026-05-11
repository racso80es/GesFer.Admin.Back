---
name: Kaizen_2026_05_10_AsNoTracking
process: automatic_task
created: 2026-05-10
priority: high
---
# Agregar AsNoTracking a Queries faltantes

Revisar los Query Handlers en `src/GesFer.Admin.Back.Application/` y agregar `.AsNoTracking()` a las consultas de solo lectura (`GetLogsQuery`, `GetAuditLogsQuery`, etc.) para prevenir fugas de memoria y seguir el estándar del proyecto.
