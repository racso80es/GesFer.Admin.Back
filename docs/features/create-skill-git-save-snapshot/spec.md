---
feature_name: create-skill-git-save-snapshot
created: "2026-04-30"
base:
  - objectives.md
functional_requirements:
  - "FR1: Ejecutar `git add .`."
  - "FR2: Ejecutar `git commit -m <commit_message>`."
  - "FR3: Si no hay cambios ('nothing to commit'), no fallar: success=true."
non_functional_requirements:
  - "RNF1: Capturar stdout y stderr."
  - "RNF2: Cumplir envelope JSON v2."
validation_criteria:
  - "VC1: Con cambios, devuelve commitHash."
  - "VC2: Sin cambios, devuelve success=true con mensaje 'Nothing to commit'."
---

## Entrada (request)

```json
{ "commit_message": "chore: snapshot" }
```

## Salida (result)

```json
{
  "committed": true,
  "commitHash": "abcd1234",
  "add": { "exitCode": 0, "output": "..." },
  "commit": { "exitCode": 0, "output": "..." }
}
```

