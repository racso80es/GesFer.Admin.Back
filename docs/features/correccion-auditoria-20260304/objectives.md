# Objetivos: Corrección Auditoría 2026-03-04

**Fecha:** 2026-03-04
**Proceso:** correccion-auditorias

## Resumen
Consolidación de hallazgos reportados en `docs/audits/AUDITORIA_2026_03_04.md` y priorización para ejecución.

## Hallazgos Consolidados

### Medios (🟡)

1. **Acoplamiento de Paquete NuGet de Infraestructura en capa API**
   - **Descripción:** El proyecto `GesFer.Admin.Back.Api` tiene referencia a `Microsoft.EntityFrameworkCore.Design`. Debe estar confinado a `Infrastructure`.
   - **Ubicación:** `src/GesFer.Admin.Back.Api/GesFer.Admin.Back.Api.csproj`, línea 12.
   - **Prioridad:** Alta (Acción Kaizen 1).

2. **Instanciación manual de DTOs en Handlers**
   - **Descripción:** Varios Handlers de `Application` instancian manualmente DTOs en consultas LINQ.
   - **Ubicación:** `GetAllCompaniesHandler.cs`, `GetCompanyByIdHandler.cs`, `GetCompanyByNameHandler.cs`, `GeoHandlers.cs`.
   - **Prioridad:** Media/Baja (Fuera de alcance inicial de esta iteración enfocada en la Hoja de Ruta Kaizen, sin embargo, se evaluará si el tiempo lo permite o en una futura iteración).

## Alcance Definido (Scope)

Se abordará el hallazgo crítico/medio establecido como **Acción Kaizen 1**: *Desacoplar Microsoft.EntityFrameworkCore.Design de la capa API*.

## Criterios de Cierre (DoD)
1. `GesFer.Admin.Back.Api.csproj` ya no tiene el paquete `Microsoft.EntityFrameworkCore.Design`.
2. El proyecto compila correctamente (`dotnet build`).
3. Los tests pasan correctamente (`dotnet test`).
