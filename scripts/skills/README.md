# Skills (paths.skillsPath)

Este directorio es **paths.skillsPath** (Cúmulo, `SddIA/agents/cumulo.json`). Contiene el índice de skills con implementación ejecutable y las cápsulas por skill.

## Listado de skills

El listado canónico se obtiene de:

- **Índice (Cúmulo):** **paths.skillsIndexPath** — fichero `index.json` en la raíz de skills.
- **Fuente de verdad de rutas:** Cúmulo **paths.skillCapsules** (`SddIA/agents/cumulo.json`).

| skillId | Descripción breve | Launcher humano (opcional) | Ejecutable (agente) |
|---------|-------------------|----------------------------|---------------------|
| **invoke-command** | Interceptor de comandos. | `Invoke-Command.bat` | `invoke_command.exe` |
| **invoke-commit** | Commit con parámetros o JSON. | `Invoke-Commit.bat` | `invoke_commit.exe` |
| **git-workspace-recon** | Reconocimiento/validación de workspace (entorno limpio). | `Git-Workspace-Recon.bat` | `git_workspace_recon.exe` |
| **git-branch-manager** | Gestión de rama (crear/cambiar/validar aislamiento) para feat/fix. | `Git-Branch-Manager.bat` | `git_branch_manager.exe` |
| **git-save-snapshot** | Consolidación de hitos atómicos (snapshot/commit). | `Git-Save-Snapshot.bat` | `git_save_snapshot.exe` |
| **git-sync-remote** | Sincronización segura con remoto (publicación). | `Git-Sync-Remote.bat` | `git_sync_remote.exe` |
| **git-tactical-retreat** | Protocolo de emergencia ante fallos estructurales. | `Git-Tactical-Retreat.bat` | `git_tactical_retreat.exe` |
| **git-create-pr** | Creación de Pull Request. | `Git-Create-PR.bat` | `git_create_pr.exe` |

Cada cápsula tiene `manifest.json`, `.exe` en la **raíz** (contrato v2) y documentación `.md`. **La IA invoca el `.exe`** con JSON en stdin según **SddIA/norms/capsule-json-io.md**. Los `.bat` son solo atajos humanos.

## Implementación: Rust

Compilar y copiar: `scripts/skills-rs/install.ps1` (genera release y copia a cada cápsula).

## Definición vs implementación

- **Definición:** SddIA/skills/&lt;skill-id&gt;/ — paths.skillsDefinitionPath.
- **Implementación:** scripts/skills/&lt;skill-id&gt;/ — paths.skillCapsules[skill-id].

Contrato: `SddIA/skills/skills-contract.md`.
