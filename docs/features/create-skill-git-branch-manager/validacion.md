---
feature_name: create-skill-git-branch-manager
branch: feat/create-skill-git-branch-manager
base_branch: main
global: fail
checks:
  - name: "build (Rust)"
    result: pending
    message: "Pendiente de compilación/copia de git_branch_manager.exe."
  - name: "contract (capsule-json-io v2)"
    result: pending
    message: "Pendiente de ejecución empírica."
git_changes:
  files_added: []
  files_modified: []
  files_deleted: []
---

## Validación

Pendiente de evidencia empírica (ejecución en repo con ramas existentes y creación con -c).

