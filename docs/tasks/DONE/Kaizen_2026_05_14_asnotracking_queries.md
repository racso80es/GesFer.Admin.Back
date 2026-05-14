---
name: "AsNoTracking for EF Core Queries"
process: "feature"
created: "2026-05-14"
priority: "high"
---
# AsNoTracking for Queries
Add .AsNoTracking() to read-only queries in Application project to avoid tracking memory issues and follow good practices.
Affected handlers: GetAuditLogsHandler, GetLogsHandler, GetAllCompaniesHandler, GeoHandlers (GetAllCountriesHandler, GetCountryByIdHandler, GetStatesByCountryIdHandler, GetCitiesByStateIdHandler, GetPostalCodesByCityIdHandler), GetAllUsersHandler.
