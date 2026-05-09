---
type: spec
base:
  - src/GesFer.Admin.Back.Application/Queries/Logs/GetAuditLogsQuery.cs
  - src/GesFer.Admin.Back.Application/Queries/Logs/GetLogsQuery.cs
scope:
  in_scope:
    - Add `.AsNoTracking()` to read-only queries in Application project.
  out_scope:
    - Modifications outside of the specified files.
---
# Especificaciones
Añadir `.AsNoTracking()` a las consultas de solo lectura en:
- `src/GesFer.Admin.Back.Application/Queries/Logs/GetAuditLogsQuery.cs`
- `src/GesFer.Admin.Back.Application/Queries/Logs/GetLogsQuery.cs`
