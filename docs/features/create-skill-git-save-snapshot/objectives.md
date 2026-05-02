---
feature_name: create-skill-git-save-snapshot
created: "2026-04-30"
process: create-skill
---

## Objetivo

Crear la skill **git-save-snapshot** para guardar un snapshot del workspace: `git add .` + `git commit -m "<mensaje>"`, tolerando el estado no crítico **“nothing to commit”**.

## Alcance

- **skill_id**: `git-save-snapshot`
- **Definición**: `paths.skillsDefinitionPath/git-save-snapshot/spec.md`
- **Cápsula**: `paths.skillCapsules.git-save-snapshot`
- **Implementación**: Rust (`scripts/skills-rs/src/bin/git_save_snapshot.rs`) → `git_save_snapshot.exe` en la raíz de la cápsula.

