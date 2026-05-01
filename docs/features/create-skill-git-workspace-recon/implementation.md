---
feature_name: create-skill-git-workspace-recon
created: "2026-04-30"
items:
  - id: 1
    action: create
    path: SddIA/skills/git-workspace-recon/spec.md
    proposal: "Definición SddIA con inputs/outputs y referencia a cápsula."
  - id: 2
    action: create
    path: scripts/skills-rs/src/bin/git_workspace_recon.rs
    proposal: "Bin Rust: ejecutar git status -s y git diff --stat, parsear, devolver capsule-json-io v2."
  - id: 3
    action: update
    path: scripts/skills-rs/Cargo.toml
    proposal: "Añadir [[bin]] git_workspace_recon."
  - id: 4
    action: create
    path: scripts/skills/git-workspace-recon/manifest.json
    proposal: "Manifest de cápsula (v2) con componentes (exe/doc/launcher_bat opcional)."
  - id: 5
    action: create
    path: scripts/skills/git-workspace-recon/git-workspace-recon.md
    proposal: "Documentación de uso y ejemplo de envelope JSON."
  - id: 6
    action: create
    path: scripts/skills/git-workspace-recon/Git-Workspace-Recon.bat
    proposal: "Launcher humano opcional (invoca git_workspace_recon.exe con modo CLI)."
  - id: 7
    action: update
    path: scripts/skills-rs/install.ps1
    proposal: "Copiar git_workspace_recon.exe a la cápsula."
  - id: 8
    action: update
    path: scripts/skills/index.json
    proposal: "Registrar la skill en el índice."
  - id: 9
    action: update
    path: SddIA/agents/cumulo.paths.json
    proposal: "Registrar paths.skillCapsules.git-workspace-recon."
---

## Touchpoints

Los ítems anteriores son los touchpoints canónicos que materializan la skill según `SddIA/process/create-skill/spec.md` y el contrato `SddIA/skills/skills-contract.md`.

