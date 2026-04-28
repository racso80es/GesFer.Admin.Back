---
process_id: audit-tool
spec_version: 3.0.0
tool_id: prepare-full-env
audit_date: 2026-04-27
audit_id: audit-2026-04-27-02
tool_spec_ref: paths.toolsDefinitionPath/prepare-full-env/spec.md
capsule_path_ref: paths.toolCapsules[prepare-full-env]
process_master_ref: paths.processPath/audit-tool/spec.md
---

# Objectives — audit-tool (prepare-full-env)

## Fase 0 — Preparar entorno y Clasificar (registro)

### Existencia en Cúmulo (SSOT)

- **tool-id:** `prepare-full-env` (kebab-case).
- **paths.toolCapsules[prepare-full-env]** (según `SddIA/agents/cumulo.paths.json`): `./scripts/tools/prepare-full-env/`
- **paths.toolsDefinitionPath/prepare-full-env/spec.md** (definición SddIA): `./SddIA/tools/prepare-full-env/spec.md`

### Taxonomía operativa (clasificación)

**Batch / Mutador de entorno**

- **Justificación:** la herramienta, según su **Objetivo** en `tool_spec_ref`, prepara el entorno levantando servicios Docker y esperando a MySQL; eso constituye **efecto secundario observable** sobre el sistema (contenedores/red/volúmenes), no un daemon permanente ni un transformador puro sin estado externo.
- **Implicación Fase 6:** validación por **efecto secundario observable** (servicios Docker y readiness de MySQL; seeds solo si el escenario de prueba las activa).
- **Implicación Fase 8 (cleanup):** reversión **solo** si la tool documenta explícitamente un comando/política de reversión; si no existe, se documenta como NO-OP con evidencia (según `paths.processPath/audit-tool/spec.md`).

---

## Fase 1 — Definir objetivos dinámicos (criterios de éxito)

### Verdad contractual — única fuente de aserciones funcionales (Fase 6)

Las aserciones de la **Fase 6** se derivan **exclusivamente** del texto de la sección **«Objetivo»** en `paths.toolsDefinitionPath/prepare-full-env/spec.md` (cuerpo Markdown, no inferencias de otras secciones):

> Herramienta que prepara el entorno de desarrollo: levanta servicios Docker (MySQL, Memcached, Adminer), espera a que MySQL esté listo y opcionalmente restaura las seeds de datos.

#### Aserciones funcionales (derivación literal)

| ID | Texto literal en «Objetivo» | Aserción ejecutable (Fase 6, tipología Batch) |
|:---|:----------------------------|:-----------------------------------------------|
| **A-OBJ-1** | «levanta servicios Docker (MySQL, Memcached, Adminer)» | Tras la invocación del escenario, existe evidencia observable de que **MySQL**, **Memcached** y **Adminer** quedan operativos según lo definido por la configuración/manifest de la tool (p. ej. contenedores/servicios compose esperados en `up` y accesibles según la promesa del escenario). |
| **A-OBJ-2** | «espera a que MySQL esté listo» | Tras la invocación del escenario, existe evidencia de **readiness** de MySQL (p. ej. healthcheck exitoso o equivalente documentado por la tool) **antes** de considerar cumplido el objetivo central del escenario. |
| **A-OBJ-3** | «opcionalmente restaura las seeds de datos» | **Solo** si el escenario de prueba activa explícitamente esa rama (flags/config que la spec presenta como entradas): existe evidencia de que se ejecutó la restauración de seeds **o** evidencia inequívoca de que no aplica al escenario (y entonces la aserción queda **N/A** documentada, sin convertirse en fallo por omisión). |

### Criterios contractuales (envelope JSON v2)

Independientes del «Objetivo», pero obligatorios por proceso maestro:

- **Salida:** un único JSON válido UTF-8 por **stdout** o por **ruta** si la invocación usa `--output-path` según spec de la tool.
- **Envelope:** cumple `SddIA/norms/capsule-json-io.md` (schema `"2.0"`): `meta`, `success`, `exitCode`, `message`, `feedback[]`, **`result`** (payload; **no** `data`), `duration_ms` si aplica.
- **Coherencia:** `exitCode === 0` si y solo si `success === true` (regla del envelope).

### Trazabilidad (preparación Fase 5)

- **Fases esperadas en `feedback[].phase`:** según `tool_spec_ref` frontmatter, `output.phases_feedback`: `init`, `docker`, `mysql`, `api`, `clients`, `done`, `error`.

### Alcance de ejecución (Fases 2–8 cerradas)

- **Completadas:** Fases **2–8** (incluye informe oficial **PARTIAL** y reversión `docker compose down`).
- **Entregables finales de auditoría:** `paths.auditsPath/tools/prepare-full-env/audit-report-2026-04-27-01.md`, `audit-result-2026-04-27-01.json`.
