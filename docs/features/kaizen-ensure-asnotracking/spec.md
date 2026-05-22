---
title: Kaizen Ensure AsNoTracking Spec
type: spec
base: ["docs/tasks/ACTIVE/Kaizen_2025_05_22_ensure_asnotracking.md"]
scope:
  in_scope: ["src/GesFer.Admin.Back.Application/Queries/Logs/GetAuditLogsQuery.cs", "src/GesFer.Admin.Back.Application/Queries/Logs/GetLogsQuery.cs", "src/GesFer.Admin.Back.Application/Handlers/Company/GetAllCompaniesHandler.cs", "src/GesFer.Admin.Back.Application/Handlers/Geo/GeoHandlers.cs", "src/GesFer.Admin.Back.Application/Handlers/User/GetAllUsersHandler.cs"]
  out_scope: []
---

# Specification
The goal is to modify the handlers such that their query objects use `AsNoTracking()`. Upon checking the code, it is verified that `GetAuditLogsQuery.cs`, `GetLogsQuery.cs`, `GetAllCompaniesHandler.cs`, `GeoHandlers.cs`, and `GetAllUsersHandler.cs` actually already implement `.AsNoTracking()` in their initial query declarations. Therefore, the code already meets the requirement, and no code modifications are needed. I will generate the finalization process document and move the task to DONE.
