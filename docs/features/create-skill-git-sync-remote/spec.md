---
feature_name: create-skill-git-sync-remote
created: "2026-04-30"
base:
  - objectives.md
functional_requirements:
  - "FR1: Ejecutar git fetch."
  - "FR2: Ejecutar git pull --rebase."
  - "FR3: Ejecutar git push origin HEAD (con --force-with-lease si request.force=true)."
non_functional_requirements:
  - "RNF1: Capturar stdout/stderr de cada comando."
  - "RNF2: Cumplir envelope JSON v2."
  - "RNF3: Estados no críticos (p. ej. 'Everything up-to-date', 'Already up to date') no deben fallar."
validation_criteria:
  - "VC1: Con repo actualizado, success=true y output indicando up-to-date."
  - "VC2: Con cambios locales, push exitoso devuelve success=true."
---

## Entrada (request)

```json
{ "force": false }
```

## Salida (result)

```json
{
  "fetch": { "exitCode": 0, "output": "..." },
  "pullRebase": { "exitCode": 0, "output": "..." },
  "push": { "exitCode": 0, "output": "..." }
}
```

