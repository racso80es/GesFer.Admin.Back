---
skill_id: git-workspace-recon
name: "Git Workspace Recon"
description: "Reconocimiento del workspace Git: status -s + diff --stat con salida JSON v2."
contract_ref: SddIA/skills/skills-contract.md
implementation_path_ref: paths.skillCapsules.git-workspace-recon
parameters:
  target_path:
    description: "Ruta opcional donde ejecutar Git (cwd). Si se omite, usa el directorio actual."
    required: false
rules:
  - "Captura stdout y stderr de git status -s y git diff --stat."
  - "Si la salida es vacía con exitCode 0, success=true."
  - "Si el comando falla (exitCode != 0), success=false con feedback error."
json_io_ref: SddIA/norms/capsule-json-io.md
---

# Skill: git-workspace-recon

## Objetivo

Ejecutar reconocimiento táctico del estado del repositorio:

- `git status -s`
- `git diff --stat`

y devolver un `result` estructurado para consumo por agentes, acciones y procesos.

## Entrada (request)

```json
{
  "target_path": "c:\\Proyectos\\GesFer.Admin.Back"
}
```

## Salida (result)

- `status.entries`: lista parseada de `git status -s`
- `diffStat.files`: lista parseada de `git diff --stat`

## Implementación

- **Rust**: `scripts/skills-rs/src/bin/git_workspace_recon.rs`
- **Cápsula**: `paths.skillCapsules.git-workspace-recon`
- **Ejecutable**: `git_workspace_recon.exe` en la raíz de la cápsula

