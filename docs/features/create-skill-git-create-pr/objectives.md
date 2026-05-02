---
feature_name: create-skill-git-create-pr
created: "2026-04-30"
process: create-skill
---

## Objetivo

Crear la skill **git-create-pr** para crear Pull Requests vía `gh pr create` y devolver la URL creada.

## Alcance

- **skill_id**: `git-create-pr`
- **Definición**: `paths.skillsDefinitionPath/git-create-pr/spec.md`
- **Cápsula**: `paths.skillCapsules.git-create-pr`
- **Implementación**: Rust (`scripts/skills-rs/src/bin/git_create_pr.rs`) → `git_create_pr.exe` en la raíz de la cápsula.

