# Plan: Auditoría de interacciones entre entidades

**Feature:** auditoria-interacciones-entidades  
**Ruta (Cúmulo):** paths.featurePath/auditoria-interacciones-entidades/  
**Referencias:** spec.md, clarify.md, clarify.json

## Objetivo del plan

Transformar la especificación y clarificación en una hoja de ruta ejecutable para que, **antes de cada commit**, se generen en paths.auditsPath los ficheros documentales de interacciones (JSON y MD) con la información contabilizada de entidades de modelo (que implementan contrato de Token).

## Fases

| Fase | Alcance | Entregable |
|------|---------|------------|
| **1** | Estructura del reporte | Schema y ejemplo de INTERACCIONES_*.json y .md; anexo en carpeta feature o docs/audits. |
| **2** | Fuente de datos | Definir y documentar de dónde se leen las interacciones (execution_history, ACCESS_LOG, o interactions log); contrato de lectura. |
| **3** | Generador | Script (PowerShell o Rust) que agregue interacciones y escriba INTERACCIONES_&lt;timestamp&gt;.json y .md en paths.auditsPath. |
| **4** | Integración pre-commit | Hook pre-commit (o invocación desde invoke-command) que ejecute el generador antes de cada commit. |
| **5** | Verificación y auditoría | Comprobar generación en commit de prueba; registrar evento en paths.auditsPath. |

---

## Fase 1: Estructura del reporte (JSON y MD)

**Objetivo:** Fijar estructura exacta del JSON y del MD para que implementación y auditor sean deterministas.

**Tareas técnicas:**

1. Crear anexo `docs/features/auditoria-interacciones-entidades/interaction-report-schema.md` con:
   - Campos mínimos del JSON: `generated_at`, `branch`, `aggregations` (por entity_type, entity_id, invoked_by; conteos y/o lista de timestamps).
   - Ejemplo de `INTERACCIONES_YYYY-MM-DD_HHmm.json`.
   - Formato del MD: tabla(s) legibles (entidad, tipo, invocador, conteo, última vez); encabezado con generated_at y branch.

2. Opcional: añadir `interaction-report-schema.json` (JSON Schema) para validación machine-readable.

**Criterio de aceptación:** Un implementador puede generar los dos ficheros sin ambigüedad.

---

## Fase 2: Fuente de datos

**Objetivo:** Definir de dónde el generador lee las interacciones (entity_type, entity_id, invoked_by, timestamp).

**Tareas técnicas:**

1. Decidir fuente(s) canónica(s):
   - **Opción A:** `docs/diagnostics/{branch}/execution_history.json` (si invoke-command ya escribe ahí datos compatibles con interaction_audit).
   - **Opción B:** `paths.auditsPath/ACCESS_LOG.md` o `paths.auditsPath/interactions/interactions.json` (formato acordado).
   - **Opción C:** Combinación: el generador lee de una o varias rutas y agrega.

2. Documentar en el anexo (o en spec) el contrato de lectura: nombres de campos, formato de fecha, encoding.

3. Si hace falta, extender invoke-command (o el flujo que registra interacciones) para que persista en la fuente elegida los campos `entity_type`, `entity_id`, `invoked_by`, `timestamp` por cada invocación.

**Criterio de aceptación:** Existe al menos una fuente documentada desde la que el generador puede leer filas de interacción.

---

## Fase 3: Generador (script)

**Objetivo:** Implementar el componente que genera INTERACCIONES_&lt;timestamp&gt;.json y .md.

**Tareas técnicas:**

1. Crear script en `scripts/` o en cápsula de skill (por ejemplo `scripts/skills/invoke-command/` o `scripts/tools/`):
   - Entrada: ruta(s) de fuente de datos (o leer desde paths definidos en Cúmulo).
   - Salida: escribir en **paths.auditsPath** (resolver vía Cúmulo o config) dos ficheros: mismo nombre base, extensiones `.json` y `.md`.

2. Lógica:
   - Leer y parsear fuente(s).
   - Agregar por (entity_type, entity_id, invoked_by) con conteo y opcionalmente última timestamp.
   - Generar JSON según schema del Fase 1.
   - Generar MD (tabla) según schema del Fase 1.

3. Nombre base: `INTERACCIONES_YYYY-MM-DD_HHmm` (ej. desde `Get-Date` en PowerShell o equivalente en Rust).

4. Asegurar que paths.auditsPath exista (crear directorio si no existe).

**Criterio de aceptación:** Ejecución manual del script produce los dos ficheros en docs/audits/ con contenido válido.

**Seguridad [REF-SEC]:**
- El script **solo** escribe en paths.auditsPath (no aceptar rutas arbitrarias por parámetro sin validación).
- Rutas de lectura fijas o resueltas desde Cúmulo; no ejecutar código externo inyectado.

---

## Fase 4: Integración pre-commit

**Objetivo:** Que antes de cada commit se ejecute el generador.

**Tareas técnicas:**

1. **Opción recomendada (hook):**
   - Añadir en `.git/hooks/pre-commit` (o vía Husky si se usa) la invocación del script generador.
   - El hook debe ejecutarse desde la raíz del repo; rutas relativas a paths.auditsPath según Cúmulo (ej. `./docs/audits/`).

2. **Alternativa (invoke-command):**
   - Documentar en invoke-command o en la acción de commit que, antes de `git commit`, se debe invocar el generador (por ejemplo con `--command-file` o paso explícito en el flujo).
   - No bloqueante para el commit si el generador falla (o sí, según decisión de producto): aclarar en plan o en spec.

3. **Decisión documentada:** El pre-commit **no bloquea** el commit cuando falla la generación del reporte. El hook (scripts/git-hooks/pre-commit, a copiar a .git/hooks/pre-commit) intenta ejecutar el generador y siempre hace `exit 0` para no impedir el commit. Ver scripts/git-hooks/README.md.

**Criterio de aceptación:** Tras configurar el hook (o el flujo), un `git commit` desencadena la generación de INTERACCIONES_*.json y .md cuando el hook/flujo se ejecuta.

---

## Fase 5: Verificación y auditoría

**Objetivo:** Validar que el flujo end-to-end cumple el spec y registrar el evento.

**Tareas técnicas:**

1. **Verificación manual o automatizada:**
   - Realizar un commit de prueba (cambio trivial).
   - Comprobar que en `docs/audits/` (paths.auditsPath) existen `INTERACCIONES_YYYY-MM-DD_HHmm.json` y `INTERACCIONES_YYYY-MM-DD_HHmm.md` con contenido coherente (misma ventana temporal, misma rama).

2. **Auditoría:**
   - El evento "generación de reporte de interacciones" puede registrarse en paths.auditsPath o en paths.accessLogFile (según contrato existente).
   - Actualizar validacion.json de la feature con un check opcional: "pre-commit genera INTERACCIONES_*.json y .md".

**Criterio de aceptación:** Al menos un commit de prueba genera los ficheros; el resultado es comprobable y documentado.

---

## Resumen de entregables

| Entregable | Fase |
|------------|------|
| interaction-report-schema.md (y opcional .json) | 1 |
| Documentación fuente de datos (contrato de lectura) | 2 |
| Script generador (PowerShell o Rust) | 3 |
| Pre-commit hook (o flujo invoke-command) | 4 |
| Verificación documentada + validacion.json (opcional) | 5 |

---

## Referencias

- SPEC: spec.md, spec.json
- Clarificación: clarify.md, clarify.json
- Contrato Token: SddIA/tokens/tokens-contract.json (interaction_audit)
- Cúmulo paths: paths.auditsPath = ./docs/audits/
