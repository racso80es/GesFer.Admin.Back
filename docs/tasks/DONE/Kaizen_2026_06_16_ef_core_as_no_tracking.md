---
id: Kaizen_2026_06_16_ef_core_as_no_tracking
created: "2026-06-16"
priority: high
status: done
type: kaizen
---
# Kaizen: Add AsNoTracking to readonly EF Core queries
## Objetivo
Asegurar el cumplimiento de la norma de evitar fugas de memoria en consultas de sólo lectura de EF Core usando \`.AsNoTracking()\`
## Hallazgos
1. Falta en \`src/GesFer.Admin.Back.Application/Handlers/Company/GetCompanyByNameHandler.cs\`
2. Falta en \`src/GesFer.Admin.Back.Application/Handlers/Company/GetCompanyByIdHandler.cs\`
3. Falta en \`src/GesFer.Admin.Back.Application/Handlers/Company/CreateCompanyHandler.cs\`
4. Falta en \`src/GesFer.Admin.Back.Application/Handlers/Company/UpdateCompanyHandler.cs\`
5. Falta en \`src/GesFer.Admin.Back.Application/Handlers/Geo/GeoHandlers.cs\`
## Acción
Agregar \`.AsNoTracking()\` en las consultas EF Core pertinentes.
