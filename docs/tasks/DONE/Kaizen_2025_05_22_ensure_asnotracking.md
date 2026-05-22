---
title: Ensure AsNoTracking is used in read-only queries
created: 2025-05-22
priority: High
---

# Kaizen: Ensure AsNoTracking is used in read-only queries

In `GesFer.Admin.Back.Application`, Entity Framework Core query handlers (read-only operations) using `IApplicationDbContext` must explicitly use `.AsNoTracking()` to prevent memory thermodynamic leaks.

Files to check and update:
- src/GesFer.Admin.Back.Application/Queries/Logs/GetAuditLogsQuery.cs
- src/GesFer.Admin.Back.Application/Queries/Logs/GetLogsQuery.cs
- src/GesFer.Admin.Back.Application/Handlers/Company/GetAllCompaniesHandler.cs
- src/GesFer.Admin.Back.Application/Handlers/Geo/GeoHandlers.cs
- src/GesFer.Admin.Back.Application/Handlers/User/GetAllUsersHandler.cs
