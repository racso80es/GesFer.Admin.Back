# Contrato de lectura: fuente de datos para reporte de interacciones

**Feature:** auditoria-interacciones-entidades  
**Referencia:** plan.md Fase 2, implementation.md ítem 2.1

## Objetivo

Definir de dónde el generador de reportes (`Generate-InteractionsReport.ps1`) lee las interacciones para producir `INTERACCIONES_*.json` y `.md`. El contrato fija rutas, nombres de campos y formato.

---

## Fuentes canónicas

### Opción A (actual): `docs/diagnostics/{branch}/execution_history.json`

- **Ruta:** `docs/diagnostics/<branch>/execution_history.json` (branch = rama git actual).
- **Formato:** Una línea por ejecución; cada línea es un objeto JSON con:
  - `Timestamp`: string (ej. `"2026-02-23 23:26:51"`).
  - `Fase`: string (ej. `"Accion"`).
  - `Command`: string (comando ejecutado, ej. `"git commit ..."` o `"git add ..."`).
  - `Status`: string (`Success` | `Failed`).
  - `Output`: string (salida o mensaje de error).

- **Mapeo a campos de interacción (para agregación):**
  - `entity_type`: si `Command` contiene `invoke-command` → `"skill"`; si empieza por `git` → `"command"`; si no → `"command"`.
  - `entity_id`: primer token del comando (ej. `git`, `dotnet`) o `invoke-command` si aparece en el comando.
  - `invoked_by`: `"system"` (invoke-command no registra identidad aún; extensible).
  - `timestamp`: valor de `Timestamp` normalizado a ISO 8601 si es posible, si no se usa tal cual.

- **Encoding:** UTF-8. Si el fichero no existe o está vacío, el generador produce reporte con `aggregations: []`.

### Opción B (futura): `paths.auditsPath/interactions/interactions.json`

- **Ruta:** `./docs/audits/interactions/interactions.json` (Cúmulo: paths.auditsPath + `interactions/interactions.json`).
- **Formato esperado:** Array JSON de objetos con campos del contrato de Token:
  - `entity_type`, `entity_id`, `invoked_by`, `timestamp` (string date-time).
- Cuando exista este fichero y esté poblado por invoke-command (u otro), el generador podrá usarlo como fuente primaria y no inferir desde execution_history.

### Opción C (combinación)

- El generador puede leer primero `interactions/interactions.json` si existe; si no, usa `execution_history.json` con el mapeo anterior.
- Permite migración gradual sin romper el reporte.

---

## Resolución de rutas

- Rutas relativas al **raíz del repositorio** (donde se ejecuta el script).
- paths.auditsPath según Cúmulo: `./docs/audits/`.
- Branch: salida de `git rev-parse --abbrev-ref HEAD`.

---

## Formato de fecha

- **Salida (reporte):** ISO 8601 (ej. `2026-02-23T23:26:51Z`).
- **Entrada (execution_history):** se acepta `yyyy-MM-dd HH:mm:ss` y se convierte a ISO 8601 en el reporte cuando sea posible.
