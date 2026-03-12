# PLAN: Refactorización Arquitectura Dual → Frontmatter en SddIA

**Proceso:** refactorization  
**Ruta (Cúmulo):** paths.featurePath/refactorization-arquitectura-frontmatter/  
**Especificación:** spec.md | **Clarificación:** clarify.md  

---

## 1. Objetivos del plan

- **Fase 1:** Mitigación inmediata: auditar duplicidad (RV-001), limpiar spec.json (RV-002), optimizar scripts Rust lazy-loading (RV-003), medir tokens LLM (RV-004). Fase 1 completa antes de Fase 2.
- **Fase 2:** Migración a frontmatter. **PRIMERO:** actualizar normas SddIA sobre el nuevo formato. Luego: script conversión, migración único PR (84 entidades + contratos), validate, Cursor, AGENTS.
- **Excepción:** karma2-token se mantiene JSON-only.
- **Restricciones:** Migración en único PR sin coexistencia; script puntual (no tool en toolCapsules); esquema YAML nativo para validación.
- **C8:** La documentación generada en un proceso (plan, spec, clarify, implementation, objectives, validacion) ha de respetar el nuevo patrón frontmatter. process_interface y acciones actualizadas.

---

## 2. Fases y tareas técnicas

### Fase 1 – Mitigación (secuencial, antes de Fase 2)

| Id | Tarea | Criterio |
|----|-------|----------|
| 1.1 | [REF-FM] RV-001: Auditar duplicidad | Inventariar en 84 pares los campos title, description, purpose duplicados entre spec.json y spec.md. Salida: documento o JSON con inventario. |
| 1.2 | [REF-FM] RV-002: Limpiar spec.json | Eliminar de cada spec.json los campos textuales que ya existan en spec.md. spec.json solo: IDs, arrays, enums, referencias, metadatos clasificación. |
| 1.3 | [REF-FM] RV-003: Optimizar scripts Rust | Lazy-loading de spec.md: cargar MD solo cuando se necesite contexto completo. Revisar paths.skillsRustPath, paths.toolsRustPath. |
| 1.4 | [REF-FM] RV-004: Medir tokens LLM | Línea base pre-optimización: medir consumo tokens al cargar entidades duales. Documentar resultado. |

### Fase 2.1 – Actualización normas SddIA (PRIMERO)

| Id | Tarea | Criterio |
|----|-------|----------|
| 2.1.1 | [REF-FM] constitution.json L7 | Enmienda L7_DOMAIN_ENTITIES: estructura frontmatter+MD como canónica; excepción tokens JSON-only (karma2-token). |
| 2.1.2 | [REF-FM] entidades-dominio-ecosistema-sddia.md | Actualizar: frontmatter+MD estructura canónica; excepción tokens; referencias a validate (esquema YAML). |
| 2.1.3 | [REF-FM] 9 contratos *-contract | actions, process, patterns, principles, security, skills, tools, templates, tokens: nuevo esquema frontmatter YAML; spec.json deprecado o reemplazado por frontmatter. |
| 2.1.4 | [REF-FM] interaction-triggers.md | Actualizar referencias: spec.md + spec.json → .md con frontmatter (skills, actions, process). |
| 2.1.5 | [REF-FM] paths-via-cumulo.md | Si aplica: referencias a estructura de entidades. |
| 2.1.6 | [REF-FM] patterns-in-planning-implementation-execution.md | spec.md/spec.json → .md con frontmatter. |
| 2.1.7 | [REF-FM] agents-principles-contract.md | spec.md, spec.json → frontmatter. |
| 2.1.8 | [REF-FM] commands-via-skills-or-tools.md, git-via-skills-or-process.md | Actualizar referencias spec.json → frontmatter. |
| 2.1.9 | [REF-FM] norms/README.md | Actualizar descripción estructura entidades. |

### Fase 2.2 – Script conversión

| Id | Tarea | Criterio |
|----|-------|----------|
| 2.2.1 | [REF-FM] Crear script Rust conversión | Ubicación: carpeta tarea o scripts/. Input: spec.json + spec.md. Output: .md con frontmatter YAML. Librería: gray_matter o similar. |
| 2.2.2 | [REF-FM] Probar conversión | Validar con al menos 3 entidades (patrón, skill, action) antes de migración masiva. |

### Fase 2.3 – Migración único PR

| Id | Tarea | Criterio |
|----|-------|----------|
| 2.3.1 | [REF-FM] Migrar entidades | Orden: Patrones (11) → Principios (21) → Skills (11) + Tools (5) → Actions (8) + Process (7) → Security (20) → Templates (1). karma2-token: no migrar (excepción). |
| 2.3.2 | [REF-FM] Migrar contratos duales | 9 contratos *-contract.md + *-contract.json → frontmatter. |
| 2.3.3 | [REF-FM] Eliminar spec.json | Tras conversión, eliminar spec.json de cada entidad migrada. |

### Fase 2.4 – Validación y difusión

| Id | Tarea | Criterio |
|----|-------|----------|
| 2.4.1 | [REF-FM] Acción validate | Actualizar check sddia_md_json_parity: validar frontmatter con esquema YAML nativo. |
| 2.4.2 | [REF-FM] Reglas Cursor | .cursor/rules/*.mdc: referencias a estructura frontmatter. |
| 2.4.3 | [REF-FM] AGENTS.md | Ley L7, referencias a entidades: alinear con frontmatter. |
| 2.4.4 | [REF-FM] Evolution Log | paths.evolutionPath + paths.evolutionLogFile: entrada al cierre. |
| 2.4.5 | [REF-FM] C8: Documentación de proceso | Actualizar process_interface (Cúmulo): artefactos .md con frontmatter en lugar de .md + .json. Actualizar acciones spec, clarify, planning, implementation para generar frontmatter. Las tareas nuevas producen objectives.md, spec.md, clarify.md, plan.md, implementation.md, validacion.md con frontmatter. |

---

## 3. Orden de ejecución recomendado

1. **Fase 1 completa** (1.1 → 1.2 → 1.3 → 1.4).
2. **Fase 2.1** (normas): 2.1.1 → 2.1.2 → 2.1.3 → 2.1.4–2.1.9 (2.1.3 puede requerir iteración por contrato).
3. **Fase 2.2** (script): 2.2.1 → 2.2.2.
4. **Fase 2.3** (migración): 2.3.1 → 2.3.2 → 2.3.3 en único PR.
5. **Fase 2.4** (validación): 2.4.1 → 2.4.2 → 2.4.3 → 2.4.4.

---

## 4. Verificación

- **Fase 1:** Inventario RV-001 documentado; spec.json sin title/description/purpose duplicados; scripts con lazy-loading; métrica tokens registrada.
- **Fase 2:** Normas actualizadas; script convierte correctamente; 84 entidades + contratos en formato frontmatter; karma2-token intacto (JSON-only); validate con esquema YAML; Cursor y AGENTS alineados; process_interface y acciones actualizadas (C8); Evolution Log actualizado.
- **Build:** dotnet build sin errores (si se toca código Rust).
- **No coexistencia:** Tras migración, no quedan pares duales salvo karma2-token.

---

## 5. Alcance cerrado

- **In scope:** Fase 1 (4 tareas); Fase 2 (normas, script, 84 entidades, 9 contratos, validate, Cursor, AGENTS).
- **Out of scope:** karma2-token migración; alternativas B/C; cambiar implementación cápsulas salvo consumidores spec; tool en toolCapsules (script puntual).

---

*Plan generado a partir de spec.md y clarify.md. Siguiente fase: implementation (doc).*
