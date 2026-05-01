---
feature_name: hotfix-git-sync-remote-upstream-safety
created: "2026-05-01"
phases:
  - id: P1
    name: Rust + normas + limpieza
  - id: P2
    name: Evolution + PR vía skills
---

# Plan

1. Refactor `git_sync_remote.rs`; `cargo build --release`; `install.ps1`.
2. Normas `git-operations.md`, actualizar `git-via-skills-or-process.md`, skill spec.
3. Borrar `docs/diagnostics/feat/refactor-actions-contract-and-finalize/execution_history.json`.
4. `sddia_evolution_register` + snapshots + `git-sync-remote` + `git-create-pr`.
