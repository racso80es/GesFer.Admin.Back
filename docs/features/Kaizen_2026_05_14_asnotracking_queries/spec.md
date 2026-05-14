---
feature_name: "Kaizen_2026_05_14_asnotracking_queries"
created: "2026-05-14"
base: ["src/GesFer.Admin.Back.Application"]
scope:
  in_scope: ["Add .AsNoTracking() to specific read-only QueryHandlers"]
  out_scope: ["Modify any write operations or other layers"]
---
# Spec
Apply `.AsNoTracking()` to the `IApplicationDbContext` Entity Framework Core query chains in read-only Query Handlers across the `GesFer.Admin.Back.Application` project, specifically addressing `GetAuditLogsHandler`, `GetLogsHandler`, `GetAllCompaniesHandler`, `GetAllCountriesHandler`, `GetCountryByIdHandler`, `GetStatesByCountryIdHandler`, `GetCitiesByStateIdHandler`, `GetPostalCodesByCityIdHandler`, and `GetAllUsersHandler`.
