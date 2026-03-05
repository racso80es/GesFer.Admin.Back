# SPEC: Kaizen iteración 2026-03-04

**Feature:** kaizen-iteracion-20260304  
**Ruta (Cúmulo):** paths.featurePath/kaizen-iteracion-20260304  
**Origen:** objectives.md

---

## 1. Contexto y objetivo

Iteración de mejora continua (Kaizen): corregir un fallo detectado en el uso de la skill **invoke-command** y asegurar que la telemetría (execution_history.json) se persista correctamente aunque el comando no produzca salida en stdout.

## 2. Alcance técnico

### 2.1 Problema

En `scripts/skills/invoke-command/Invoke-Command.ps1`, al construir la entrada de log (paso 4), se usa `$output.ToString()`. Cuando el comando ejecutado no devuelve nada (p. ej. `git fetch origin`), `$output` es `$null` y se produce la excepción *"No se puede llamar a un método en una expresión con valor NULL"*, rompiendo el script aunque el comando haya terminado correctamente.

### 2.2 Solución

- Tratar `$output` como potencialmente nulo o no-string antes de serializar a JSON.
- Usar una representación segura para el campo `Output` del log (p. ej. string vacío o texto fijo cuando sea null).
- No cambiar la firma del script ni el contrato de la skill (spec.json); solo robustez interna.

### 2.3 Archivos afectados

| Archivo | Cambio |
| :--- | :--- |
| `scripts/skills/invoke-command/Invoke-Command.ps1` | En el bloque que construye `$logEntry`, sustituir `$output.ToString()` por una expresión que maneje `$null` (p. ej. `$(if ($null -eq $output) { "" } else { $output.ToString() })` o equivalente). |

## 3. Criterios de aceptación

- [ ] Ejecutar `Invoke-Command.ps1 -Command "git fetch origin"` no lanza excepción y el comando se considera exitoso.
- [ ] Se crea o actualiza `docs/diagnostics/<branch>/execution_history.json` con una entrada que incluye `Output` como string (vacío si no hubo salida).
- [ ] Comportamiento existente para comandos que sí devuelven salida se mantiene.
- [ ] Build del backend (dotnet build) sigue pasando.

## 4. Dependencias

- Skill invoke-command: SddIA/skills/invoke-command/spec.json.
- Cápsula: paths.skillCapsules.invoke-command (scripts/skills/invoke-command/).

## 5. Riesgos

- Ninguno significativo; cambio localizado y de defensa ante null.
