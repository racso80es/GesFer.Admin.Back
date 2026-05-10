---
base:
  - GetAuditLogsQuery.cs
  - GetLogsQuery.cs
scope:
  in_scope: Apply AsNoTracking to ApplicationDbContext queries.
  out_scope: Changing other files.
---
# Specification
Fix EF Core read queries to use AsNoTracking for performance.