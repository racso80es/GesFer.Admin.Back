# Plan: create-skill-invoke-commit

**Rama:** feat/create-skill-invoke-commit  
**Persistencia:** paths.featurePath/create-skill-invoke-commit/

## Fases

### 1. Definición SddIA
- Crear `SddIA/skills/invoke-commit/spec.md`
- Crear `SddIA/skills/invoke-commit/spec.json`

### 2. Implementación invoke_commit (Rust)
- Crear `scripts/skills-rs/src/bin/invoke_commit.rs`
- Añadir binario a `scripts/skills-rs/Cargo.toml`
- Parámetros: --message, --files (comma-separated), --all, --type, --scope, --fase, --contexto
- Conventional Commits: type(scope): message
- Registro en docs/diagnostics/{branch}/execution_history.json

### 3. Cápsula invoke-commit
- Crear `scripts/skills/invoke-commit/`
- Invoke-Commit.bat (launcher)
- manifest.json
- invoke-commit.md (documentación)

### 4. Extensión push_and_create_pr
- Añadir --body y --body-file a `scripts/skills-rs/src/bin/push_and_create_pr.rs`
- Precedencia: body-file > body > persist > rama

### 5. Integración
- Actualizar `scripts/skills-rs/install.ps1` (invoke_commit → invoke-commit)
- Actualizar `SddIA/agents/cumulo.paths.json` (skillCapsules.invoke-commit)
- Actualizar `scripts/skills/index.json`

### 6. Validación
- Compilar y probar invoke_commit
- Probar push_and_create_pr con --body y --body-file
