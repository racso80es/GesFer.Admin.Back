---
feature_name: "Kaizen_2026_05_14_asnotracking_queries"
created: "2026-05-14"
items:
  - Add docs since code is already updated.
---
# Implementation
The queries already have `.AsNoTracking()` applied.
Wait, let's look at the result of grep.

`src/GesFer.Admin.Back.Application/Queries/Logs/GetAuditLogsQuery.cs:        var query = _context.AuditLogs.AsNoTracking().AsQueryable();`
`src/GesFer.Admin.Back.Application/Queries/Logs/GetLogsQuery.cs:        var query = _context.Logs.AsNoTracking().AsQueryable();`
...
It seems `.AsNoTracking()` is ALREADY present in all read-only queries!
Therefore, the feature/kaizen task can be closed immediately.
