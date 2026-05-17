---
feature_name: kaizen-asnotracking
created: 2026-05-10
process: automatic_task
---
# Objetivo
Agregar `.AsNoTracking()` a las consultas de validación previas a mutaciones (lectura) en los Handlers del directorio `src/GesFer.Admin.Back.Application/` que lo permitan (ej. validaciones de unicidad), para prevenir fugas de memoria y seguir el estándar del proyecto.
