---
feature_name: create-skill-git-close-cycle
artifact: execution
process: feature
---

## Registro de ejecución

- Skill **git-close-cycle**: `scripts/skills-rs/src/bin/git_close_cycle.rs`, cápsula `scripts/skills/git-close-cycle/`, entradas en `cumulo.paths.json`, `index.json`, `install.ps1`.
- Acción **finalize-process**: `SddIA/actions/finalize-process/spec.md` actualizado (orquestación post-fusión).
- Difusión: `interaction-triggers.md`, `SddIA/skills/README.md`, `.cursor/rules/skill-suggestions.mdc`, `SddIA/process/feature/spec.md`.
- Build: `cargo build --release` + `install.ps1` (exe copiado a cápsula).
