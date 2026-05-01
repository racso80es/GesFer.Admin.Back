---
norm_id: git-operations
related:
  - SddIA/norms/git-via-skills-or-process.md
  - SddIA/norms/commands-via-skills-or-tools.md
  - paths.skillCapsules.git-sync-remote
description: >-
  Reglas duras para operaciones Git por agentes: prohibición de bypass con invoke-command o terminal nativa;
  fallo de cápsula implica reporte y refactorización, no atajo manual.
---

# Norma: Operaciones Git (Ley de Hierro para agentes)

**Alcance:** agentes de IA (p. ej. Tekton) y cualquier automatización que deba mutar el repositorio Git.

**Fuente canónica complementaria:** `SddIA/norms/git-via-skills-or-process.md`, `AGENTS.md` (Ley GIT, Ley COMANDOS).

## Ley de Hierro

**Prohibición de comandos primitivos:** Queda **estrictamente prohibido** el uso de `invoke-command`, `powershell -Command`, `cmd /c` o **cualquier ejecución de terminal nativa** para **operaciones de Git** (incluye `git push`, `git pull`, `git fetch`, `git branch`, `git merge`, `git rebase`, etc.).

Si una **skill** del Arsenal Git (p. ej. `git-sync-remote`) **falla**, el agente debe **reportar la incidencia** (mensaje, `exitCode`, salida de la cápsula) y **solicitar o ejecutar la refactorización de la cápsula** conforme al proceso de tarea. Queda **explícitamente prohibido el bypass manual** (p. ej. empujar la rama con un comando suelto) para garantizar **trazabilidad** (`docs/diagnostics/.../execution_history.json`, envelope JSON, ausencia de entropía no registrada).

## Corolarios

1. **Único canal Git para la IA:** skills Git S+ (`git-workspace-recon`, `git-branch-manager`, `git-save-snapshot`, `git-sync-remote`, `git-tactical-retreat`, `git-create-pr`) e, cuando aplique al contrato, `invoke-commit` para commits parametrizados — **nunca** `invoke-command` con cadena `git ...`.
2. **Errores explícitos:** Una cápsula debe devolver `success: false` y `exitCode` ≠ 0 con mensaje claro; el agente **no** reinterpreta un fallo como éxito.
3. **Coherencia con acciones:** La acción `finalize-process` orquesta Git S+; esta norma refuerza que ningún paso intermedio sustituya la cápsula por shell.

## Referencias

- Cápsula `git-sync-remote`: `paths.skillCapsules.git-sync-remote`, contrato `SddIA/norms/capsule-json-io.md`.
- Invocación estable: `scripts/skills/run-capsule-from-tekton-request.ps1` + `.tekton_request.json` en la raíz del repositorio.
