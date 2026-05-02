---
skill_id: git-create-pr
name: "Git Create PR"
description: "Crea Pull Request vía gh pr create y devuelve la URL."
contract_ref: SddIA/skills/skills-contract.md
implementation_path_ref: paths.skillCapsules.git-create-pr
parameters:
  title:
    description: "Título del PR."
    required: true
  body:
    description: "Body del PR."
    required: true
  base_branch:
    description: "Rama base (default: main)."
    required: false
    default: main
rules:
  - "Si ya existe PR, devuelve success=true y URL existente."
json_io_ref: SddIA/norms/capsule-json-io.md
---

# Skill: git-create-pr

## Entrada (request)

```json
{ "title": "feat: X", "body": "## Summary\n...", "base_branch": "main" }
```

## Salida (result)

- `prUrl`: URL del PR.
- `created`: true si se creó, false si ya existía.

