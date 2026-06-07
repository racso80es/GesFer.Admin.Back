---
type: objectives
feature_name: correccion-2026-06-07
---

# Objetivos de la corrección de la auditoría 2026-06-07

## Resumen de la Auditoría
La auditoría reveló que la métrica de **Estabilidad Async** está al 85%. Se encontraron dos pain points principales relacionados con la ausencia de `CancellationToken` en operaciones asíncronas de la base de datos (Entity Framework Core).

## Hallazgos Priorizados

1. **[Medio] Falta de CancellationToken en AdminAuthService.cs**
   - **Ubicación**: `src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs` (Línea 37)
   - **Objetivo**: Actualizar `IAdminAuthService.cs` y `AdminAuthService.cs` para soportar y propagar `CancellationToken`.

2. **[Medio] Falta de propagación de CancellationToken en Seeders**
   - **Ubicación**: `src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs` (Múltiples llamadas a `ToListAsync()`)
   - **Objetivo**: Añadir un `CancellationToken` opcional (por defecto `default`) a los métodos `SeedDataAsync` y propagar en todas las llamadas de EF.

## Criterios de Cierre (DoD)
- El proyecto debe compilar correctamente.
- Todos los tests (unitarios, de integración) deben pasar.
- `AuthenticateAsync` requiere `CancellationToken`.
- `AdminJsonDataSeeder.SeedDataAsync` requiere/utiliza `CancellationToken`.
