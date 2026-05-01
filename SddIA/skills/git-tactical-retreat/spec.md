---
skill_id: git-tactical-retreat
name: "Git Tactical Retreat"
description: "Revertir cambios por fichero y/o limpieza total (reset --hard + clean -fd) con guardas destructivas."
contract_ref: SddIA/skills/skills-contract.md
implementation_path_ref: paths.skillCapsules.git-tactical-retreat
parameters:
  target_path:
    description: "Ruta opcional a revertir con git checkout -- <path>."
    required: false
  hard_reset:
    description: "Si true, ejecuta reset --hard y clean -fd (requiere confirmación)."
    required: false
    default: false
  confirm_destructive:
    description: "Debe ser true si hard_reset=true. Sin esta confirmación, la skill no ejecuta acciones destructivas."
    required: false
rules:
  - "hard_reset=true exige confirm_destructive=true (Zero Vision)."
json_io_ref: SddIA/norms/capsule-json-io.md
---

# Skill: git-tactical-retreat

## Entrada (request)

```json
{ "target_path": "src/file.cs" }
```

```json
{ "hard_reset": true, "confirm_destructive": true }
```

