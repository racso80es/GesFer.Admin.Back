---
feature_name: kaizen-as-no-tracking
created: 2026-05-10
base: ["GetLogsQuery.cs", "GetAuditLogsQuery.cs"]
scope:
  in_scope: ["Agregar .AsNoTracking() a GetLogsQuery", "Agregar .AsNoTracking() a GetAuditLogsQuery"]
  out_scope: ["Refactorización de otros módulos"]
---
# Especificación Técnica
Modificar `var query = _context.Logs.AsQueryable();` a `var query = _context.Logs.AsNoTracking().AsQueryable();`
Modificar `var query = _context.AuditLogs.AsQueryable();` a `var query = _context.AuditLogs.AsNoTracking().AsQueryable();`