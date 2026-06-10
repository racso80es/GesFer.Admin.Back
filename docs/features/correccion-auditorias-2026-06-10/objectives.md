---
type: objectives
feature_name: correccion-auditorias-2026-06-10
---

# Objetivos de Corrección de Auditoría

1. Corregir el memory leak y mejorar la estabilidad en `AdminAuthService.cs`
   - Agregar `.AsNoTracking()` a la consulta en `AuthenticateAsync`.
   - Propagar `CancellationToken` a través de `IAdminAuthService`, `AdminAuthService` y la consulta a la base de datos.
   - Actualizar el uso en `AdminLoginHandler.cs`.
   - Actualizar los tests unitarios.
