# Implementación: create-skill-invoke-commit

**Rama:** feat/create-skill-invoke-commit

## Touchpoints

| # | Archivo | Acción |
|---|---------|--------|
| 1 | SddIA/skills/invoke-commit/spec.md | Crear (copiar desde docs/features/.../spec.md, adaptar) |
| 2 | SddIA/skills/invoke-commit/spec.json | Crear |
| 3 | scripts/skills-rs/src/bin/invoke_commit.rs | Crear |
| 4 | scripts/skills-rs/Cargo.toml | Añadir [[bin]] invoke_commit |
| 5 | scripts/skills/invoke-commit/Invoke-Commit.bat | Crear |
| 6 | scripts/skills/invoke-commit/manifest.json | Crear |
| 7 | scripts/skills/invoke-commit/invoke-commit.md | Crear |
| 8 | scripts/skills-rs/src/bin/push_and_create_pr.rs | Modificar: --body, --body-file |
| 9 | scripts/skills-rs/install.ps1 | Añadir invoke_commit → invoke-commit |
| 10 | SddIA/agents/cumulo.paths.json | Añadir skillCapsules.invoke-commit |
| 11 | scripts/skills/index.json | Añadir entrada invoke-commit |

## Detalle invoke_commit.rs

- Parsear: --message/-m (obligatorio), --files (comma-separated), --all/-a, --type, --scope, --fase, --contexto
- Construir mensaje: si --type y --scope: `{type}({scope}): {message}`; si solo --type: `{type}: {message}`; si no: `{message}`
- git add: si --all → `git add -A`; si --files → `git add file1 file2 ...` (split por coma)
- git commit -m "{mensaje}"
- Log en docs/diagnostics/{branch}/execution_history.json (formato como invoke_command)

## Detalle push_and_create_pr.rs

- Añadir --body y --body-file
- body = si --body-file y existe fichero → leer; si --body → usar; si --persist → Documentación; else Rama
