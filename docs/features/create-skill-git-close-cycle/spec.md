---
feature_name: create-skill-git-close-cycle
artifact: spec
process: feature
---

## Especificación técnica

### Hito 1 — Skill `git-close-cycle`

| Campo | Valor |
| --- | --- |
| **skill_id** | `git-close-cycle` |
| **Request** | `target_branch` (string, **requerido**): rama local a eliminar tras el cierre. |
| **Comportamiento** | 1) Resolver troncal local `main` o `master` (primera existente). 2) `git checkout <troncal>`. 3) `git pull origin HEAD`. 4) `git fetch --prune`. 5) Si existe `refs/heads/<target_branch>`, `git branch -d <target_branch>`; si falla, `git branch -D <target_branch>`. Si la rama no existe localmente, éxito con aviso. |
| **Salida** | `result` con fases (checkout, pull, fetch, delete) y códigos de salida Git agregados en JSON. |

### Hito 2 — Acción `finalize-process`

- En disparadores tipo **«tarea finalizada»** con cierre **post-fusión en remoto**, el ejecutor debe **orquestar** como paso final de la secuencia documentada la skill **git-close-cycle** pasando **`target_branch`** igual al nombre de la **rama de trabajo** de la tarea (feat/ o fix/), una vez confirmada la integración en el troncal remoto.
- Las fases previas (sync remoto de la rama de trabajo, PR, validación, evolution si aplica) permanecen; **git-close-cycle** no sustituye a **git-sync-remote** / **git-create-pr**; complementa el cierre local **después** de la fusión.

### Cúmulo

- Añadir `paths.skillCapsules.git-close-cycle` → `./scripts/skills/git-close-cycle/`.
