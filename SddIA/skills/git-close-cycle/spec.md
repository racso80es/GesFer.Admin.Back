---
skill_id: git-close-cycle
name: "Git Close Cycle"
description: "Cierra el ciclo local tras fusión en remoto: checkout troncal, pull, fetch --prune, eliminar rama de trabajo."
contract_ref: SddIA/skills/skills-contract.md
implementation_path_ref: paths.skillCapsules.git-close-cycle
parameters:
  target_branch:
    description: "Nombre de la rama de trabajo a eliminar en local (feat/ o fix/)."
    required: true
rules:
  - "Resuelve troncal local main o master (primera existente)."
  - "Ejecuta git checkout, git pull origin HEAD, git fetch --prune, luego git branch -d con fallback a -D."
  - "Si la rama objetivo no existe en local, finaliza con éxito y aviso."
json_io_ref: SddIA/norms/capsule-json-io.md
---

# Skill: git-close-cycle

## Propósito

Actualizar el clon local al estado del remoto en el troncal y **eliminar** la rama de tarea ya integrada, manteniendo refs al día (`fetch --prune`).

## Entrada (`request`)

```json
{ "targetBranch": "feat/mi-feature" }
```

## Salida (`result`)

Objeto con fases: `trunk`, `checkout`, `pull`, `fetchPrune`, `deleteBranch` (cada una con `exitCode` y `output` cuando aplica).

## Implementación

- Rust: `scripts/skills-rs/src/bin/git_close_cycle.rs`
- Cápsula: `paths.skillCapsules.git-close-cycle`
