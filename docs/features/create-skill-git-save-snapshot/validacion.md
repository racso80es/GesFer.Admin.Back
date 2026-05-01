---
feature_name: create-skill-git-save-snapshot
branch: feat/create-skill-git-save-snapshot
base_branch: main
global: fail
checks:
  - name: "build (Rust)"
    result: pending
    message: "Pendiente de compilación/copia de git_save_snapshot.exe."
  - name: "smoke (nothing to commit)"
    result: pending
    message: "Pendiente de ejecución empírica en workspace limpio."
git_changes:
  files_added: []
  files_modified: []
  files_deleted: []
---

## Validación

Pendiente de evidencia empírica.

