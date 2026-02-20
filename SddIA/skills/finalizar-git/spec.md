# Skill: Finalizar Git

**skill_id:** `finalizar-git`

## Objetivo

Centralizar todas las interacciones con Git necesarias para el cierre de una feature o fix: desde la rama de trabajo hasta **aceptar el PR en master**, **unificar** la rama, **eliminar la rama unificada** (local y opcionalmente remota) y **volver a master** con el repositorio limpio. Esta skill es consumible por la acción **finalize** (paths.actionsPath/finalize.md, Cúmulo) y por agentes que ejecuten el cierre del ciclo.

## Alcance

- **Pre-PR (rama de trabajo):** Push de la rama, creación del PR hacia master (la acción finalize orquesta esto; la skill puede describir los comandos de apoyo).
- **Post-PR (una vez el PR está aceptado/mergeado):** Checkout a master, pull de master, eliminación de la rama local ya fusionada, eliminación opcional de la rama remota, y comprobación del estado final.

## Especificación (Spec)

### Entradas

| Entrada | Tipo | Descripción |
| :--- | :--- | :--- |
| Rama actual | string | Nombre de la rama de trabajo (ej. `feat/nombre-feature` o `fix/nombre-fix`). |
| Persist | string | (Opcional) Ruta de la carpeta de la feature/fix (ej. paths.featurePath/<nombre_feature>/) para trazabilidad. |
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
3. **Push y creación del PR:** ejecutar el componente **Push-And-CreatePR.ps1** de la cápsula:
   - Hace `git push origin <rama_actual>`.
   - Si **GitHub CLI (gh)** está instalado y autenticado, ejecuta `gh pr create --base master|main --head <rama> --title <título> --body "Documentación: <ruta Cúmulo>"`.
   - Si no hay `gh`, muestra la URL para crear el PR manualmente (GitHub) o instrucciones para otro proveedor.
4. La descripción del PR debe enlazar a la ruta de documentación (Cúmulo) si se pasa el parámetro `-Persist`.

#### Fase `post_pr` (después de aceptar/mergear el PR en el remoto)

1. Asegurarse de que el PR ya está mergeado en `master` (o `main`) en el remoto.
2. **Implementación:** Cápsula paths.skillCapsules[\"finalizar-git\"]: launcher `Merge-To-Master-Cleanup.bat` (o .ps1). Ejecutable por defecto en Rust (paths.skillsRustPath, Cúmulo) en bin/ si existe.

   Desde la raíz del repo:
   ```powershell
   .\scripts\skills\finalizar-git\Merge-To-Master-Cleanup.bat "<rama_actual>" -DeleteRemote
   # o .ps1: -BranchName "<rama_actual>" -DeleteRemote
   ```

3. **Alternativa manual:** Si el script no se usa, ejecutar en orden: `git checkout master` (o `main`), `git pull origin master`, `git branch -d <rama_actual>`, opcionalmente `git push origin --delete <rama_actual>`, y comprobar con `git status` y `git branch -vv`.

### Reglas (Ley GIT)

- **Nunca commit directo en master:** Todo el trabajo se hace en rama feat/ o fix/; el merge a master se hace vía PR en el remoto (o, en flujo local, vía `git merge` desde master después de certificar).
- **Mensajes convencionales:** Los commits deben seguir Conventional Commits (ej. `feat:`, `fix:`, `chore:`).
- **Pre-push:** Ejecutar validación local (build/tests) antes de push cuando lo exija el proceso (validacion.json pass antes de finalize).

### Integración con la cápsula

| Fase    | Componente en cápsula | Uso |
|--------|------------------------|-----|
| **pre_pr**  | Unificar-Rama.ps1 | Certificar rama (build, documentación, commit). |
| **pre_pr**  | **Push-And-CreatePR.ps1** | Push de la rama y **crear el PR** (GitHub CLI `gh pr create` si está disponible; si no, URL/instrucciones). Parámetros: `-BranchName`, `-Persist` (ruta paths.featurePath/...), `-Title` opcional. |
| **post_pr** | Merge-To-Master-Cleanup.bat (.exe en bin/ o .ps1) | Tras aceptar el PR: posicionar en master/main, sincronizar y eliminar la rama mergeada (local y opcionalmente remota). |

Ruta canónica de la cápsula: Cúmulo paths.skillCapsules[\"finalizar-git\"].

### Dependencia opcional: GitHub CLI (gh)

Para que **Push-And-CreatePR.ps1** cree el PR automáticamente, debe estar instalado y autenticado **GitHub CLI** (`gh`). Validación:

- `gh --version` — comprobar que está instalado.
- `gh auth status` — debe mostrar sesión activa en github.com con scope `repo`.

Si `gh` no está disponible, el script muestra la URL para crear el PR manualmente.

### Consumidores

- **Acción finalize:** Usa esta skill para los pasos de Git (push, PR y, si aplica, pasos post-pr).
- **Agentes:** Tekton Developer, Finalizer / Release Agent (según paths.actionsPath/finalize.md).

---
*Especificación del skill Finalizar Git. Definición en paths.skillsDefinitionPath/finalizar-git/ (contrato paths.skillsDefinitionPath/skills-contract.md).*
