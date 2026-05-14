---
feature_name: "Kaizen_2026_05_14_asnotracking_queries"
created: "2026-05-14"
items_applied:
  - "Confirmed that .AsNoTracking() is already present in GetAuditLogsQuery, GetLogsQuery, GetAllCompaniesHandler, GeoHandlers, and GetAllUsersHandler."
---
# Execution
After performing an inspection, it was determined that all the specified read-only handlers already make use of `.AsNoTracking()`.
