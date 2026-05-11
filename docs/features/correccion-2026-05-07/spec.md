---
base: ["main"]
scope:
  in_scope: ["src/GesFer.Admin.Back.Application/Queries/Logs/GetAuditLogsQuery.cs", "src/GesFer.Admin.Back.Application/Queries/Logs/GetLogsQuery.cs"]
  out_scope: []
---

# Especificaciones

Agregado `.AsNoTracking()` a las colecciones leídas en los `QueryHandlers` de solo lectura.
