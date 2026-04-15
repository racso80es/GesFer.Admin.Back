---
feature_name: kaizen-2026-03-30-global-exception-middleware
created: 2026-03-30
items_applied:
  - id: middleware-code
    path: src/GesFer.Admin.Back.Api/Middleware/GlobalExceptionHandlingMiddleware.cs
    action: create
    status: OK
    message: Creado middleware para manejo centralizado.
  - id: program-code
    path: src/GesFer.Admin.Back.Api/Program.cs
    action: update
    status: OK
    message: Registrado middleware en pipeline.
---
# Ejecución
- Se ha creado el archivo de middleware en `src/GesFer.Admin.Back.Api/Middleware/GlobalExceptionHandlingMiddleware.cs`.
- Se ha registrado el middleware en `Program.cs`.
