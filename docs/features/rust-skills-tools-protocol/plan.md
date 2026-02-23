# Plan: rust-skills-tools-protocol

**Feature:** rust-skills-tools-protocol  
**Rama:** feat/rust-skills-tools-protocol  
**Cúmulo:** paths.featurePath/rust-skills-tools-protocol/

---

## Tareas en orden

| # | Tarea | Responsable | Salida |
|---|--------|-------------|--------|
| 1 | Crear SddIA/norms/commands-via-skills-or-tools.md | Tekton/Cúmulo | Norma redactada. |
| 2 | Actualizar SddIA/norms/README.md con entrada de la nueva norma | Tekton | Índice actualizado. |
| 3 | Actualizar AGENTS.md: añadir referencia a norma de comandos (ley o sección) | Tekton | AGENTS coherente con norma. |
| 4 | Actualizar SddIA/constitution.json: configuration.skills_tools_implementation y paths_ref | Tekton | Constitution con contexto Rust. |
| 5 | Actualizar SddIA/actions/sddia-difusion/spec.md: criterios norma + Rust | Tekton | Spec sddia-difusion actualizado. |
| 6 | Crear .cursor/rules/commands-via-skills-tools.mdc (difusión) | Tekton | Regla Cursor. |
| 7 | Opcional: actualizar .cursor/rules/sddia-ssot.mdc con mención a Rust y norma | Tekton | SSOT alineado. |
| 8 | Validación: build, lectura de normas, checklist sddia-difusion | QA/Juez | validacion.json. |
| 9 | Finalize: commit, push, PR (skill finalizar-git) | Tekton | PR abierto. |

## Dependencias

- 1 → 2. 3 y 4 pueden ir en paralelo. 5 → 6 (sddia-difusion define qué difundir; 6 implementa la difusión). 7 opcional tras 6.

## Criterios de validación

- cargo build en skills-rs y tools-rs OK (sin cambios de código en este feature, solo comprobar).
- Existencia de commands-via-skills-or-tools.md y entrada en norms/README.md.
- AGENTS.md y constitution.json contienen las referencias acordadas.
- .cursor/rules/commands-via-skills-tools.mdc existe y es coherente con la norma.
