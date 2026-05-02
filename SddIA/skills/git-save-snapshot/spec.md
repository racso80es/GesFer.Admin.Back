---
skill_id: git-save-snapshot
name: "Git Save Snapshot"
description: "git add . + git commit -m. Trata 'nothing to commit' como estado no crítico."
contract_ref: SddIA/skills/skills-contract.md
implementation_path_ref: paths.skillCapsules.git-save-snapshot
parameters:
  commit_message:
    description: "Mensaje del commit."
    required: true
rules:
  - "Ejecuta git add . y git commit -m."
  - "Si git commit reporta 'nothing to commit', devuelve success=true (exitCode 0)."
json_io_ref: SddIA/norms/capsule-json-io.md
---

# Skill: git-save-snapshot

## Entrada (request)

```json
{ "commit_message": "chore: snapshot" }
```

## Salida (result)

- `committed`: true si se creó commit; false si “nothing to commit”.
- `commitHash`: hash del commit cuando `committed=true`.

