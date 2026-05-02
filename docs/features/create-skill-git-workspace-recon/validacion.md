---
feature_name: create-skill-git-workspace-recon
branch: feat/create-skill-git-workspace-recon
base_branch: main
global: fail
checks:
  - name: "build (Rust)"
    result: pending
    message: "Pendiente de compilación en scripts/skills-rs y copia a cápsula."
  - name: "contract (capsule-json-io v2)"
    result: pass
    message: "Implementación usa CapsuleResponse v2 y parseo de request."
git_changes:
  files_added: []
  files_modified: []
  files_deleted: []
---

## Validación

Pendiente de evidencia empírica (compilación/copia del `.exe` y ejecución contra repo limpio y con cambios).

