# Implementación: Refactorización Arquitectura Dual → Frontmatter en SddIA

**Proceso:** refactorization  
**Ruta (Cúmulo):** paths.featurePath/refactorization-arquitectura-frontmatter/  
**Plan:** plan.md | **SPEC:** spec.md | **Clarify:** clarify.md  

---

## Ítems de implementación

Cada ítem: **Id** | **Acción** | **Ruta** | **Ubicación** | **Propuesta** | **Dependencias**

### Fase 1 – Mitigación

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| IMPL-1.1 | Crear | docs/features/refactorization-arquitectura-frontmatter/inventario-duplicidad.json | Nuevo archivo | Generar inventario: para cada uno de los 84 pares (SddIA/actions, process, patterns, principles, security, skills, tools, templates, tokens), listar campos title, description, purpose presentes en spec.json que también existen en spec.md. Estructura: `{ "entity_path": [...duplicated_fields] }`. | — |
| IMPL-1.2 | Modificar | SddIA/**/spec.json (84 archivos) | Campos title, description, purpose | Eliminar de cada spec.json los campos textuales que ya existan en spec.md. Mantener solo: IDs, arrays, enums, referencias, metadatos clasificación. Aplicar según inventario IMPL-1.1. Rutas: SddIA/actions/*/spec.json, SddIA/process/*/spec.json, SddIA/patterns/*/spec.json, SddIA/principles/*/spec.json, SddIA/security/*/spec.json, SddIA/skills/*/spec.json, SddIA/tools/*/spec.json, SddIA/templates/*/spec.json. Excluir SddIA/Tokens/karma2-token/spec.json. | IMPL-1.1 |
| IMPL-1.3 | Modificar | scripts/skills-rs/src/*.rs, scripts/tools-rs/src/*.rs | Lógica de carga de spec | Implementar lazy-loading: cargar spec.md solo cuando se necesite contexto completo de la entidad. Los binarios que indexan o filtran deben leer solo spec.json (o frontmatter tras migración); el MD se carga bajo demanda. Revisar: iniciar_rama.rs, invoke_command.rs, push_and_create_pr.rs, merge_to_master_cleanup.rs, start_api.rs, etc. | — |
| IMPL-1.4 | Crear | docs/features/refactorization-arquitectura-frontmatter/metrica-tokens.json | Nuevo archivo | Documentar línea base: medir consumo tokens LLM al cargar N entidades (ej. 5 patrones + 3 skills) en formato dual. Incluir: modelo, tokens_input, tokens_output, entidades_cargadas. | — |

### Fase 2.1 – Actualización normas SddIA (PRIMERO)

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| IMPL-2.1.1 | Modificar | SddIA/constitution.json | universal_laws → L7_DOMAIN_ENTITIES | Enmienda: "Entities must respect structure: **Markdown with Frontmatter YAML** (single .md per entity). Exception: tokens may be JSON-only per tokens-contract. Norm: SddIA/norms/entidades-dominio-ecosistema-sddia.md." | — |
| IMPL-2.1.2 | Modificar | SddIA/norms/entidades-dominio-ecosistema-sddia.md | Estructura obligatoria, §2 | Actualizar: "Estructura canónica: archivo .md con frontmatter YAML (metadatos) + cuerpo Markdown. Excepción: tokens (paths.tokensPath) pueden ser JSON-only por tokens-contract. Validación: esquema YAML nativo en acción validate." | — |
| IMPL-2.1.3 | Modificar | SddIA/actions/actions-contract.json, SddIA/process/process-contract.json, SddIA/patterns/patterns-contract.json, SddIA/principles/principles-contract.json, SddIA/security/security-contract.json, SddIA/skills/skills-contract.json, SddIA/tools/tools-contract.json, SddIA/templates/templates-contract.json, SddIA/Tokens/tokens-contract.json | Esquema required_files | Nuevo esquema: required_files: [".md con frontmatter YAML"]. Eliminar o deprecar spec.json como obligatorio. tokens-contract: mantener spec.json opcional para karma2-token. | — |
| IMPL-2.1.4 | Modificar | SddIA/norms/interaction-triggers.md | Referencias skills, actions, process | Sustituir "spec.md, spec.json" por ".md con frontmatter" en §#Skill, §#Action, §#Process. Ej.: "paths.skillsDefinitionPath/<skill-id>/ (archivo .md con frontmatter)". | — |
| IMPL-2.1.5 | Modificar | SddIA/norms/paths-via-cumulo.md | Si aplica | Revisar referencias a estructura spec.md/spec.json; actualizar a frontmatter. | — |
| IMPL-2.1.6 | Modificar | SddIA/norms/patterns-in-planning-implementation-execution.md | Referencias spec | Sustituir "spec.md, spec.json" por ".md con frontmatter" en referencias a patrones. | — |
| IMPL-2.1.7 | Modificar | SddIA/norms/agents-principles-contract.md | Referencias principles | "spec.md, spec.json" → ".md con frontmatter". | — |
| IMPL-2.1.8 | Modificar | SddIA/norms/commands-via-skills-or-tools.md, SddIA/norms/git-via-skills-or-process.md | Referencias spec.json | "spec.json" → "archivo .md con frontmatter" en descripción de skills/tools. | — |
| IMPL-2.1.9 | Modificar | SddIA/norms/README.md | Descripción entidades | Actualizar: estructura de entidades = .md con frontmatter YAML; excepción tokens JSON-only. | — |

### Fase 2.2 – Script conversión

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| IMPL-2.2.1 | Crear | docs/features/refactorization-arquitectura-frontmatter/dual-to-frontmatter/ (o scripts/dual-to-frontmatter/) | Nuevo proyecto Rust | Script binario: input: ruta a carpeta con spec.json + spec.md. Output: archivo .md con frontmatter YAML (metadatos) + cuerpo (contenido de spec.md). Usar gray_matter o similar. Parámetros: --input-dir, --output-path. | — |
| IMPL-2.2.2 | — | — | Validación | Ejecutar script sobre 3 entidades (ej. 1 patrón, 1 skill, 1 action). Verificar output válido. Documentar en carpeta tarea. | IMPL-2.2.1 |

### Fase 2.3 – Migración único PR

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| IMPL-2.3.1 | Modificar | SddIA/patterns/*/, SddIA/principles/*/, SddIA/skills/*/, SddIA/tools/*/, SddIA/actions/*/, SddIA/process/*/, SddIA/security/*/, SddIA/templates/*/ | Cada entidad | Ejecutar script: spec.json + spec.md → .md con frontmatter. Orden: Patrones (11) → Principios (21) → Skills (11) + Tools (5) → Actions (8) + Process (7) → Security (20) → Templates (1). Excluir SddIA/Tokens/karma2-token. | IMPL-2.2.1 |
| IMPL-2.3.2 | Modificar | SddIA/actions/actions-contract.json (+ .md), SddIA/process/process-contract.json, SddIA/patterns/patterns-contract.json, SddIA/principles/principles-contract.json, SddIA/security/security-contract.json, SddIA/skills/skills-contract.json, SddIA/tools/tools-contract.json, SddIA/templates/templates-contract.json, SddIA/Tokens/tokens-contract.json | Contratos duales | Convertir *-contract.md + *-contract.json → *-contract.md con frontmatter. | IMPL-2.2.1 |
| IMPL-2.3.3 | Eliminar | SddIA/**/spec.json (84 archivos) | — | Eliminar spec.json de cada entidad migrada. No eliminar SddIA/Tokens/karma2-token/spec.json. | IMPL-2.3.1, IMPL-2.3.2 |

### Fase 2.4 – Validación y difusión

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| IMPL-2.4.1 | Modificar | SddIA/actions/validate/spec.md, spec.json | Check sddia_md_json_parity | Renombrar/adaptar: validar frontmatter YAML en .md de entidades con esquema YAML nativo. Si el diff toca paths.skillsDefinitionPath, paths.processPath, etc., comprobar que el .md tenga frontmatter válido. | — |
| IMPL-2.4.2 | Modificar | .cursor/rules/*.mdc | Referencias estructura | Actualizar reglas que referencien spec.md/spec.json: usar frontmatter. | — |
| IMPL-2.4.3 | Modificar | AGENTS.md | Ley L7, referencias | Alinear con nueva estructura: entidades frontmatter; excepción tokens. | — |
| IMPL-2.4.4 | Modificar | docs/evolution/EVOLUTION_LOG.md | Nueva entrada | Añadir sección: fecha, título "Refactorización arquitectura frontmatter", resumen, referencia a docs/features/refactorization-arquitectura-frontmatter/objectives.md. | — |
| IMPL-2.4.5 | Modificar | SddIA/agents/cumulo.json | process_interface | Actualizar process_interface.required_artefacts: en lugar de .md + .json, indicar ".md con frontmatter YAML" por artefacto (objectives, spec, clarify, plan, implementation, validacion). | — |
| IMPL-2.4.6 | Modificar | SddIA/actions/spec/spec.md, SddIA/actions/clarify/spec.md, SddIA/actions/planning/spec.md, SddIA/actions/implementation/spec.md | Salida de la acción | Actualizar: la acción genera artefacto .md con frontmatter (no par .md + .json). Documentar formato esperado en cada spec. | — |

---

## Resumen por archivo

| Archivo / Patrón | Ítems |
|------------------|-------|
| docs/features/refactorization-arquitectura-frontmatter/ | IMPL-1.1, IMPL-1.4 |
| SddIA/**/spec.json (84) | IMPL-1.2, IMPL-2.3.3 |
| scripts/skills-rs/, scripts/tools-rs/ | IMPL-1.3 |
| SddIA/constitution.json | IMPL-2.1.1 |
| SddIA/norms/entidades-dominio-ecosistema-sddia.md | IMPL-2.1.2 |
| 9 contratos | IMPL-2.1.3, IMPL-2.3.2 |
| SddIA/norms/ (interaction-triggers, paths-via-cumulo, patterns, agents-principles, commands, git, README) | IMPL-2.1.4–2.1.9 |
| Script conversión (nuevo) | IMPL-2.2.1 |
| SddIA/actions/validate/ | IMPL-2.4.1 |
| .cursor/rules/ | IMPL-2.4.2 |
| AGENTS.md | IMPL-2.4.3 |
| docs/evolution/EVOLUTION_LOG.md | IMPL-2.4.4 |
| SddIA/agents/cumulo.json | IMPL-2.4.5 |
| SddIA/actions/spec, clarify, planning, implementation | IMPL-2.4.6 |

---

## Orden de ejecución

1. **Fase 1:** IMPL-1.1 → IMPL-1.2 → IMPL-1.3 → IMPL-1.4.
2. **Fase 2.1:** IMPL-2.1.1 → 2.1.2 → 2.1.3 → 2.1.4–2.1.9 (2.1.3 puede iterar por contrato).
3. **Fase 2.2:** IMPL-2.2.1 → IMPL-2.2.2.
4. **Fase 2.3:** IMPL-2.3.1 → IMPL-2.3.2 → IMPL-2.3.3 (en único PR).
5. **Fase 2.4:** IMPL-2.4.1 → 2.4.2 → 2.4.3 → 2.4.4 → 2.4.5 → 2.4.6.

---

## Verificación post-ejecución

- Fase 1: inventario RV-001; spec.json limpio; lazy-loading; métrica tokens.
- Fase 2: normas actualizadas; script operativo; 84 entidades + contratos en frontmatter; karma2-token intacto; validate con esquema YAML; Cursor/AGENTS; process_interface y acciones C8; Evolution Log.
- No coexistencia dual/frontmatter salvo karma2-token.

---

*Documento de implementación para la fase execution. Referencias: spec.md, plan.md, clarify.md.*
