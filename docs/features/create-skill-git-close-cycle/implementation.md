---
feature_name: create-skill-git-close-cycle
artifact: implementation
process: feature
---

## Touchpoints

| Área | Ruta / entidad |
| --- | --- |
| Definición skill | `SddIA/skills/git-close-cycle/spec.md` |
| Cúmulo | `SddIA/agents/cumulo.paths.json` → `skillCapsules.git-close-cycle` |
| Rust | `scripts/skills-rs/src/bin/git_close_cycle.rs` |
| Manifest crate | `scripts/skills-rs/Cargo.toml` → `[[bin]] git_close_cycle` |
| Cápsula | `scripts/skills/git-close-cycle/` (`manifest.json`, `Git-Close-Cycle.bat`, doc opcional) |
| Índice skills | `scripts/skills/index.json` |
| Install | `scripts/skills-rs/install.ps1` → entrada `git_close_cycle` / `git-close-cycle` |
| Acción | `SddIA/actions/finalize-process/spec.md` (frontmatter + cuerpo) |
| Proceso feature | `SddIA/process/feature/spec.md` → `related_skills` |
| Difusión | `SddIA/norms/interaction-triggers.md`, `SddIA/skills/README.md`, `.cursor/rules/skill-suggestions.mdc` |
| Evolution | `SddIA/evolution/` vía `sddia_evolution_register` |

## Contrato request (JSON)

```json
{ "targetBranch": "feat/create-skill-git-close-cycle" }
```
