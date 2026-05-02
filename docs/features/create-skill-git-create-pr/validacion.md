---
feature_name: create-skill-git-create-pr
branch: feat/create-skill-git-create-pr
base_branch: main
global: fail
checks:
  - name: "build (Rust)"
    result: pending
    message: "Pendiente de compilación/copia de git_create_pr.exe."
  - name: "smoke (gh pr create)"
    result: pending
    message: "Pendiente de evidencia empírica con gh autenticado."
git_changes:
  files_added: []
  files_modified: []
  files_deleted: []
---

## Validación

Pendiente de evidencia empírica.

