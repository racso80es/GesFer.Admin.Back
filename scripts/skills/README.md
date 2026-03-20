# Skills (paths.skillsPath)

Este directorio es **paths.skillsPath** (Cúmulo, `SddIA/agents/cumulo.json`). Contiene el índice de skills con implementación ejecutable y las cápsulas por skill.

## Listado de skills

El listado canónico se obtiene de:

- **Índice (Cúmulo):** **paths.skillsIndexPath** — fichero `index.json` en la raíz de skills.
- **Fuente de verdad de rutas:** Cúmulo **paths.skillCapsules** (`SddIA/agents/cumulo.json`).

| skillId | Descripción breve | Launcher humano (opcional) | Ejecutable (agente) |
|---------|-------------------|----------------------------|---------------------|
| **iniciar-rama** | Crea rama feat/ o fix/ actualizada con troncal. | `iniciar-rama/Iniciar-Rama.bat` | `iniciar_rama.exe` |
| **finalizar-git** | Pre-PR: push + PR; post-PR: limpieza de rama. | `Push-And-CreatePR.bat`, `Merge-To-Master-Cleanup.bat` | `push_and_create_pr.exe`, `merge_to_master_cleanup.exe` |
| **invoke-command** | Interceptor de comandos. | `Invoke-Command.bat` | `invoke_command.exe` |
| **invoke-commit** | Commit con parámetros o JSON. | `Invoke-Commit.bat` | `invoke_commit.exe` |

Cada cápsula tiene `manifest.json`, `.exe` en la **raíz** (contrato v2) y documentación `.md`. **La IA invoca el `.exe`** con JSON en stdin según **SddIA/norms/capsule-json-io.md**. Los `.bat` son solo atajos humanos.

## Implementación: Rust

Compilar y copiar: `scripts/skills-rs/install.ps1` (genera release y copia a cada cápsula).

## Definición vs implementación

- **Definición:** SddIA/skills/&lt;skill-id&gt;/ — paths.skillsDefinitionPath.
- **Implementación:** scripts/skills/&lt;skill-id&gt;/ — paths.skillCapsules[skill-id].

Contrato: `SddIA/skills/skills-contract.md`.
