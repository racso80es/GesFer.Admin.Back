---
feature_name: create-skill-git-workspace-recon
created: "2026-04-30"
process: create-skill
---

## Objetivo

Forjar la skill **git-workspace-recon** para inspección táctica del workspace Git (status + diff stat) bajo el contrato **capsule-json-io v2**.

## Alcance

- **skill_id**: `git-workspace-recon`
- **Definición**: `paths.skillsDefinitionPath/git-workspace-recon/spec.md`
- **Cápsula**: `paths.skillCapsules.git-workspace-recon`
- **Implementación**: Rust (`scripts/skills-rs/src/bin/git_workspace_recon.rs`) → binario `git_workspace_recon.exe` en la raíz de la cápsula.

## Ley aplicada

- `SddIA/norms/commands-via-skills-or-tools.md`
- `SddIA/norms/git-via-skills-or-process.md`
- `SddIA/norms/capsule-json-io.md` (schema_version 2.0)

