---
feature_name: create-skill-git-tactical-retreat
created: "2026-04-30"
process: create-skill
---

## Objetivo

Crear la skill **git-tactical-retreat** para revertir cambios: `git checkout -- <path>` y/o limpieza total con `git reset --hard HEAD` + `git clean -fd`.

## Restricción de seguridad (Zero Vision)

La operación destructiva (`hard_reset=true`) exige confirmación explícita en el request: `confirm_destructive=true`. Sin esa confirmación la skill debe fallar (no ejecutar reset/clean).

## Alcance

- **skill_id**: `git-tactical-retreat`
- **Definición**: `paths.skillsDefinitionPath/git-tactical-retreat/spec.md`
- **Cápsula**: `paths.skillCapsules.git-tactical-retreat`
- **Implementación**: Rust (`scripts/skills-rs/src/bin/git_tactical_retreat.rs`) → `git_tactical_retreat.exe` en la raíz de la cápsula.

