# Especificación: Encapsulamiento de skills

**Feature:** skill.Token  
**Ruta (Cúmulo):** paths.featurePath/skill.Token/

## 1. Patrón a replicar (tools)

- **Definición (SddIA):** `paths.toolsDefinitionPath/<tool-id>/` con spec.md y spec.json; `implementation_path_ref` apunta a Cúmulo (paths.toolCapsules.<tool-id>).
- **Implementación (scripts):** Cápsula en `paths.toolCapsules[<tool-id>]` con manifest.json, .ps1, .bat (launcher: binario en bin/ si existe, si no .ps1), config, doc, bin/.
- **Rust:** Implementación por defecto en Rust; binarios en scripts/tools-rs; install.ps1 copia .exe a cada cápsula bin/.

## 2. Cambios en Cúmulo

Añadir en `SddIA/agents/cumulo.json` → `paths`:

| Clave | Valor | Descripción |
|-------|--------|-------------|
| skillsPath | "./scripts/skills/" | Raíz de implementaciones. |
| skillsDefinitionPath | "./SddIA/skills/" | Raíz de definiciones (spec por skill). |
| skillsIndexPath | "./scripts/skills/index.json" | Índice de skills con implementación ejecutable. |
| skillCapsules | { "iniciar-rama": "./scripts/skills/iniciar-rama/", "finalizar-git": "./scripts/skills/finalizar-git/", "invoke-command": "./scripts/skills/invoke-command/" } | Rutas canónicas de cápsulas. |

En `instructions`: Map [SKILL] → paths.skillsPath; Map [SKILL-DEF] → paths.skillsDefinitionPath/<skill-id>/; listado paths.skillsIndexPath.

## 3. Contrato de skills (SddIA)

- **skills-contract.md:** Documento paralelo a tools-contract.md: desacoplamiento definición/implementación, artefactos por skill con ejecutable (manifest, launcher .bat, .ps1, doc, bin/), **implementación por defecto en Rust** (igual que tools).
- **skills-contract.json:** Incluir `default_implementation: { "language": "rust", "rationale": "...", "delivery": ["..."] }` y `required_artefacts_per_skill` (para skills con script: spec en SddIA, cápsula en scripts con manifest, .bat, .ps1, doc, bin/).

## 4. Estructura SddIA/skills

- **Raíz:** README.md, skills-contract.md, skills-contract.json (contrato global).
- **Por skill:** subcarpeta `<skill-id>/` con:
  - spec.md — Especificación legible (objetivo, entradas, salidas, flujo).
  - spec.json — Metadatos machine-readable; incluir `implementation_path_ref`: "paths.skillCapsules.<skill-id>" si tiene cápsula.

Skills actuales a reestructurar: iniciar-rama, finalizar-git, invoke-command, git-operations, documentation, filesystem-ops, dotnet-development, frontend-build, frontend-test, security-audit. Los que tienen script en scripts/skills tendrán además cápsula; el resto solo definición en SddIA/skills/<skill-id>/.

## 5. Estructura scripts/skills

- **index.json:** index_version, description, skillsPath, cumulo_ref, contract_ref, skills: [ { skillId, path, manifest, launcher_bat, description }, ... ].
- **Por skill con ejecutable:** cápsula `<skill-id>/`:
  - manifest.json (skillId, version, description, contract_ref, components: launcher_bat, launcher_ps1, config, doc, bin).
  - &lt;ScriptName&gt;.ps1 (fallback).
  - &lt;ScriptName&gt;.bat (invocar bin/<skill_bin>.exe si existe, si no .ps1).
  - Documentación .md (es-ES).
  - bin/ (ejecutable Rust copiado por skills-rs/install.ps1).
  - Config .json si aplica.

Migración: Iniciar-Rama.ps1 → scripts/skills/iniciar-rama/; Merge-To-Master-Cleanup.ps1 y Unificar-Rama.ps1 → scripts/skills/finalizar-git/; Invoke-Command.ps1 + Invoke-Command.json → scripts/skills/invoke-command/.

## 6. scripts/skills-rs

- Proyecto Rust (Cargo.toml, src/lib.rs con tipos de resultado si se desea salida JSON; src/bin/*.rs).
- Binarios: iniciar_rama, merge_to_master_cleanup (y opcionalmente invoke_command si se implementa).
- install.ps1: cargo build --release; copiar .exe a scripts/skills/<skill-id>/bin/ según skillCapsules.

## 7. Agentes y constitution

- **feature.md:** Fase 0: invocar skill iniciar-rama vía path Cúmulo (paths.skillCapsules["iniciar-rama"]) o documentar "Iniciar-Rama.bat en la cápsula iniciar-rama".
- **bug-fix-specialist.json, finalize.md, tekton-developer.json, etc.:** Sustituir rutas literales "scripts/skills/Iniciar-Rama.ps1" por referencia a la cápsula (ej. scripts/skills/iniciar-rama/Iniciar-Rama.bat o resolución vía Cúmulo).
- **constitution.json:** Si existe referencia a skills, indicar consulta a Cúmulo para rutas.

## 8. Restricciones

- skill-id en kebab-case.
- Entorno: Windows 11, PowerShell 7+ (Ley 2).
- Rutas canónicas solo desde Cúmulo; no rutas literales en documentación.
