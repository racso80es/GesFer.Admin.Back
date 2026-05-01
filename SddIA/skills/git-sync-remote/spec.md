---
skill_id: git-sync-remote
name: "Git Sync Remote"
description: "Sincroniza repo con remoto: fetch + pull --rebase + push origin HEAD (opcional --force-with-lease)."
contract_ref: SddIA/skills/skills-contract.md
implementation_path_ref: paths.skillCapsules.git-sync-remote
parameters:
  force:
    description: "Si true, usa --force-with-lease en el push."
    required: false
    default: false
rules:
  - "Captura stdout/stderr de fetch, pull --rebase y push."
  - "Mensajes 'up-to-date' se consideran no críticos (success=true)."
json_io_ref: SddIA/norms/capsule-json-io.md
---

# Skill: git-sync-remote

## Entrada (request)

```json
{ "force": false }
```

## Salida (result)

`fetch`, `pullRebase` y `push` contienen exitCode + output.

