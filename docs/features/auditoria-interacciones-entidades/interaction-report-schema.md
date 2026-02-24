# Schema del reporte de interacciones (INTERACCIONES_*.json y .md)

**Feature:** auditoria-interacciones-entidades  
**Referencia:** plan.md Fase 1, implementation.md ítem 1.1

## Objetivo

Definir la estructura exacta de los ficheros generados antes de cada commit en paths.auditsPath (Cúmulo: `./docs/audits/`), para que el generador y el auditor sean deterministas.

---

## 1. Fichero JSON: `INTERACCIONES_YYYY-MM-DD_HHmm.json`

### Campos mínimos

| Campo | Tipo | Obligatorio | Descripción |
|-------|------|-------------|-------------|
| `generated_at` | string (date-time ISO 8601) | Sí | Momento de generación del reporte. |
| `branch` | string | Sí | Rama de git en el momento de generación (ej. `feat/auditor-unificado-kaizen`). |
| `aggregations` | array de objetos | Sí | Una entrada por combinación (entity_type, entity_id, invoked_by) con conteo y última vez. |

### Estructura de cada elemento de `aggregations`

| Campo | Tipo | Descripción |
|-------|------|-------------|
| `entity_type` | string | Tipo de entidad de modelo (skill, tool, action, process u otro que implemente contrato de Token). |
| `entity_id` | string | Identificador de la entidad (skill_id, tool_id, action_id, process_id). |
| `invoked_by` | string | Identidad o agente que invocó (identity.issuer / invoked_by del token). |
| `count` | number | Número de veces que se invocó esta entidad por este invocador en la ventana considerada. |
| `last_timestamp` | string (date-time) | Timestamp de la última interacción registrada para esta combinación. |

### Ejemplo

```json
{
  "generated_at": "2026-02-23T23:45:00Z",
  "branch": "feat/auditor-unificado-kaizen",
  "aggregations": [
    {
      "entity_type": "skill",
      "entity_id": "invoke-command",
      "invoked_by": "cursor-agent",
      "count": 3,
      "last_timestamp": "2026-02-23T23:44:12Z"
    },
    {
      "entity_type": "tool",
      "entity_id": "prepare-full-env",
      "invoked_by": "cursor-agent",
      "count": 1,
      "last_timestamp": "2026-02-23T22:10:00Z"
    }
  ]
}
```

---

## 2. Fichero MD: `INTERACCIONES_YYYY-MM-DD_HHmm.md`

### Estructura

1. **Encabezado:** título y metadatos en texto.
   - `# Reporte de interacciones (entidades de modelo)`
   - `Generated at: <generated_at>`
   - `Branch: <branch>`

2. **Tabla:** una tabla Markdown con columnas legibles.

| Columna   | Origen        | Descripción breve        |
|-----------|----------------|---------------------------|
| Entidad   | entity_id      | ID de la entidad         |
| Tipo      | entity_type    | skill, tool, action, process |
| Invocador | invoked_by     | Quién invocó             |
| Conteo    | count          | Veces en la ventana      |
| Última vez| last_timestamp | Última interacción       |

### Ejemplo

```markdown
# Reporte de interacciones (entidades de modelo)

**Generated at:** 2026-02-23T23:45:00Z  
**Branch:** feat/auditor-unificado-kaizen

| Entidad          | Tipo   | Invocador    | Conteo | Última vez          |
|------------------|--------|--------------|--------|---------------------|
| invoke-command   | skill  | cursor-agent | 3      | 2026-02-23T23:44:12Z |
| prepare-full-env | tool   | cursor-agent | 1      | 2026-02-23T22:10:00Z |
```

---

## 3. Nombre base

- Formato: `INTERACCIONES_YYYY-MM-DD_HHmm` (ej. `INTERACCIONES_2026-02-23_2345`).
- Extensions: `.json` y `.md`.
- Ubicación: paths.auditsPath (por defecto `./docs/audits/`).
