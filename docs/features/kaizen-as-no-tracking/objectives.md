---
feature_name: kaizen-as-no-tracking
created: 2026-05-10
process: automatic_task
---
# Objetivos de Kaizen AsNoTracking
Agregar `.AsNoTracking()` a las consultas de solo lectura en los queries `GetLogsQuery` y `GetAuditLogsQuery` para prevenir fugas de memoria.