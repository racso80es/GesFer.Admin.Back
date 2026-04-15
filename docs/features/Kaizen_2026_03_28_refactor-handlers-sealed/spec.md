---
id: Kaizen_2026_03_28_refactor-handlers-sealed
type: feature
---
# Especificación: Acción Kaizen (Refactorización Handlers)

## Alcance
Esta especificación formaliza la refactorización de Handlers en la capa Application para hacerlos `sealed`.

## Acciones Requeridas
- Modificar los archivos en `src/GesFer.Admin.Back.Application/` cambiando `public class` por `public sealed class` en las clases de tipo Handler.
- Validar que el proyecto compila y los tests pasan exitosamente.