---
feature_name: create-skill-git-branch-manager
created: "2026-04-30"
base:
  - objectives.md
functional_requirements:
  - "FR1: Si request.create=true, ejecutar `git switch -c <branch_name>`."
  - "FR2: Si request.create=false, ejecutar `git switch <branch_name>`."
  - "FR3: Devolver la rama activa en result."
non_functional_requirements:
  - "RNF1: Capturar stdout y stderr."
  - "RNF2: Cumplir envelope JSON v2."
validation_criteria:
  - "VC1: Con create=true, la rama creada queda activa y se reporta en result.activeBranch."
  - "VC2: Con create=false, cambia a rama existente y se reporta."
---

## Entrada (request)

```json
{
  "branch_name": "feat/mi-rama",
  "create": true
}
```

## Salida (result)

```json
{
  "activeBranch": "feat/mi-rama",
  "switch": {
    "exitCode": 0,
    "output": "..."
  }
}
```

