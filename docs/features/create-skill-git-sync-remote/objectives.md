---
feature_name: create-skill-git-sync-remote
created: "2026-04-30"
process: create-skill
---

## Objetivo

Crear la skill **git-sync-remote** para sincronizar con remoto: `git fetch` → `git pull --rebase` → `git push origin HEAD` (opcional `--force-with-lease`).

## Alcance

- **skill_id**: `git-sync-remote`
- **Definición**: `paths.skillsDefinitionPath/git-sync-remote/spec.md`
- **Cápsula**: `paths.skillCapsules.git-sync-remote`
- **Implementación**: Rust (`scripts/skills-rs/src/bin/git_sync_remote.rs`) → `git_sync_remote.exe` en la raíz de la cápsula.

