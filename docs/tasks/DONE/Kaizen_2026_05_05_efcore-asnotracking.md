---
name: Kaizen 2026-05-05 efcore-asnotracking
process: automatic_task
created: 2026-05-05
priority: high
---
# Acción Kaizen: Optimización Termodinámica EF Core (AsNoTracking)

## Descripción
Se ha detectado una fuga termodinámica en la capa `GesFer.Admin.Back.Application`: los Query Handlers (ej. `GetAllCompaniesHandler`, `GetAllUsersHandler`, `GetCompanyByIdHandler`, etc.) no utilizan `.AsNoTracking()` al consultar datos a través de `IApplicationDbContext`. Esto provoca que Entity Framework Core cargue las entidades en el Change Tracker innecesariamente para operaciones de solo lectura, degradando el rendimiento y consumiendo memoria.

## Pasos a realizar
1. Ejecutar el proceso `automatic_task` instanciando un ciclo `feature`.
2. Crear la documentación de la feature en `docs/features/kaizen-ef-asnotracking/`.
3. Refactorizar los Query Handlers para inyectar `.AsNoTracking()` en las consultas.
4. Asegurarse de que el repositorio compile y las pruebas pasen sin errores.
5. Finalizar la tarea, actualizar el log de evolución.
