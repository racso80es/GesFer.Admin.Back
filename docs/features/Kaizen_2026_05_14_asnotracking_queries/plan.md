---
feature_name: "Kaizen_2026_05_14_asnotracking_queries"
created: "2026-05-14"
phases:
  - id: 1
    name: "Modify Queries/Logs/GetAuditLogsQuery.cs"
  - id: 2
    name: "Modify Queries/Logs/GetLogsQuery.cs"
  - id: 3
    name: "Modify Handlers/Company/GetAllCompaniesHandler.cs"
  - id: 4
    name: "Modify Handlers/Geo/GeoHandlers.cs"
  - id: 5
    name: "Modify Handlers/User/GetAllUsersHandler.cs"
  - id: 6
    name: "Move Task to DONE and generate finalize docs"
---
# Plan
1. Add `.AsNoTracking()` before `.ToListAsync` in `GetAuditLogsQuery.cs`.
2. Add `.AsNoTracking()` before `.ToListAsync` in `GetLogsQuery.cs`.
3. Add `.AsNoTracking()` before `.ToListAsync` in `GetAllCompaniesHandler.cs`.
4. Add `.AsNoTracking()` before `.ToListAsync` / `.FirstOrDefaultAsync` in all read handlers in `GeoHandlers.cs`.
5. Add `.AsNoTracking()` before `.ToListAsync` in `GetAllUsersHandler.cs`.
6. Update EVOLUTION_LOG.md and move the kaizen file to `docs/tasks/DONE/`. Generate `finalize-process.md`.
