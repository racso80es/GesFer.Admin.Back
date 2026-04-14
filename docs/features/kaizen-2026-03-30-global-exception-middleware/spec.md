---
feature_name: kaizen-2026-03-30-global-exception-middleware
created: 2026-03-30
updated: 2026-03-30
base:
  - objectives.md
  - clarify.md
scope:
  in_scope:
    - Nuevo Archivo: `src/GesFer.Admin.Back.Api/Middleware/GlobalExceptionHandlingMiddleware.cs`.
    - Modificación: `src/GesFer.Admin.Back.Api/Program.cs`.
  out_scope: []
functional_requirements:
  - id: FR-01
    text: Interceptar excepciones y devolver códigos HTTP 400, 404, 500 según corresponda.
---
# Especificación Técnica

## Componentes a Modificar
1. **Nuevo Archivo:** `src/GesFer.Admin.Back.Api/Middleware/GlobalExceptionHandlingMiddleware.cs`.
2. **Modificación:** `src/GesFer.Admin.Back.Api/Program.cs`.
