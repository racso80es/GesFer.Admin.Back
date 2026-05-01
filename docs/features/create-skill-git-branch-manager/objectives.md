---
feature_name: create-skill-git-branch-manager
created: "2026-04-30"
process: create-skill
---

## Objetivo

Crear la skill **git-branch-manager** para cambiar o crear ramas (`git switch` / `git switch -c`) devolviendo confirmación de rama activa en `result`.

## Alcance

- **skill_id**: `git-branch-manager`
- **Definición**: `paths.skillsDefinitionPath/git-branch-manager/spec.md`
- **Cápsula**: `paths.skillCapsules.git-branch-manager`
- **Implementación**: Rust (`scripts/skills-rs/src/bin/git_branch_manager.rs`) → `git_branch_manager.exe` en la raíz de la cápsula.

