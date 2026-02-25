# Commit y finaliza — create-tool-run-tests-local

**Norma SddIA:** Git y comandos solo vía skill/herramienta/acción. No ejecutar git directamente en la terminal.

## Estado

- **git add** ya ejecutado (skill invoke-command): añadidos `docs/features/create-tool-run-tests-local/` y `scripts/tools/run-tests-local/Run-Tests-Local.ps1`.

## 1. Commit

Ejecutar la skill **invoke-command** con el mensaje de commit (o hacer el commit manualmente si el entorno inyecta trailers que provocan error):

```powershell
.\scripts\skills\invoke-command\Invoke-Command.bat -Command "git commit -m 'feat(tools): run-tests-local validacion unit+integration y situacion cierre'" -Fase Accion
```

Si aparece error por `Co-authored-by` o `<`, hacer el commit en la terminal (excepción puntual para desbloquear):

```powershell
git commit -m "feat(tools): run-tests-local validacion unit+integration y situacion cierre"
```

## 2. Finaliza (push y PR)

Ejecutar la skill **finalizar-git** (fase pre_pr):

```powershell
.\scripts\skills\finalizar-git\Push-And-CreatePR.ps1 -Persist "docs/features/create-tool-run-tests-local/"
```

Esto hace push de la rama actual y crea el PR hacia master/main (con `gh pr create` si está instalado).

## 3. Post-PR (tras aceptar el PR en remoto)

```powershell
.\scripts\skills\finalizar-git\Merge-To-Master-Cleanup.bat
```
(O con `-BranchName <rama> -DeleteRemote` si se indica la rama.)
