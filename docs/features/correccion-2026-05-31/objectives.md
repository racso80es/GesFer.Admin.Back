---
type: objectives
feature_name: correccion-2026-05-31
branch: feat/correccion-2026-05-31
---

# Objetivos de Corrección de Auditoría (2026-05-31)

1. Propagar `CancellationToken` en operaciones de Entity Framework (como `ToListAsync`, `SaveChangesAsync`, `FirstOrDefaultAsync`) en la capa de Infraestructura y Aplicación para prevenir bloqueo del thread pool.
2. Añadir `.AsNoTracking()` en las consultas de solo lectura en el `AdminJsonDataSeeder.cs` para evitar fugas de memoria.
