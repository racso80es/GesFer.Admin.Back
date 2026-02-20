# Skill finalizar-git — Cápsula de implementación

**skillId:** finalizar-git  
**Ruta canónica:** Cúmulo paths.skillCapsules["finalizar-git"] (scripts/skills/finalizar-git/)

## Fases

### pre_pr (antes del merge a master)
- Script: `Unificar-Rama.ps1 -BranchName "<rama_actual>" -CommitMessage "chore: finalizar tarea"`
- Certificar rama (build, documentación, commit), push y crear PR.

### post_pr (tras aceptar el PR en remoto)
- Launcher: `Merge-To-Master-Cleanup.bat` o `Merge-To-Master-Cleanup.ps1 -BranchName "<rama>" -DeleteRemote`
- Si existe `bin/merge_to_master_cleanup.exe` (Rust), el .bat lo invoca; si no, usa el .ps1.
- Checkout a master, pull, eliminar rama local (y opcionalmente remota).

Definición: SddIA/skills/finalizar-git/spec.md y spec.json.
