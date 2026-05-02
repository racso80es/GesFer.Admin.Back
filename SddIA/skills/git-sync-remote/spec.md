---
skill_id: git-sync-remote
name: "Git Sync Remote"
description: "Sincroniza repo con remoto: fetch; si hay upstream, pull --rebase; push (si no hay upstream: push -u origin HEAD). Opcional --force-with-lease."
contract_ref: SddIA/skills/skills-contract.md
implementation_path_ref: paths.skillCapsules.git-sync-remote
parameters:
  force:
    description: "Si true, usa --force-with-lease en el push."
    required: false
    default: false
rules:
  - "Tras fetch, si la rama no tiene upstream (git rev-parse @{u} falla), omitir pull --rebase y ejecutar git push -u origin HEAD (o con --force-with-lease si force=true)."
  - "Si hay upstream, ejecutar pull --rebase antes del push."
  - "Captura stdout/stderr; errores devuelven success=false y mensaje explícito (nunca éxito falso)."
  - "Mensajes 'up-to-date' en push se consideran no críticos (success=true)."
json_io_ref: SddIA/norms/capsule-json-io.md
---

# Skill: git-sync-remote

## Entrada (request)

```json
{ "force": false }
```

## Salida (result)

- `hadUpstream`: boolean
- `pushMode`: `"normal"` | `"setUpstream"`
- `fetch`, `pullRebase` (con `skipped` si no hubo pull), `push`: exitCode + output

