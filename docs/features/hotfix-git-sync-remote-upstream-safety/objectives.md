---
feature_name: hotfix-git-sync-remote-upstream-safety
created: "2026-05-01"
process: feature
---

## Objetivo

Corregir **git-sync-remote** para ramas **sin upstream**: detectar ausencia de seguimiento, omitir `pull --rebase` inválido y ejecutar `git push -u origin HEAD` automáticamente. Documentar la **Ley de Hierro** en `SddIA/norms/git-operations.md` (prohibición de `invoke-command`/terminal nativa para Git). Eliminar residuos en `docs/diagnostics/` y registrar evolution.

## Ley aplicada

- `SddIA/norms/git-via-skills-or-process.md`, `SddIA/norms/git-operations.md`, `AGENTS.md`.
