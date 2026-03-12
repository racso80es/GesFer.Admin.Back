# SPEC: Refactorización Arquitectura Dual → Frontmatter en SddIA

## 1. Información General

| Campo | Detalle |
| :--- | :--- |
| **ID de Especificación** | SPEC-REF-2026-002 |
| **Rama Relacionada** | feat/refactorization-arquitectura-frontmatter |
| **Estado** | Draft |
| **Responsable** | Spec Architect / Tekton |
| **Ruta (Cúmulo)** | paths.featurePath/refactorization-arquitectura-frontmatter |

## 2. Propósito y Contexto

### 2.1. Objetivo (Goal)

Optimizar la arquitectura dual actual (spec.md + spec.json) del ecosistema SddIA mediante dos fases: (1) mitigación inmediata reduciendo duplicidad y optimizando I/O; (2) migración planificada hacia **Markdown con Frontmatter YAML** (Alternativa A del análisis). El resultado final será un único archivo `.md` por entidad con metadatos estructurados en frontmatter y contenido legible en el cuerpo, eliminando la fragmentación de contexto LLM y el doble I/O.

### 2.2. Alcance (Scope)

**Incluido — Fase 1 (Mitigación):**
- Auditar los 84 pares duales para identificar campos duplicados (title, description, purpose).
- Limpiar spec.json: eliminar campos textuales que ya existan en spec.md; spec.json solo IDs, arrays, enums, referencias y metadatos de clasificación.
- Optimizar scripts Rust para lazy-loading de spec.md (cargar MD solo cuando se necesite contexto completo).
- Medir consumo de tokens LLM antes/después como línea base.

**Incluido — Fase 2 (Migración):**
- **1. Actualización de normas SddIA (PRIMERO):** constitution.json (L7), entidades-dominio-ecosistema-sddia.md, 9 contratos, interaction-triggers.md, paths-via-cumulo.md y demás normas que referencien la estructura de entidades. Definir y mandatar el nuevo formato frontmatter antes de cualquier migración.
- **2. Script Rust de conversión** (puntual, en carpeta tarea o scripts/): spec.json + spec.md → .md con frontmatter YAML.
- **3. Migración en único PR (sin coexistencia):** 84 entidades + contratos duales. Toda entidad SddIA ha de respetar la nueva estructura. Excepción: karma2-token (JSON-only por contrato).
- **4. Validación:** Esquema YAML nativo en acción validate. Actualizar check sddia_md_json_parity.
- **5. Actualización** de reglas Cursor (.cursor/rules) y AGENTS.md.
- **6. Documentación de proceso (C8):** Los artefactos generados por procesos (objectives, spec, clarify, plan, implementation, validacion) en paths.featurePath y paths.fixPath han de respetar el nuevo patrón: un único .md con frontmatter por artefacto, en lugar de pares .md + .json. Actualizar process_interface (Cúmulo) y acciones spec, clarify, planning, implementation.

**Fuera de alcance:**
- Alternativas B (JSON unificado) y C (SQLite): descartadas por el análisis.
- Cambiar la implementación de cápsulas (scripts/, binarios) salvo los que consumen spec.md/spec.json.

## 3. Arquitectura y Diseño Técnico

### 3.1. Formato objetivo (Frontmatter YAML + MD)

```markdown
---
id: "e98e4d2a-1c3f-4e56-9a2b-8f7d6c5b4a12"
category: "Arquitectura de Software"
interested_agents: ["architect", "tekton-developer"]
security_model:
  required_token: "Karma2Token"
---
# Título del Patrón
Contenido en lenguaje natural...
```

### 3.2. Componentes afectados

| Zona | Alcance | Cambio |
| :--- | :--- | :--- |
| **constitution.json** | L7_DOMAIN_ENTITIES | **PRIMERO.** Enmienda: estructura frontmatter+MD como canónica; excepción tokens JSON-only. |
| **entidades-dominio-ecosistema-sddia.md** | Estructura obligatoria | **PRIMERO.** Frontmatter+MD como estructura canónica; excepción tokens. |
| **9 contratos** | actions, process, patterns, principles, security, skills, tools, templates, tokens | **PRIMERO.** Nuevo esquema frontmatter YAML. Incluidos en migración Fase 2. |
| **interaction-triggers.md, paths-via-cumulo.md** | Normas SddIA | **PRIMERO.** Actualizar referencias a estructura de entidades. |
| **validate** | sddia_md_json_parity | Esquema YAML nativo (post-migración). |
| **Scripts Rust** | paths.skillsRustPath, paths.toolsRustPath | Fase 1: lazy-loading spec.md. Fase 2: parsear frontmatter. |
| **Reglas Cursor** | .cursor/rules/*.mdc | Actualizar referencias a estructura de entidades. |
| **AGENTS.md** | Ley L7, referencias | Alinear con nueva estructura. |
| **process_interface** | Cúmulo (cumulo.json) | Artefactos de tarea: .md con frontmatter en lugar de .md + .json. |
| **Acciones spec, clarify, planning, implementation** | paths.actionsPath | Generar artefactos en formato frontmatter. |

### 3.3. Orden de ejecución Fase 2

1. **Actualizar normas SddIA** (constitution, entidades-dominio, contratos, interaction-triggers, paths-via-cumulo).
2. Script Rust de conversión (puntual).
3. Migración en único PR: 84 entidades + contratos duales. Orden sugerido: Patrones → Principios → Skills/Tools → Actions/Process → Security → Templates. Tokens: karma2-token se mantiene JSON-only (excepción).

## 4. Requisitos de Seguridad

- **Token Karma2Token:** Todas las entidades de dominio operan bajo contexto de Token; la herramienta de conversión y los scripts deben respetar paths.tokensPath.
- **Validación de esquema:** El frontmatter YAML debe validarse contra esquema YAML nativo (acción validate) para prevenir inyecciones.
- **Auditoría:** Cambios rastreados en carpeta de tarea (Cúmulo), paths.evolutionPath + paths.evolutionLogFile al cierre.

## 5. Criterios de Aceptación

**Fase 1:**
- [ ] Auditoría RV-001 completada: inventario de campos duplicados en 84 pares.
- [ ] RV-002 ejecutado: spec.json sin title, description, purpose duplicados del MD.
- [ ] Scripts Rust con lazy-loading de spec.md (RV-003).
- [ ] Línea base de tokens LLM medida (RV-004).

**Fase 2:**
- [ ] **1. Normas SddIA actualizadas** (constitution, entidades-dominio, contratos, interaction-triggers, paths-via-cumulo) — PRIMERO.
- [ ] Script Rust de conversión dual → frontmatter (puntual).
- [ ] Check validate actualizado para esquema YAML nativo.
- [ ] 84 entidades + contratos duales migrados en único PR (sin coexistencia).
- [ ] karma2-token mantenido como excepción JSON-only.
- [ ] Reglas Cursor y AGENTS.md actualizadas.
- [ ] **Documentación de proceso (C8):** process_interface y acciones actualizadas para generar artefactos en frontmatter; tareas nuevas producen objectives.md, spec.md, clarify.md, plan.md, implementation.md, validacion.md con frontmatter.
- [ ] Evolution Log actualizado al cierre.

## 6. Trazabilidad

- **Origen:** docs/features/refactorization-arquitectura-frontmatter/objectives.md
- **Análisis:** docs/analysis/analisis-arquitectura-dual.md, revision-arquitectura-dual-2026-03-12.md
- **Proceso:** paths.processPath/refactorization/
- **Clarificaciones:** clarify.md (resueltas)
