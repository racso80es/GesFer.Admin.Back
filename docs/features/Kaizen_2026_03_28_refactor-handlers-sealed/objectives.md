---
id: Kaizen_2026_03_28_refactor-handlers-sealed
type: feature
---
# Objetivos Kaizen (Refactorización Handlers)

## Objetivos
1. Agregar el modificador `sealed` a las clases Handler en `src/GesFer.Admin.Back.Application/` que implementan `IRequestHandler`.
2. Mejorar la adherencia a convenciones de C# moderno y prevenir la herencia no deseada de estas clases, optimizando el rendimiento.