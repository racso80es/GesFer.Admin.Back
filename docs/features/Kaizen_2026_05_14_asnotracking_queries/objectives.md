---
feature_name: "Kaizen_2026_05_14_asnotracking_queries"
created: "2026-05-14"
process: "feature"
---
# Objectives
Add .AsNoTracking() to read-only queries in Application project to avoid tracking memory issues and follow good practices.
Affected handlers: GetAuditLogsHandler, GetLogsHandler, GetAllCompaniesHandler, GeoHandlers (GetAllCountriesHandler, GetCountryByIdHandler, GetStatesByCountryIdHandler, GetCitiesByStateIdHandler, GetPostalCodesByCityIdHandler), GetAllUsersHandler.
