# Refactor: skill finalizar-git → FinalizarProceso

**Rama:** feat/refactor-skill-finalizar-proceso  
**Ruta (Cúmulo):** paths.featurePath/refactor-skill-finalizar-proceso

## Objetivo

Renombrar la skill `finalizar-git` a **FinalizarProceso** (skill_id: `finalizar-proceso`): definición en SddIA/skills/finalizar-proceso/, cápsula en scripts/skills/finalizar-proceso/, Cúmulo y referencias actualizados; carpetas antiguas finalizar-git eliminadas.

## Alcance

- Definición: SddIA/skills/finalizar-proceso/ (spec.md, spec.json).
- Cápsula: scripts/skills/finalizar-proceso/ (todos los componentes).
- paths.skillCapsules["finalizar-proceso"], Invoke-Finalize.ps1, index.json, install.ps1, normas y .cursor/rules actualizados.
