# Auditoría: carpeta `scripts/`

**Fecha:** 2026-02-19  
**Objetivo:** Analizar el contenido de las herramientas en `scripts/` para eliminar lo no útil, refinar lo que sí lo sea y aplicar en casos de uso (skill finalize).

---

## 1. Resumen ejecutivo

- **Salida principal:** La skill **finalize** (acción finalize + skill finalizar-git) hace uso del nuevo script **`scripts/skills/Merge-To-Master-Cleanup.ps1`**, que ejecuta la fase **post_pr**: posicionarse en master/main, sincronizar con origin y eliminar la rama ya mergeada (local y opcionalmente remota).
- **Recomendaciones:** Eliminar scripts deprecados; alinear Unificar-Rama.ps1 y documentación de ramas con `docs/features/` (Cúmulo); mantener scripts PowerShell como estándar en Windows (Ley 2).

---

## 2. Inventario y valoración

### 2.1 Raíz `scripts/`

| Script | Valoración | Uso / Nota |
|--------|------------|------------|
| `validate-services-and-health.ps1` | **Útil** | Health checks de ProductApi, AdminApi, ProductFront; opción -StartServices. Mantener. |
| `run-service-with-log.ps1` | **Útil** | Ejecuta servicios con log estructurado en `logs/services/`. Mantener. |
| `cerrar-procesos-servicios.ps1` | **Útil** | Cierra puertos y procesos dotnet/GesFer (evita MSB3027/MSB3021). Mantener. |
| `install-front-dependencies.ps1` | **Útil** | npm install en Product/Front y Admin/Front. Mantener. |
| `ejecutar-tests.ps1` | **Útil** | Tests Docker (frontend + backend). Mantener. |
| `audit_frontend_daily.py` | **Útil** | Auditoría frontend (términos prohibidos, @ts-ignore, alt, etc.); escribe en `docs/audits/`. Mantener. |
| `validate-pr.ps1.deprecated` | **Eliminar** | Deprecado. |
| `validate-pr.sh.deprecated` | **Eliminar** | Deprecado. |
| `validate-commit.ps1.deprecated` | **Eliminar** | Deprecado. |
| `validate-commit.sh.deprecated` | **Eliminar** | Deprecado. |
| `validate-commit.bat.deprecated` | **Eliminar** | Deprecado. |

### 2.2 `scripts/skills/`

| Script / artefacto | Valoración | Uso / Nota |
|--------------------|------------|------------|
| `Unificar-Rama.ps1` | **Refinar** | Certificación pre-merge (build, doc rama). Actualmente exige `docs/branches/$BranchName/OBJETIVO.md`; el proyecto canoniza `docs/features/<nombre>/` y `objectives.md` (Cúmulo). Refactor: aceptar también `docs/features/<slug>/objectives.md` o documentar la discrepancia. |
| **`Merge-To-Master-Cleanup.ps1`** | **Nuevo** | **Aplicado.** Fase post_pr de finalizar-git: checkout master/main, pull, eliminar rama local y opcionalmente remota. Invocado por la skill finalizar-git y la acción finalize. |
| `Invoke-Command.ps1` + `.json` | **Útil** | Wrapper de ejecución para IA (telemetría, logs en `docs/diagnostics/`). Mantener. |
| `pr-skill.sh` + `pr-skill.md` | **Mantener** | Pre-push / CI: token, compilación, documentación de rama, tests. Entorno: Bash (Git Bash en Windows). Documentación correcta; no hay versión PowerShell; mantener por uso en Husky y GitHub Actions. |
| `security-validation-skill.sh` | **Mantener** | Usado por pr-skill para bypass. Bash. |
| `commit-skill.sh` | **Revisar** | Bash; verificar si se usa en hooks o CI; si no, valorar eliminación o migración a PowerShell. |

### 2.3 `scripts/auditor/`

| Script | Valoración | Uso / Nota |
|--------|------------|------------|
| `process-token-manager.ps1` | **Útil** | Token de interacción (Validate, Generate, Revoke). Usado por pr-skill (vía .sh). Mantener. |
| `process-token-manager.sh` | **Útil** | Mismo rol en Bash; invocado por pr-skill.sh. Mantener. |

### 2.4 `scripts/Propuesta/`

| Contenido | Valoración | Nota |
|------------|------------|------|
| `PROPUESTA_*.md`, `0*-Propuesta-*.md` | **Documentación** | No son ejecutables. Mover a `docs/` (p. ej. `docs/proposals/`) o mantener como referencia; no eliminar sin criterio de negocio. |

---

## 3. Cambios realizados (aplicación)

1. **Nuevo script:** `scripts/skills/Merge-To-Master-Cleanup.ps1`
   - Parámetros: `-BranchName` (opcional), `-DeleteRemote`, `-MainBranch` (opcional; por defecto se detecta master/main).
   - Flujo: checkout a troncal → pull origin → branch -d → opcional push --delete.
   - Solo para **post-merge** (PR ya aceptado en remoto); no hace merge local (Ley GIT).

2. **Skill finalizar-git**
   - `SddIA/skills/finalizar-git.md`: fase post_pr documenta la invocación de `Merge-To-Master-Cleanup.ps1`; tabla de integración con scripts (pre_pr / post_pr).
   - `SddIA/skills/finalizar-git.json`: steps de post_pr actualizados; añadido `scripts.post_pr` y referencia al script en common_workflows.

3. **Acción finalize**
   - `SddIA/actions/finalize.md`: paso 7 (Post-PR) indica explícitamente el uso de `scripts/skills/Merge-To-Master-Cleanup.ps1`.

---

## 4. Recomendaciones pendientes

| Acción | Prioridad | Detalle |
|--------|-----------|---------|
| **Eliminar scripts .deprecated** | Hecho | Eliminados: `validate-pr.ps1/sh.deprecated`, `validate-commit.ps1/sh/bat.deprecated`. |
| **Refinar Unificar-Rama.ps1** | Media | Aceptar documentación en `docs/features/<slug>/objectives.md` además de `docs/branches/.../OBJETIVO.md`, o documentar convención dual en finalizar-git. |
| **Revisar commit-skill.sh** | Baja | Confirmar si está referenciado en hooks o CI; si no, archivar o eliminar. |
| **Propuesta/** | Baja | Decidir si mover a `docs/proposals/` o mantener en scripts como referencia. |

---

## 5. Referencias

- Skill finalizar-git: `SddIA/skills/finalizar-git.md`, `SddIA/skills/finalizar-git.json`
- Acción finalize: `SddIA/actions/finalize.md`
- Cúmulo (rutas): `SddIA/agents/cumulo.json` → paths.featurePath, docs/audits
- Ley GIT (AGENTS.md): no commit en master; merge a master vía PR.

---
*Auditoría generada en el marco del análisis de la carpeta scripts y la integración con la skill finalize.*
