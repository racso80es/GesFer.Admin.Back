---
type: objectives
feature_name: correccion-2026-05-23
status: active
---

# Objetivos de Corrección de Auditoría

1. **Propagación de CancellationToken y AsNoTracking() en AdminAuthService**
   - Actualizar la firma de `IAdminAuthService.AuthenticateAsync` para recibir `CancellationToken`.
   - Agregar `.AsNoTracking()` y propagar el token a `FirstOrDefaultAsync` en `AdminAuthService`.
   - Propagar el token en los consumidores (e.g. `AdminLoginHandler`) y actualizar los tests.

2. **Propagación de CancellationToken en AdminJsonDataSeeder**
   - Propagar `CancellationToken` a los métodos de inserción y lectura asincrónica de bases de datos para evitar bloqueos del pool de hilos.

Criterios de éxito: Todos los proyectos compilan sin errores, todos los tests pasan y las métricas de estabilidad asíncrona mejoran a 100%.
