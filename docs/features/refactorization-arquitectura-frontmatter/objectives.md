# Refactorización: Arquitectura Dual (MD + JSON) → Frontmatter en SddIA

**Rama:** feat/refactorization-arquitectura-frontmatter  
**Ruta (Cúmulo):** paths.featurePath/refactorization-arquitectura-frontmatter  
**Proceso:** paths.processPath/refactorization/

---

## Objetivo

Refactorizar el sistema SddIA para optimizar la arquitectura dual actual (spec.md + spec.json) según el análisis registrado en `docs/analysis/`. La dirección es migrar hacia **Markdown con Frontmatter YAML** (Alternativa A), ejecutando primero una fase de mitigación inmediata y luego la migración planificada.

---

## Alcance

### Fase 1 — Mitigación inmediata (sin cambio arquitectónico)
- Auditar los 84 pares duales y eliminar campos duplicados del JSON que ya existan en el MD.
- El `spec.json` debe contener exclusivamente: IDs, arrays, enums, referencias y metadatos de clasificación.
- Optimizar scripts Rust para lazy-loading del spec.md.
- Medir tokens consumidos antes/después como línea base.

### Fase 2 — Migración a Frontmatter (proceso refactorization)
- Rama: `feat/refactorization-arquitectura-frontmatter`.
- **1. Actualización de normas SddIA (PRIMERO):** constitution.json (L7), entidades-dominio-ecosistema-sddia.md, 9 contratos, interaction-triggers.md, paths-via-cumulo.md y demás normas que referencien la estructura de entidades. Definir y mandatar el nuevo formato frontmatter.
- **2. Script Rust de conversión** (puntual): spec.json + spec.md → .md con frontmatter YAML.
- **3. Migración en único PR** (sin coexistencia): 84 entidades + contratos duales. Toda entidad SddIA ha de respetar la nueva estructura. Excepción: karma2-token (JSON-only).
- **4. Validación:** Esquema YAML nativo en acción validate.
- **5. Actualizar** reglas Cursor y AGENTS.md.
- **6. Documentación de proceso (C8):** Los artefactos generados por procesos (objectives, spec, clarify, plan, implementation, validacion) han de respetar el nuevo patrón frontmatter. Actualizar process_interface y acciones.

### Entidades afectadas (84 pares duales)
| Tipo | Cantidad |
|------|----------|
| Acciones | 8 |
| Procesos | 7 |
| Patrones | 11 |
| Principios | 21 |
| Seguridad | 20 |
| Skills | 11 |
| Herramientas | 5 |
| Templates | 1 |
| **Total** | **84** |

---

## Ley aplicada

- **L7_DOMAIN_ENTITIES (constitution.json):** Las entidades de dominio **deben** tener spec.md + spec.json y mantener sincronía. La migración requiere enmienda formal.
- **SddIA/norms/entidades-dominio-ecosistema-sddia.md:** Estructura obligatoria y paridad MD/JSON.
- **Acción validate → sddia_md_json_parity:** Check automatizado de paridad; debe actualizarse para el nuevo esquema.
- **9 contratos *-contract.json:** Definen esquema de spec.json por tipo de entidad; deben enmendarse para frontmatter.

---

## Análisis de situación actual

### Fuentes canónicas
- **Análisis original:** `docs/analysis/analisis-arquitectura-dual.md` — Diagnóstico de pros/contras, alternativas (A, B, C), recomendación hacia Alternativa A.
- **Revisión auditora:** `docs/analysis/revision-arquitectura-dual-2026-03-12.md` — Verificación contra estado real, restricciones normativas, plan en dos fases.

### Hallazgos confirmados
1. **Duplicidad real:** Campos como `title`, `description`, `purpose` aparecen en spec.json y en encabezados/secciones del spec.md.
2. **Fragmentación de contexto LLM:** Cargar un patrón completo requiere inyectar ambos archivos con metadatos superpuestos.
3. **Doble I/O:** Scripts Rust necesitan contexto completo de una entidad (dos lecturas).
4. **Separación de responsabilidades:** Funciona; scripts indexan solo JSON para filtrado rápido.

### Restricciones normativas no contempladas en auditoría original
| Artefacto | Requisito |
|-----------|-----------|
| constitution.json → L7_DOMAIN_ENTITIES | spec.md + spec.json obligatorios; sincronía |
| entidades-dominio-ecosistema-sddia.md | Estructura y paridad obligatorias |
| validate → sddia_md_json_parity | Check automatizado |
| 9 contratos *-contract.json | Esquemas por tipo de entidad |

**Implicación:** Cualquier migración requiere enmendar constitución, norma de entidades, 9 contratos y check de validación.

### Precedente de flexibilidad
- `tokens-contract.json` ya define `spec.md` como opcional (karma2-token solo spec.json). El ecosistema admite excepciones documentadas.

---

## Acciones de seguimiento (tracking)

| ID | Acción | Prioridad | Estado |
|----|--------|-----------|--------|
| RV-001 | Auditar duplicidad de campos en los 84 pares (title, description, purpose) | Alta | Pendiente |
| RV-002 | Limpiar spec.json: eliminar campos textuales duplicados del MD | Alta | Pendiente |
| RV-003 | Optimizar scripts Rust para lazy-loading de spec.md | Media | Pendiente |
| RV-004 | Medir consumo de tokens LLM (línea base pre-optimización) | Media | Pendiente |
| RV-005 | Evaluar viabilidad de enmienda a L7 y contratos para Fase 2 | Media | Pendiente |
| RV-006 | Diseñar herramienta Rust de conversión dual → frontmatter | Baja | Pendiente |
| RV-007 | Crear proceso refactorization formal para la migración | Baja | En curso |

---

## Resumen del proceso (refactorization)

| Fase | Acción |
|------|--------|
| 0 | Rama feat/refactorization-arquitectura-frontmatter (skill iniciar-rama) |
| 1 | Documentación objetivos (este documento) y análisis de situación |
| 2 | Spec (spec.md, spec.json en carpeta de la tarea / Cúmulo) |
| 3–6 | Clarificación, plan, implementación (doc), ejecución |
| 7 | Validación (validacion.json) |
| 8 | Cierre y PR (skill finalizar-git) |

---

## Referencias

- **Análisis:** docs/analysis/analisis-arquitectura-dual.md
- **Revisión:** docs/analysis/revision-arquitectura-dual-2026-03-12.md, revision-arquitectura-dual-2026-03-12.json
- **Cúmulo:** SddIA/agents/cumulo.json → pathsContract (cumulo.paths.json)
- **Proceso:** SddIA/process/refactorization/
- **Norma entidades:** SddIA/norms/entidades-dominio-ecosistema-sddia.md
