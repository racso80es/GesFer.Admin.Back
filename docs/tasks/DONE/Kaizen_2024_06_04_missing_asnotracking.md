---
created: 2024-06-04
priority: high
type: kaizen
---
# Agregar AsNoTracking a consultas read-only

Durante el triaje automático se detectó que existen múltiples consultas a base de datos de solo lectura (por ejemplo, llamadas a `ToListAsync()` en Handlers/Queries) que no están empleando `.AsNoTracking()`. Esto incumple las directrices de optimización en memoria de Entity Framework Core.

**Objetivo:** Modificar las consultas de solo lectura identificadas para que incluyan `.AsNoTracking()` donde corresponda, evitando fugas de memoria y bloqueos innecesarios.
