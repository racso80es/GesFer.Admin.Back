---
feature_name: create-skill-git-tactical-retreat
created: "2026-04-30"
base:
  - objectives.md
functional_requirements:
  - "FR1: Si request.target_path existe, ejecutar `git checkout -- <path>`."
  - "FR2: Si request.hard_reset=true, ejecutar `git reset --hard HEAD` y `git clean -fd`."
  - "FR3: hard_reset exige confirmación explícita: request.confirm_destructive=true."
non_functional_requirements:
  - "RNF1: Capturar stdout/stderr."
  - "RNF2: Cumplir envelope JSON v2."
validation_criteria:
  - "VC1: target_path revierte fichero sin tocar otros cambios."
  - "VC2: hard_reset sin confirm_destructive falla sin ejecutar acciones."
  - "VC3: hard_reset con confirm_destructive limpia workspace."
---

## Entrada (request)

```json
{ "target_path": "src/file.cs", "hard_reset": false }
```

```json
{ "hard_reset": true, "confirm_destructive": true }
```

## Salida (result)

```json
{
  "checkout": { "executed": true, "exitCode": 0, "output": "..." },
  "resetHard": { "executed": false, "exitCode": null, "output": "" },
  "cleanFd": { "executed": false, "exitCode": null, "output": "" }
}
```

