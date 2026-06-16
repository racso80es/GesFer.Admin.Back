---
type: objective
feature_name: kaizen-ef-core-as-no-tracking
---
# Objetivos: Kaizen EF Core AsNoTracking
## Objetivo
Asegurar el cumplimiento de la norma de evitar fugas de memoria en consultas de sólo lectura de EF Core usando \`.AsNoTracking()\`
Tras un análisis exhaustivo, se ha confirmado que en el proyecto *GesFer.Admin.Back.Application* todas las consultas relevantes de sólo lectura (GetCompanyByNameHandler, GetAllCompaniesHandler, GetAuditLogsHandler, GeoHandlers, CreateCompanyHandler, CreateUserHandler, etc.) YA INCLUYEN \`.AsNoTracking()\`.
En los handlers de Update/Delete, las consultas principales omiten AsNoTracking correctamente dado que las entidades resultantes se modifican luego.
No hay hallazgos reales de fugas de memoria que corregir respecto a esto.
Por lo tanto, la Kaizen se cerrará sin hacer cambios de código funcionales en Application, formalizando así el chequeo satisfactorio.
## Alcance
- Creación de documentación del proceso y finalización formal.
