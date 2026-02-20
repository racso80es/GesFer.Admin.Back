# Evolution Log

Registro cronológico de cierres de tareas y PR. Formato por línea: `[YYYY-MM-DD] [rama] [Descripción breve.] [Estado].`

---

[2026-02-20] [feat/sddia-paths-cumulo] Rutas vía Cúmulo: Cúmulo ampliado con paths; actions, process, skills, agents, norms, tools y templates usan paths.*; norma paths-via-cumulo.md. Documentación en docs/features/sddia-ecosystem-independence/. [PR #17]
[2026-02-20] [feat/skills-encapsulation] Encapsulamiento de skills (patrón tools, feature skill.Token): Cúmulo paths skills, SddIA/skills por carpeta, cápsulas scripts/skills, skills-rs (Rust), normas #Skill/#Action/#Process, finalizar-git con Push-And-CreatePR (gh). Documentación en docs/features/skill.Token/. [PR #13]
[2026-02-20] [feat/skill.Token] Refactor: renombrar feature skills-encapsulation a skill.Token; docs en docs/features/skill.Token/. [PR #14]
[2026-02-20] [feat/sddia-ecosystem-independence] Independencia ecosistema SddIA: acción referencia contrato de skill; Tekton responsable de indicar e invocar implementación. Documentación en docs/features/sddia-ecosystem-independence/. [PR #15]
[2026-02-21] [feat/resolve-audit-debt] Resolución de deuda técnica de auditoría: compilación (DTOs), arquitectura (Inversión de dependencias, API desacoplada) y limpieza (src/tests borrado). Documentación en docs/features/resolve-audit-debt/. [PR pendiente]
[2026-02-18] [feat/correccion-hallazgos-auditoria] Corrección de hallazgos de auditoría: referencias Shared dentro del repo, build y tests pasan. Documentación en docs/features/correccion-hallazgos-auditoria/. [PR #3]
[2026-02-17] [feat/sddia-process-validate-docs] SddIA: procesos (feature, bug-fix), validate con validación git obligatoria, Cúmulo como fuente de paths, interfaz .md/.json. Documentación en docs/features/sddia-process-validate-docs/. [PR pendiente]
[2026-02-19] [feat/standardize-nomenclature] [Standardized project nomenclature to GesFer.Admin.Back.*] [Completed]
[2026-02-19] [feat/estandarizacion-gesfer-admin-back] Estandarización Admin Back: solo CRUD empresas, get/update empresa, logs y auditorías; Shared reubicado en Admin y eliminado; nomenclatura GesFer.Admin.*; Persistence renombrado a Repository. Documentación en docs/features/estandarizacion-gesfer-admin-back/. [PR pendiente]
[2026-02-19] [feat/estandarizacion-gesfer-admin-back] Gestión y uso de skills SddIA: contrato skills (.md+.json), skill finalizar-git (pre_pr/post_pr), integración en acción finalize. Documentación en docs/features/estandarizacion-gesfer-admin-back/ y SddIA/skills/. [PR pendiente]
[2026-02-19] [main] Auditoría carpeta scripts: script Merge-To-Master-Cleanup.ps1 (post_pr) integrado en skill finalizar-git y acción finalize; scripts deprecados eliminados; informe docs/audits/AUDITORIA_CARPETA_SCRIPTS_20260219.md. [Cierre documental en main]
[2026-02-19] [main] Skill iniciar-rama: script Iniciar-Rama.ps1 para crear rama feat/fix actualizada con master; integrada en proceso feature (fase 0) y bug-fix. Cierre en main. [Cierre documental en main]
[2026-02-19] [feat/tools-env-and-seeds] Herramientas Prepare-FullEnv (entorno Docker/API) e Invoke-MySqlSeeds (MySQL, migraciones EF, seeds Admin); contrato tools (JSON + feedback). Documentación en docs/features/prepare-full-env/ y docs/features/tools-env-and-seeds/. [Fin de tarea]
