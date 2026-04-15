---
name: Kaizen 2026-03-30 global-exception-middleware
process: automatic_task
created: 2026-03-30
priority: high
---
# Acción Kaizen: Implementar GlobalExceptionHandlingMiddleware

## Descripción
Se detecta la ausencia del GlobalExceptionHandlingMiddleware en GesFer.Admin.Back.Api. Es necesario crearlo y registrarlo para centralizar el manejo de excepciones y evitar try-catch repetitivos en los controladores.

## Pasos a realizar
1. Crear `src/GesFer.Admin.Back.Api/Middleware/GlobalExceptionHandlingMiddleware.cs`.
2. Registrar el middleware en `src/GesFer.Admin.Back.Api/Program.cs`.
3. Crear documentación de la feature en `docs/features/kaizen-2026-03-30-global-exception-middleware/`.
4. Asegurarse de que el repositorio compile y los tests pasen.
5. Finalizar la tarea y actualizar el log de evolución.
