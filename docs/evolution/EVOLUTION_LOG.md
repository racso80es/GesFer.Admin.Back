# Evolution Log (detalle)

Registro de cierres de tareas con resumen de alcance y referencia a documentación.

---

## 2026-02-20 — Encapsulamiento de skills (feature skill.Token, rama feat/skills-encapsulation)

**Rama:** feat/skills-encapsulation.

**Alcance:**
- **Cúmulo:** paths skillsPath, skillsDefinitionPath, skillsIndexPath, skillCapsules (iniciar-rama, finalizar-git, invoke-command).
- **SddIA/skills:** contrato v1.1 (Rust por defecto), definición por carpeta \<skill-id\>/ con spec.md y spec.json; skills-contract.md.
- **scripts/skills:** índice index.json y cápsulas con manifest, .bat, .ps1, doc, bin/ (ejecutables Rust).
- **scripts/skills-rs:** binarios iniciar_rama, merge_to_master_cleanup, invoke_command; install.ps1.
- **Normas de interacción:** SddIA/norms/ (interaction-triggers.md|.json) y disparadores #Skill, #Action, #Process; AGENTS.norms.md; .cursor/rules para sugerencias.
- **Agents y constitution:** referencias a cápsulas y Cúmulo; paths_ref en constitution.

**Referencias:**
- `docs/features/skill.Token/objectives.md` — Objetivo y alcance.
- `docs/features/skill.Token/spec.md` — Especificación.
- `docs/features/skill.Token/validacion.json` — Validación.
- `AGENTS.norms.md` — Tabla de disparadores.

---

## 2026-02-20 — Independencia de ecosistema SddIA (feat/sddia-ecosystem-independence)

**Rama:** feat/sddia-ecosystem-independence.

**Alcance:**
- **Norma:** En la acción solo se referencia el **contrato** de la skill (SddIA/skills/\<skill-id\>/). El agente **Tekton** es responsable de indicar e invocar la **implementación** (resolver cápsula vía Cúmulo, ejecutar launcher).
- **Actions/process:** Referencian skill por contrato (paths.skillsDefinitionPath); sin rutas a scripts ni paths.skillCapsules en la acción.
- **Tekton:** Definido como ejecutor que resuelve implementación (Cúmulo) e invoca launcher según contrato.
- **Pendiente:** Aplicar cambios en SddIA/actions (finalize, validate, execution), process (feature.md), agents (Tekton), norms.

**Referencias:**
- `docs/features/sddia-ecosystem-independence/objectives.md` — Objetivo y alcance.
- `docs/features/sddia-ecosystem-independence/spec.md` — Especificación (acción→contrato, Tekton→implementación).
- `docs/features/sddia-ecosystem-independence/validacion.json` — Validación.

---

## 2026-02-21 — Resolución de Deuda Técnica (feat/resolve-audit-debt)

**Rama:** feat/resolve-audit-debt.

**Alcance:**
- **Compilación:** Creación de DTOs faltantes en `Application/DTOs/Logs/` para solucionar errores de build en `LogController`.
- **Arquitectura Limpia:**
  - Definición de `IApplicationDbContext` e interfaces de servicios (`IAdminAuthService`, etc.) en `Application/Common/Interfaces`.
  - Implementación de estas interfaces en `Infrastructure`.
  - Inversión de dependencias: `Application` ya no referencia a `Infrastructure`; `Infrastructure` referencia a `Application`.
  - Desacoplamiento de API: `LogController` y `AdminAuthController` dependen de interfaces, no de implementaciones concretas.
- **Limpieza:** Eliminación de carpeta legacy `src/tests`.

**Referencias:**
- `docs/features/resolve-audit-debt/objectives.md` — Objetivo y alcance.
- `docs/features/resolve-audit-debt/spec.md` — Especificación técnica.
- `docs/features/resolve-audit-debt/plan.md` — Plan de implementación.
- `docs/features/resolve-audit-debt/implementation.md` — Detalle de cambios en código.
- `docs/features/resolve-audit-debt/execution.json` — Registro de ejecución.
- `docs/features/resolve-audit-debt/validacion.json` — Resultado de validación.

---

## 2026-02-18 — Corrección de hallazgos de auditoría

**Rama:** feat/correccion-hallazgos-auditoria.

**Alcance:**
- Corrección del hallazgo bloqueante de `docs/audits/validacion-main-20260217.json`: build fallaba por referencias a Shared fuera del repositorio.
- Referencias actualizadas en `GesFer.Domain.csproj`, `GesFer.Infrastructure.csproj` y `GesFer.Product.sln` para usar `src/Shared/Back/` dentro del repo.
- Build y tests (108) pasan correctamente.

**Referencias:**
- `docs/features/correccion-hallazgos-auditoria/objectives.md` — Objetivo y plan de corrección.
- `docs/features/correccion-hallazgos-auditoria/audit-hallazgos.json` — Inventario de hallazgos y estado.

---

## 2026-02-17 — SddIA procesos, validate y documentación

**Rama:** feat/sddia-process-validate-docs.

**Alcance:**
- Documentación inicial del proyecto aislado: `Objetivos.md`, `README.md`.
- **SddIA/process/:** Definición de procesos de tarea (feature, bug-fix). `feature.md` y `bug-fix-specialist.json` movidos desde actions/ y agents/; índice en `SddIA/process/README.md`. Interfaz de procesos (.md y .json) en Cúmulo y en agentes.
- **AGENTS.md:** Cúmulo como única fuente de rutas (paths); sección "Inicio de tarea" con tabla de procesos; norma de interfaz para procesos; rol BUG-FIX.
- Eliminación de referencias a knowledge-architect; sustitución por Cúmulo (`SddIA/agents/cumulo.json`).
- **Acción validate:** Ampliada para incluir siempre validación de cambios git (diff frente a rama base); modo sin documentación con persistencia en `docs/audits/`. Estructura de informe con `git_changes` obligatoria.
- Acciones **execution**, **validate**, **finalize** creadas/refinadas en `SddIA/actions/`.
- Validación ejecutada (modo sin documentación): informe en `docs/audits/validacion-main-20260217.json` (build falla por dependencias externas Shared; análisis de cambios PR incluido).

**Referencias:**
- `docs/features/sddia-process-validate-docs/objectives.md` — Objetivo y alcance de esta tarea.
- `docs/audits/validacion-main-20260217.json` — Informe de validación (modo pr_analysis).
- `SddIA/process/README.md` — Índice de procesos.
- `SddIA/actions/validate.md` — Definición de la acción validate (validación git siempre).
- `AGENTS.md` — Inicio de tarea y roles.
## [2026-02-19] Estandarización de Nomenclatura (feat/standardize-nomenclature)
**Alcance**: Renombrado de solución, proyectos, carpetas y namespaces a `GesFer.Admin.Back`.
**Resultado**: Compilación y tests exitosos.
**Referencia**: [docs/features/standardize-nomenclature/objectives.md](docs/features/standardize-nomenclature/objectives.md)

---

## 2026-02-19 — Estandarización GesFer.Admin.Back (feat/estandarizacion-gesfer-admin-back)

**Rama:** feat/estandarizacion-gesfer-admin-back.

**Alcance:**
- Alcance Admin limitado a: CRUD empresas, get/update empresa concreta, logs y auditorías. Eliminados Dashboard, ProductApiClient, DashboardSummaryDto y tests asociados; eliminada carpeta GesFer.Product.UnitTests.
- Directorio Shared reubicado en Admin: entidades, value objects y servicios en GesFer.Admin.Domain; SequentialGuidValueGenerator y DbContextExtensions en GesFer.Admin.Infrastructure.Repository. Eliminado directorio src/Shared.
- Nomenclatura unificada a GesFer.Admin.* (Domain, Infrastructure, Application, Api). Namespace Persistence renombrado a Repository.
- Solución actualizada con rutas reales de proyectos; tests unitarios e integración (70) pasan.

**Referencias:**
- `docs/features/estandarizacion-gesfer-admin-back/objectives.md` — Objetivo y alcance.
- `docs/features/estandarizacion-gesfer-admin-back/spec.md` — Especificación técnica.
- `docs/features/estandarizacion-gesfer-admin-back/validacion.json` — Resultado de validación.

---

## 2026-02-19 — Gestión y uso de skills SddIA (feat/estandarizacion-gesfer-admin-back)

**Rama:** feat/estandarizacion-gesfer-admin-back.

**Alcance:**
- **Contrato de skills:** `SddIA/skills/README.md` y `SddIA/skills/skills-contract.json`: todo skill debe disponer de artefacto `.md` (documentación) y `.json` (metadatos); consumibles por acciones y agentes.
- **Skill finalizar-git:** `SddIA/skills/finalizar-git.md` y `SddIA/skills/finalizar-git.json`. Centraliza interacciones Git: fase pre_pr (push, creación PR a master) y fase post_pr (checkout master, pull, eliminar rama local/remota). Especificación aplicada (entradas, salidas, flujo, reglas).
- **Integración en finalize:** La acción `SddIA/actions/finalize.md` referencia y utiliza la skill `finalizar-git` para los pasos Git de cierre; skill declarada como obligatoria en la tabla del agente responsable.

**Referencias:**
- `SddIA/skills/README.md` — Contrato e índice de skills.
- `SddIA/skills/finalizar-git.md` — Especificación de la skill finalizar-git.
- `SddIA/actions/finalize.md` — Acción finalize (usa skill finalizar-git).

---

## 2026-02-19 — Auditoría carpeta scripts y script post-merge (main)

**Rama:** main.

**Alcance:**
- **Auditoría de `scripts/`:** Inventario y valoración de todos los scripts (raíz, skills/, auditor/, Propuesta/). Eliminados los scripts deprecados: validate-pr.ps1/sh.deprecated, validate-commit.ps1/sh/bat.deprecated.
- **Nuevo script:** `scripts/skills/Merge-To-Master-Cleanup.ps1`. Ejecuta la fase post_pr de finalizar-git: checkout a master/main, pull origin, eliminar rama local ya mergeada y opcionalmente remota (-DeleteRemote). Solo post-merge (PR ya aceptado en remoto); Ley GIT respetada.
- **Integración en finalize:** La skill `finalizar-git` (md y json) y la acción `finalize.md` referencian e invocan Merge-To-Master-Cleanup.ps1 para la fase post_pr. Tabla de scripts en finalizar-git: pre_pr → Unificar-Rama.ps1, post_pr → Merge-To-Master-Cleanup.ps1.
- **Documentación:** `docs/audits/AUDITORIA_CARPETA_SCRIPTS_20260219.md` con recomendaciones (refinar Unificar-Rama.ps1 con docs/features/, revisar commit-skill.sh).

**Referencias:**
- `docs/audits/AUDITORIA_CARPETA_SCRIPTS_20260219.md` — Informe de auditoría.
- `scripts/skills/Merge-To-Master-Cleanup.ps1` — Script de cierre post-merge.
- `SddIA/skills/finalizar-git.md`, `SddIA/actions/finalize.md` — Integración del script.

---

## 2026-02-19 — Skill iniciar-rama (main)

**Rama:** main.

**Alcance:**
- **Skill iniciar-rama:** `SddIA/skills/iniciar-rama.md` y `SddIA/skills/iniciar-rama.json`. Objetivo: inicio de una acción en una rama nueva (feat/ o fix/) actualizada con master/main.
- **Script:** `scripts/skills/Iniciar-Rama.ps1`. Parámetros: BranchType (feat|fix), BranchName (slug), MainBranch (opcional), SkipPull (opcional). Flujo: fetch, checkout troncal, pull, checkout -b; si la rama existe, checkout y merge con troncal.
- **Integración:** Proceso feature (fase 0) y proceso bug-fix referencian la skill y el script. Bug Fix Specialist añade skill iniciar-rama. Listado en `SddIA/skills/README.md`.

**Referencias:**
- `SddIA/skills/iniciar-rama.md`, `SddIA/skills/iniciar-rama.json` — Definición del skill.
- `scripts/skills/Iniciar-Rama.ps1` — Script de inicio de rama.
- `SddIA/process/feature.md`, `SddIA/process/bug-fix-specialist.json` — Consumidores.

---

## 2026-02-19 — Herramientas entorno y seeds (feat/tools-env-and-seeds)

**Rama:** feat/tools-env-and-seeds.

**Alcance:**
- **Prepare-FullEnv:** Cápsula **paths.toolCapsules['prepare-full-env']** (Cúmulo): script .ps1, prepare-env.json, prepare-env.md, manifest.json, opcional bin/. Launcher wrapper en **paths.toolsPath** (Prepare-FullEnv.bat). Levanta Docker (gesfer-db, cache, adminer), espera MySQL y opcionalmente la Admin API y clientes. Salida JSON y feedback por fases (contrato tools).
- **Contrato de herramientas:** `SddIA/tools/tools-contract.json` y `tools-contract.md`. Define salida JSON obligatoria (toolId, exitCode, success, timestamp, message, feedback[], data) y reglas de feedback adecuado (fases, niveles info/warning/error).
- **Invoke-MySqlSeeds:** Cápsula **paths.toolCapsules['invoke-mysql-seeds']** (Cúmulo): script .ps1, mysql-seeds-config.json, mysql-seeds.md, manifest.json, opcional bin/. Launcher wrapper en **paths.toolsPath** (Invoke-MySqlSeeds.bat). Comprueba MySQL (contenedor gesfer_db), ejecuta `dotnet ef database update` y los seeds de Admin (RUN_SEEDS_ONLY=1 en la API). Modo RUN_SEEDS_ONLY en `src/Api/Program.cs`.

**Referencias:**
- **paths.featurePath** prepare-full-env — Especificación Prepare-FullEnv.
- **paths.featurePath** tools-env-and-seeds — objectives.md.
- `SddIA/tools/tools-contract.json` — Contrato de herramientas.
- Rutas de herramientas (Cúmulo): **paths.toolsPath**, **paths.toolCapsules** (`SddIA/agents/cumulo.json`).
