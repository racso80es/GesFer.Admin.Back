# Skill: Finalizar Git

**skill_id:** `finalizar-git`

## Objetivo

Centralizar todas las interacciones con Git necesarias para el cierre de una feature o fix: desde la rama de trabajo hasta **aceptar el PR en master**, **unificar** la rama, **eliminar la rama unificada** (local y opcionalmente remota) y **volver a master** con el repositorio limpio. Esta skill es consumible por la acción **finalize** (`SddIA/actions/finalize.md`) y por agentes que ejecuten el cierre del ciclo.

## Alcance

- **Pre-PR (rama de trabajo):** Push de la rama, creación del PR hacia master (la acción finalize orquesta esto; la skill puede describir los comandos de apoyo).
- **Post-PR (una vez el PR está aceptado/mergeado):** Checkout a master, pull de master, eliminación de la rama local ya fusionada, eliminación opcional de la rama remota, y comprobación del estado final.

## Especificación (Spec)

### Entradas

| Entrada | Tipo | Descripción |
| :--- | :--- | :--- |
| Rama actual | string | Nombre de la rama de trabajo (ej. `feat/nombre-feature` o `fix/nombre-fix`). |
| Persist | string | (Opcional) Ruta de la carpeta de la feature/fix (ej. `docs/features/<nombre_feature>/`) para trazabilidad. |
| Fase | enum | `pre_pr` (push y preparación de PR) o `post_pr` (post-merge: unificar local, limpiar, volver a master). |

### Salidas

| Salida | Descripción |
| :--- | :--- |
| Rama publicada | En fase `pre_pr`: rama pusheada a origin. |
| PR creado / referencia | En fase `pre_pr`: PR hacia master (o instrucciones para crearlo). |
| Master actualizado | En fase `post_pr`: repositorio local en `master` con los cambios mergeados. |
| Ramas limpias | En fase `post_pr`: rama local eliminada; opcionalmente rama remota eliminada. |

### Flujo de ejecución

#### Fase `pre_pr` (antes del merge a master)

1. Comprobar que la rama actual no es `master`.
2. Comprobar estado: cambios sin commitear → commit atómico con mensaje convencional.
3. Push de la rama: `git push origin <rama_actual>`.
4. Creación del PR hacia `master` (API del proveedor o instrucciones); descripción del PR debe enlazar a `{persist}` si existe.

#### Fase `post_pr` (después de aceptar/mergear el PR en el remoto)

1. Asegurarse de que el PR ya está mergeado en `master` (o `main`) en el remoto.
2. **Script recomendado:** Invocar `scripts/skills/Merge-To-Master-Cleanup.ps1` desde la raíz del repo (PowerShell):

   ```powershell
   .\scripts\skills\Merge-To-Master-Cleanup.ps1 -BranchName "<rama_actual>" -DeleteRemote
   ```

   Si no se indica `-BranchName`, se usa la rama actual. El script: hace checkout a master/main, ejecuta `git pull origin <troncal>`, elimina la rama local ya fusionada y, con `-DeleteRemote`, la rama remota.

3. **Alternativa manual:** Si el script no se usa, ejecutar en orden: `git checkout master` (o `main`), `git pull origin master`, `git branch -d <rama_actual>`, opcionalmente `git push origin --delete <rama_actual>`, y comprobar con `git status` y `git branch -vv`.

### Reglas (Ley GIT)

- **Nunca commit directo en master:** Todo el trabajo se hace en rama feat/ o fix/; el merge a master se hace vía PR en el remoto (o, en flujo local, vía `git merge` desde master después de certificar).
- **Mensajes convencionales:** Los commits deben seguir Conventional Commits (ej. `feat:`, `fix:`, `chore:`).
- **Pre-push:** Ejecutar validación local (build/tests) antes de push cuando lo exija el proceso (validacion.json pass antes de finalize).

### Integración con scripts

| Fase    | Script | Uso |
|--------|--------|-----|
| **pre_pr**  | `scripts/skills/Unificar-Rama.ps1` | Certificar rama antes del merge (build, documentación, commit). Desde la raíz: `.\scripts\skills\Unificar-Rama.ps1 -BranchName "<rama_actual>" -CommitMessage "chore: finalizar tarea"`. |
| **post_pr** | `scripts/skills/Merge-To-Master-Cleanup.ps1` | Tras aceptar el PR: posicionar en master/main, sincronizar y eliminar la rama mergeada (local y opcionalmente remota). Desde la raíz: `.\scripts\skills\Merge-To-Master-Cleanup.ps1 -BranchName "<rama_actual>" -DeleteRemote`. |

Si algún script no existe, los pasos de la skill se realizan con comandos git estándar.

### Consumidores

- **Acción finalize:** Usa esta skill para los pasos de Git (push, PR y, si aplica, pasos post-pr).
- **Agentes:** Tekton Developer, Finalizer / Release Agent (según `SddIA/actions/finalize.md`).

---
*Especificación del skill Finalizar Git. Artefacto obligatorio según contrato en `SddIA/skills/README.md`.*
