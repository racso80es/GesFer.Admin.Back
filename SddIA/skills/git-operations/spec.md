---
common_workflows:
  finalize-process: git-sync-remote (skills) + git-create-pr (skills)
  save_progress: "git-save-snapshot (skills) con Conventional Commits"
  start_task: "git-branch-manager (skills) para feat/fix"
contract_ref: paths.skillsDefinitionPath/skills-contract.md (Cúmulo)
rules:
- 'STRICT PROHIBITION: Never commit directly to ''master'' or ''main''.'
- 'Mandatory Branching: Work must be done in ''feat/<desc>'' or ''fix/<desc>'' branches.'
- 'Commit Message Format: Use Conventional Commits (e.g. ''feat: add login'', ''fix: resolve null ref'').'
- 'Pre-Push: Always run local validation (build/tests) before requesting to push.'
skill_id: git-operations
---

# Skill: Git Operations Standard

**skill_id:** `git-operations`

## Objetivo

Uso seguro y semántico de Git: ramas feat/fix, commits convencionales, pre-push.

## Reglas

- **STRICT PROHIBITION:** Never commit directly to 'master' or 'main'.
- **Mandatory Branching:** Work must be done in 'feat/<desc>' or 'fix/<desc>' branches.
- **Commit Message Format:** Use Conventional Commits (e.g. 'feat: add login', 'fix: resolve null ref').
- **Pre-Push:** Always run local validation (build/tests) before requesting to push.

## Common workflows

- start_task: git-branch-manager (suite Git S+)
- save_progress: git-save-snapshot (suite Git S+)
- finalize-process: git-sync-remote + git-create-pr (suite Git S+; no git push directo)

## Alcance

Skill de definición únicamente (sin cápsula ejecutable). Consumido por agentes y procesos (feature, bug-fix).

---
*Definición en paths.skillsDefinitionPath/git-operations/ (contrato paths.skillsDefinitionPath/skills-contract.md).*
