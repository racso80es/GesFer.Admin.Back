# Skill: Iniciar Rama

**skill_id:** `iniciar-rama`

## Objetivo

Encargarse del **inicio de una acción**: crear una rama nueva adecuada (feat/ o fix/) actualizada con master/main y posicionar el repositorio en ella. Consumible por la fase 0 del proceso **feature**, por el proceso **bug-fix** y por cualquier acción o agente que deba comenzar una tarea en una rama de trabajo. Ley GIT: no trabajar en master; todo el trabajo en feat/ o fix/.

## Alcance

- **Entrada:** Tipo de tarea (feat o fix) y nombre/slug de la rama (ej. mi-feature, correccion-timeout).
- **Salida:** Repositorio en la rama `feat/<slug>` o `fix/<slug>`, creada desde la rama troncal actualizada con origin (master o main).

## Especificación (Spec)

### Entradas

| Entrada | Tipo | Descripción |
| :--- | :--- | :--- |
| BranchType | enum | `feat` (funcionalidad) o `fix` (corrección). |
| BranchName | string | Slug del nombre de la rama (ej. auditoria-scripts, admin-back-login). Se formará feat/BranchName o fix/BranchName. |
| MainBranch | string | (Opcional) Rama troncal: `master` o `main`. Por defecto se detecta automáticamente. |
| SkipPull | bool | (Opcional) Si true, no ejecuta pull en la troncal (útil cuando ya está actualizada). |

### Salidas

| Salida | Descripción |
| :--- | :--- |
| Rama creada | Rama local `feat/<slug>` o `fix/<slug>` creada desde la troncal actualizada. |
| Working tree | Repositorio en esa rama, listo para commits y trabajo. |

### Flujo de ejecución

1. Normalizar el nombre de rama (slug sin espacios ni barras).
2. Si la rama ya existe: checkout a ella y actualizar con origin/master (merge); salir.
3. Si no existe:
   - `git fetch origin`
   - `git checkout master` (o main)
   - `git pull origin master` (o main), salvo SkipPull
   - `git checkout -b feat/<slug>` (o fix/<slug>)
4. Comprobar estado: `git status`, `git branch -vv`.

### Reglas (Ley GIT)

- **Nunca trabajar en master/main:** El inicio de una acción debe dejar el repo en una rama feat/ o fix/.
- **Troncal actualizada:** La nueva rama se crea desde la troncal ya actualizada con origin para evitar desvíos innecesarios.

### Integración con scripts

**Script recomendado:** `scripts/skills/Iniciar-Rama.ps1` (PowerShell). Desde la raíz del repo:

```powershell
.\scripts\skills\Iniciar-Rama.ps1 -BranchType feat -BranchName "mi-feature"
.\scripts\skills\Iniciar-Rama.ps1 -BranchType fix -BranchName "correccion-timeout"
```

Si la rama ya existe, el script hace checkout a ella y la actualiza con la troncal (merge).

### Consumidores

- **Proceso feature (fase 0):** Preparar entorno = crear rama feat/<nombre_feature> desde master actualizado. Ver `SddIA/process/feature.md`.
- **Proceso bug-fix:** Crear rama fix/<nombre_fix> desde master. Ver `SddIA/process/bug-fix-specialist.json`.
- **Agentes:** Tekton Developer, Arquitecto, Bug Fix Specialist (al iniciar una tarea).

---
*Especificación del skill Iniciar Rama. Artefacto obligatorio según contrato en `SddIA/skills/README.md`.*
