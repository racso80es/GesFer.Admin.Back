---
type: objectives
feature_name: correccion-2026-06-09
status: active
---

# Objetivos de la Tarea: correccion-2026-06-09

## 1. Meta Principal
Aplicar las correcciones derivadas de la auditoría del 2026-06-09 para mejorar la estabilidad asíncrona del proyecto, específicamente propagando `CancellationToken` en las consultas de Entity Framework.

## 2. Alcance
- Modificar `IAdminAuthService` y `AdminAuthService` para propagar `CancellationToken`.
- Modificar `AdminJsonDataSeeder` para propagar `CancellationToken`.
- Actualizar los tests que hagan uso de las implementaciones anteriores si es necesario.

## 3. Criterios de Éxito (Definition of Done)
- El proyecto compila correctamente.
- Los tests pasan exitosamente.
- La búsqueda de `.FirstOrDefaultAsync()` y `.ToListAsync()` en los servicios auditados muestra el uso explícito del `CancellationToken`.
