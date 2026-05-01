---
skill_id: git-branch-manager
name: "Git Branch Manager"
description: "Cambia o crea rama con git switch; devuelve rama activa."
contract_ref: SddIA/skills/skills-contract.md
implementation_path_ref: paths.skillCapsules.git-branch-manager
parameters:
  branch_name:
    description: "Nombre de rama destino."
    required: true
  create:
    description: "Si true, crea la rama (git switch -c). Si false, solo cambia (git switch)."
    required: false
    default: false
rules:
  - "Ejecuta git switch según create."
  - "Devuelve activeBranch en result."
json_io_ref: SddIA/norms/capsule-json-io.md
---

# Skill: git-branch-manager

## Entrada (request)

```json
{ "branch_name": "feat/mi-rama", "create": true }
```

## Salida (result)

- `activeBranch`: rama activa tras el switch.

## Implementación

- Rust: `scripts/skills-rs/src/bin/git_branch_manager.rs`
- Cápsula: `paths.skillCapsules.git-branch-manager`

