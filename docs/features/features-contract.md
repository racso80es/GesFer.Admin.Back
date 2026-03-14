---
contract_id: features-documentation
norm_ref: SddIA/norms/features-documentation-pattern.md
paths_ref: paths.featurePath (Cúmulo)
format: .md con frontmatter YAML + cuerpo Markdown
no_json: true
---

# Contrato: Documentación de features (paths.featurePath)

**Alcance:** paths.featurePath (docs/features/) y paths.fixPath (docs/bugs/). Toda documentación de tarea debe cumplir este contrato.

## Principio

**Un solo fichero `.md` por acción**, con frontmatter YAML (metadatos) + cuerpo Markdown. **No se usan ficheros `.json` separados.** Mismo patrón que skills, tools y entidades de dominio SddIA.

## Artefactos por acción

### objectives.md

| Campo frontmatter | Tipo | Obligatorio |
|------------------|------|-------------|
| feature_name | string | Sí |
| created | string (YYYY-MM-DD) | Sí |
| process | string (feature, bug-fix, etc.) | Sí |

**Cuerpo:** Objetivo principal, alcance, ley aplicada, resumen del proceso.

---

### spec.md

| Campo frontmatter | Tipo | Obligatorio |
|------------------|------|-------------|
| feature_name | string | Sí |
| created | string | Sí |
| base | string o array (objectives.md, analysis.md, etc.) | Opc. |
| scope | object (in_scope, out_scope) | Opc. |
| functional_requirements | array | Opc. |
| non_functional_requirements | array | Opc. |
| touchpoints | array | Opc. |
| validation_criteria | array | Opc. |

**Cuerpo:** Especificación técnica completa (objetivo, alcance, RF, RNF, arquitectura, touchpoints, criterios de validación).

---

### clarify.md

| Campo frontmatter | Tipo | Obligatorio |
|------------------|------|-------------|
| feature_name | string | Sí |
| created | string | Sí |
| purpose | string | Opc. |
| decisions | array | Opc. |

**Cuerpo:** Clarificaciones, opciones evaluadas, decisiones cerradas.

---

### plan.md

| Campo frontmatter | Tipo | Obligatorio |
|------------------|------|-------------|
| feature_name | string | Sí |
| created | string | Sí |
| phases | array | Opc. |
| tasks | array | Opc. |

**Cuerpo:** Plan de implementación, fases, tareas técnicas, orden sugerido.

---

### implementation.md

| Campo frontmatter | Tipo | Obligatorio |
|------------------|------|-------------|
| feature_name | string | Sí |
| created | string | Sí |
| items | array (id, action, path, location, proposal, dependencies) | Opc. |

**Cuerpo:** Touchpoints detallados, ítems de implementación con ruta, ubicación y propuesta.

---

### execution.md

| Campo frontmatter | Tipo | Obligatorio |
|------------------|------|-------------|
| feature_name | string | Sí |
| created | string | Sí |
| items_applied | array (id, path, action, status, message, timestamp) | Opc. |

**Cuerpo:** Registro de ejecución, ítems aplicados con estado (OK/Error).

---

### validacion.md

| Campo frontmatter | Tipo | Obligatorio |
|------------------|------|-------------|
| feature_name | string | Sí |
| branch | string | Sí |
| base_branch | string | Sí |
| global | string (pass \| fail) | Sí |
| blocking | boolean | Opc. |
| checks | array (name, result, message) | Sí |
| git_changes | object (files_added, files_modified, files_deleted) | Sí |

**Cuerpo:** Informe de validación, resumen de checks, detalle de git_changes.

---

### finalize.md (opcional)

| Campo frontmatter | Tipo | Obligatorio |
|------------------|------|-------------|
| feature_name | string | Sí |
| pr_url | string | Opc. |
| branch | string | Opc. |
| timestamp | string | Opc. |

**Cuerpo:** Resumen de cierre, enlace al PR.

## Nomenclatura

- **spec.md** (no SPEC-*.md salvo convención explícita del proyecto)
- **validacion.md** (no validacion.json)
- **plan.md** (no plan.json ni PLAN-*.md salvo convención)

## Validación

- El check opcional `sddia_frontmatter_valid` (acción validate) valida frontmatter YAML en los `.md` de paths.featurePath.
- Norma canónica: **SddIA/norms/features-documentation-pattern.md**.

---
*Contrato de documentación de features. Consumido por proceso feature, acciones y agentes.*
