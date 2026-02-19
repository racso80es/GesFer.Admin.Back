# Evolution Log (detalle)

Registro de cierres de tareas con resumen de alcance y referencia a documentación.

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
