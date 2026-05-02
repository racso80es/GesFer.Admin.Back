# Skill git-workspace-recon — Cápsula de implementación

**skillId:** git-workspace-recon  
**Ruta canónica:** Cúmulo `paths.skillCapsules["git-workspace-recon"]` (`scripts/skills/git-workspace-recon/`)

## Uso (humano)

Desde la raíz del repositorio:

```powershell
.\scripts\skills\git-workspace-recon\Git-Workspace-Recon.bat
.\scripts\skills\git-workspace-recon\Git-Workspace-Recon.bat --target-path "c:\Proyectos\GesFer.Admin.Back"
```

## Uso (agente / IA)

Invocar `git_workspace_recon.exe` (raíz de cápsula) con envelope JSON v2 por stdin (o `GESFER_CAPSULE_REQUEST`).

Ejemplo de request:

```json
{
  "meta": {
    "schema_version": "2.0",
    "entity_kind": "skill",
    "entity_id": "git-workspace-recon",
    "token": "Karma2Token..."
  },
  "request": {
    "target_path": "c:\\Proyectos\\GesFer.Admin.Back"
  }
}
```

## Salida

`result.status.entries` y `result.diffStat.files` contienen listas parseadas; `raw` conserva el texto completo (stdout+stderr) para diagnóstico.

