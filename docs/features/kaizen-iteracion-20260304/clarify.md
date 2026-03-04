# Clarificaciones: Kaizen iteración 2026-03-04

**Feature:** kaizen-iteracion-20260304  
**Ruta (Cúmulo):** paths.featurePath/kaizen-iteracion-20260304

## Decisiones

| Tema | Decisión |
| :--- | :--- |
| Representación de output nulo en log | Guardar `Output` como string vacío `""` cuando `$output` sea `$null`. |
| Objetos no string | Si `$output` no es null pero no es string (p. ej. objeto), usar `.ToString()`; si falla, usar `""`. |
| Alcance | Solo Invoke-Command.ps1; no modificar .bat ni binarios Rust si existen. |

Sin ambigüedades pendientes para esta iteración.
