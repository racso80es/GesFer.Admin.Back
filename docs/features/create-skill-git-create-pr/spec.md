---
feature_name: create-skill-git-create-pr
created: "2026-04-30"
base:
  - objectives.md
functional_requirements:
  - "FR1: Ejecutar `gh pr create --title <title> --body <body> --base <base_branch>`."
  - "FR2: base_branch es opcional (default: main)."
  - "FR3: Devolver URL del PR creado en result.prUrl."
  - "FR4: Si ya existe un PR, devolver success=true y result.prUrl con la URL existente."
non_functional_requirements:
  - "RNF1: Capturar stdout/stderr."
  - "RNF2: Cumplir envelope JSON v2."
validation_criteria:
  - "VC1: Con gh autenticado, crea PR y devuelve URL."
  - "VC2: Si PR ya existe, devuelve URL existente sin fallar."
---

## Entrada (request)

```json
{
  "title": "feat: mi cambio",
  "body": "## Summary\n...\n",
  "base_branch": "main"
}
```

## Salida (result)

```json
{
  "prUrl": "https://github.com/org/repo/pull/123",
  "created": true
}
```

