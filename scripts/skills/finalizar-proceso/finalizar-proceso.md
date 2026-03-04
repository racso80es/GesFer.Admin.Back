# Skill FinalizarProceso — Cápsula de implementación

**skillId:** finalizar-proceso  
**Ruta canónica:** Cúmulo paths.skillCapsules["finalizar-proceso"] (scripts/skills/finalizar-proceso/)

## Fases

### pre_pr (antes del merge a master)
- **Certificar rama:** `Unificar-Rama.ps1 -BranchName "<rama_actual>" -CommitMessage "chore: finalizar tarea"` (build, documentación, commit).
- **Push y crear PR:** `Push-And-CreatePR.ps1 -Persist "docs/features/<nombre_feature>/"` — hace push y, si está instalado GitHub CLI (`gh`), ejecuta `gh pr create`; si no, muestra la URL para crear el PR manualmente.
- **Validar dependencia gh:** `gh --version` y `gh auth status` (debe estar logueado en github.com con scope `repo`).

### post_pr (finalizar proceso)
- **Punto de entrada:** `Finalizar-Proceso.ps1` o `Finalizar-Proceso.bat [rama]`.
- Por defecto elimina la rama local y la rama remota (`-DeleteRemote`). Use `-NoDeleteRemote` para no borrar en origin.
- Invoca internamente `Merge-To-Master-Cleanup.ps1`. Si existe `bin/merge_to_master_cleanup.exe` (Rust), `Merge-To-Master-Cleanup.bat` lo usa.
- Pasos: checkout a main, pull, eliminar rama local, eliminar rama remota (por defecto).

Definición: SddIA/skills/finalizar-proceso/spec.md y spec.json.
