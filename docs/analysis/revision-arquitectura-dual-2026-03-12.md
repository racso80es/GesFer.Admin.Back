# Revisión: Análisis de Arquitectura Dual (MD + JSON) en SddIA

> **Fecha:** 2026-03-12
> **Documento auditado:** [`docs/analysis/analisis-arquitectura-dual.md`](./analisis-arquitectura-dual.md)
> **Revisado por:** Agente Auditor (rol AUDITOR, modelo Claude Opus 4.6)
> **Seguimiento:** [`revision-arquitectura-dual-2026-03-12.json`](./revision-arquitectura-dual-2026-03-12.json)

---

## 1. Propósito de esta revisión

Evaluar la calidad, precisión y viabilidad de las conclusiones del documento `analisis-arquitectura-dual.md`, contrastándolas con el estado real del ecosistema SddIA, y registrar acciones de seguimiento con estado trazable.

---

## 2. Verificación de datos contra el estado real

### 2.1. Escala real de la arquitectura dual

| Tipo de entidad | Pares duales (spec.md + spec.json) |
|---|---|
| Acciones | 8 |
| Procesos | 7 |
| Patrones | 11 |
| Principios | 21 |
| Seguridad | 20 |
| Skills | 11 |
| Herramientas | 5 |
| Templates | 1 |
| **Total** | **84** |

- **1 entidad** solo con `spec.json` (karma2-token; permitido por `tokens-contract.json`).
- **9 contratos** tipo `*-contract.json`, de los cuales 8 tienen también `*-contract.md`.
- **Total de archivos gestionados por la dualidad:** ~185 (168 specs + 17 contratos).

### 2.2. Restricciones normativas no contempladas por la auditoría

La auditoría original no menciona que la dualidad está **codificada como ley** en el ecosistema:

| Artefacto normativo | Requisito |
|---|---|
| `constitution.json` → ley L7_DOMAIN_ENTITIES | Las entidades de dominio **deben** tener `spec.md + spec.json` y mantener sincronía. |
| `SddIA/norms/entidades-dominio-ecosistema-sddia.md` | Estructura obligatoria y paridad MD/JSON. |
| Acción `validate` → check `sddia_md_json_parity` | Validación automatizada de paridad. |
| 9 contratos `*-contract.json` | Definen esquema de `spec.json` por tipo de entidad. |

**Implicación:** Cualquier migración requiere enmendar la constitución, la norma de entidades, los 9 contratos y el check de validación.

---

## 3. Evaluación de la auditoría

### 3.1. Calidad general

| Criterio | Valoración |
|---|---|
| Identificación de pros/contras | **Alta** — Los 4 pros y 4 contras son correctos y verificables. |
| Alternativas propuestas | **Adecuadas** — A (frontmatter) es la mejor opción; B y C correctamente descartadas. |
| Profundidad técnica | **Media-Alta** — Cubre tokenización, I/O, DevEx y parseo. |
| Cobertura normativa | **Baja** — No contempla L7, contratos ni norma de entidades. |
| Estimación de impacto de migración | **Insuficiente** — Subestima el coste normativo y de refactorización de contratos. |

### 3.2. Hallazgos confirmados

1. **Duplicidad real de datos:** Campos como `title`, `description` y `purpose` aparecen tanto en `spec.json` como en encabezados/secciones del `spec.md` en múltiples entidades.
2. **Fragmentación de contexto LLM:** Confirmada. Cargar un patrón completo requiere inyectar ambos archivos con metadatos superpuestos.
3. **Doble I/O:** Verificado en scripts Rust que necesitan contexto completo de una entidad.
4. **Separación de responsabilidades:** Funciona correctamente; los scripts de Rust indexan solo JSON para filtrado rápido.

### 3.3. Hallazgos adicionales (no presentes en la auditoría)

1. **Contratos duales:** Los propios contratos (`*-contract.json` / `*-contract.md`) replican el patrón dual, añadiendo 17 archivos más al inventario.
2. **Ley constitucional:** La migración no es solo técnica sino normativa; requiere proceso formal de enmienda.
3. **Tokens como excepción:** `tokens-contract.json` ya define `spec.md` como opcional, demostrando que el ecosistema ya tiene precedente de flexibilidad en la dualidad.

---

## 4. Recomendación revisada

### Coincidencias con la auditoría

- La **Alternativa A (Frontmatter YAML + MD)** es la dirección correcta.
- Las alternativas B (JSON unificado) y C (SQLite) son correctamente descartables.
- La mitigación inmediata (reducir duplicidad, optimizar scripts) es prioritaria.

### Diferencias con la auditoría

| Aspecto | Auditoría original | Esta revisión |
|---|---|---|
| Coste de migración | Menciona actualizar scripts Rust | Incluye además: constitución, 9 contratos, norma de entidades, check validate, reglas Cursor, AGENTS.md |
| Enfoque | Migración directa | Dos fases: mitigación inmediata + migración planificada |
| Precedente de flexibilidad | No mencionado | `tokens-contract.json` ya admite `spec.md` opcional |

### Plan propuesto en dos fases

**Fase 1 — Mitigación inmediata (sin cambio arquitectónico):**
- Auditar los 84 pares y eliminar campos duplicados del JSON que ya existan en el MD.
- El `spec.json` debe contener exclusivamente: IDs, arrays, enums, referencias y metadatos de clasificación.
- Optimizar scripts Rust para lazy-loading del MD.
- Medir tokens consumidos antes/después como línea base.

**Fase 2 — Migración a Frontmatter (proceso `refactorization`):**
- Rama: `feat/refactorization-arquitectura-frontmatter`.
- Crear herramienta Rust de conversión automática (spec.json + spec.md → .md con frontmatter YAML).
- Enmendar `constitution.json` (ley L7), contratos, norma de entidades, check validate.
- Migrar entidad por entidad, validando contra nuevo esquema.
- Actualizar reglas de difusión Cursor y AGENTS.md.

---

## 5. Acciones de seguimiento

| ID | Acción | Origen | Prioridad | Estado |
|---|---|---|---|---|
| `RV-001` | Auditar duplicidad de campos en los 84 pares (title, description, purpose) | Auditoría §5 + Revisión §4 | Alta | Pendiente |
| `RV-002` | Limpiar spec.json: eliminar campos textuales duplicados del MD | Auditoría §5 | Alta | Pendiente |
| `RV-003` | Optimizar scripts Rust para lazy-loading de spec.md | Auditoría §5 | Media | Pendiente |
| `RV-004` | Medir consumo de tokens LLM (línea base pre-optimización) | Revisión §4 | Media | Pendiente |
| `RV-005` | Evaluar viabilidad de enmienda a L7 y contratos para Fase 2 | Revisión §3.2 | Media | Pendiente |
| `RV-006` | Diseñar herramienta Rust de conversión dual → frontmatter | Revisión §4 | Baja | Pendiente |
| `RV-007` | Crear proceso `refactorization` formal para la migración | Revisión §4 | Baja | Pendiente |

---

## 6. Resumen ejecutivo

| Dimensión | Resultado |
|---|---|
| **Calidad de la auditoría** | Alta en diagnóstico técnico; incompleta en cobertura normativa. |
| **Diagnóstico principal** | Correcto: la dualidad funciona pero no escala eficientemente. |
| **Recomendación** | Acertada (Alternativa A). Falta planificación del coste normativo. |
| **Impacto real de migración** | Alto: 84 entidades, 9 contratos, constitución, normas, scripts, reglas Cursor. |
| **Urgencia** | Media. Fase 1 ejecutable inmediatamente; Fase 2 requiere proceso formal. |
| **Riesgo de inacción** | Deuda técnica creciente proporcional a nuevas entidades. |
