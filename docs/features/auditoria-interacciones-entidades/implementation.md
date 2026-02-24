# Implementación: Auditoría de interacciones entre entidades

**Feature:** auditoria-interacciones-entidades  
**Plan:** plan.md | **SPEC:** spec.md | **Clarify:** clarify.md  
**Ruta (Cúmulo):** paths.featurePath/auditoria-interacciones-entidades/

---

## Ítems de implementación

Cada ítem: **Id** | **Acción** | **Ruta** | **Ubicación** | **Propuesta** | **Dependencias**

### Fase 1 — Estructura del reporte (JSON y MD)

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| 1.1 | Crear | docs/features/auditoria-interacciones-entidades/interaction-report-schema.md | (nuevo) | Anexo con: campos mínimos JSON (generated_at, branch, aggregations por entity_type, entity_id, invoked_by con conteos/timestamps); ejemplo INTERACCIONES_YYYY-MM-DD_HHmm.json; formato MD (tabla: entidad, tipo, invocador, conteo, última vez; encabezado generated_at, branch). | — |
| 1.2 | Crear | docs/features/auditoria-interacciones-entidades/interaction-report-schema.json | (nuevo) | JSON Schema para validación del reporte INTERACCIONES_*.json: generated_at (string date-time), branch (string), aggregations (array de objetos con entity_type, entity_id, invoked_by, count, last_timestamp). | 1.1 |

### Fase 2 — Fuente de datos (documentación)

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| 2.1 | Crear | docs/features/auditoria-interacciones-entidades/data-source-contract.md | (nuevo) | Documentar contrato de lectura: fuente(s) canónica(s) (execution_history, ACCESS_LOG o interactions.json), nombres de campos (entity_type, entity_id, invoked_by, timestamp), formato fecha, encoding. Decidir Opción A/B/C del plan. | 1.2 |
| 2.2 | Modificar | docs/features/auditoria-interacciones-entidades/spec.md | Sección Cambios / Auditor | Añadir referencia a data-source-contract.md y path de lectura (paths.auditsPath o docs/diagnostics/{branch}/). | 2.1 |

### Fase 3 — Generador (script)

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| 3.1 | Crear | scripts/audits/Generate-InteractionsReport.ps1 | (nuevo) | Script PowerShell: leer fuente(s) según data-source-contract; agregar por (entity_type, entity_id, invoked_by); escribir en paths.auditsPath (./docs/audits/) INTERACCIONES_YYYY-MM-DD_HHmm.json y .md según interaction-report-schema. Crear directorio si no existe. Solo escribir en paths.auditsPath. | 2.1, 1.2 |
| 3.2 | Modificar | SddIA/agents/cumulo.paths.json | paths | Asegurar auditsPath presente (ya existe). Opcional: añadir interactionReportScript si se quiere ruta canónica al script. | — |

### Fase 4 — Integración pre-commit

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| 4.1 | Crear | .husky/pre-commit o .git/hooks/pre-commit | (nuevo) | Hook que invoque scripts/audits/Generate-InteractionsReport.ps1 (o la ruta acordada) desde raíz del repo antes de completar el commit. Documentar si bloquea o solo registra fallo. | 3.1 |
| 4.2 | Modificar | docs/features/auditoria-interacciones-entidades/plan.md | Fase 4 | Añadir decisión: bloquea commit si falla generación (sí/no). | 4.1 |

### Fase 5 — Verificación

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| 5.1 | Crear | docs/features/auditoria-interacciones-entidades/validacion.json | (nuevo) | JSON con checks: pre_commit_generates_interactions_report (boolean o descripción); opcional lista de archivos a comprobar. | 4.1 |
| 5.2 | Modificar | docs/features/auditoria-interacciones-entidades/plan.json | phases[4].status | Pasar a "done" cuando commit de prueba haya generado INTERACCIONES_*.json y .md. | 5.1 |

---

## Orden sugerido de aplicación

1. Fase 1: 1.1 → 1.2  
2. Fase 2: 2.1 → 2.2  
3. Fase 3: 3.1 → 3.2  
4. Fase 4: 4.1 → 4.2  
5. Fase 5: 5.1 → 5.2  

## Resumen por archivo

| Archivo | Ítems |
|---------|-------|
| docs/features/auditoria-interacciones-entidades/interaction-report-schema.md | 1.1 |
| docs/features/auditoria-interacciones-entidades/interaction-report-schema.json | 1.2 |
| docs/features/auditoria-interacciones-entidades/data-source-contract.md | 2.1 |
| docs/features/auditoria-interacciones-entidades/spec.md | 2.2 |
| scripts/audits/Generate-InteractionsReport.ps1 | 3.1 |
| SddIA/agents/cumulo.paths.json | 3.2 |
| .husky/pre-commit o .git/hooks/pre-commit | 4.1 |
| docs/features/auditoria-interacciones-entidades/plan.md | 4.2 |
| docs/features/auditoria-interacciones-entidades/validacion.json | 5.1 |
| docs/features/auditoria-interacciones-entidades/plan.json | 5.2 |
