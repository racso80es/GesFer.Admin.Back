# Guía para repetir el proceso: rutas vía Cúmulo (remanente)

**Objetivo:** En próximos cambios, usar esta guía para sustituir las rutas literales restantes en SddIA por referencias a Cúmulo (paths.*). Norma: SddIA/norms/paths-via-cumulo.md. Fuente: SddIA/agents/cumulo.json → paths.

---

## 1. Tabla de sustitución

| Literal | Sustituir por |
|---------|----------------|
| docs/features/\<nombre_feature\>/ | paths.featurePath/\<nombre_feature\>/ o {persist} |
| docs/bugs/\<nombre_fix\>/ | paths.fixPath/\<nombre_fix\>/ |
| docs/evolution/, docs/evolution/EVOLUTION_LOG.md | paths.evolutionPath; paths.evolutionPath + paths.evolutionLogFile |
| docs/audits/, docs/audits/ACCESS_LOG.md | paths.auditsPath; paths.auditsPath + paths.accessLogFile |
| SddIA/actions/ | paths.actionsPath |
| SddIA/process/ | paths.processPath |
| SddIA/skills/... | paths.skillsDefinitionPath/\<skill-id\>/ |
| scripts/skills/..., scripts/skills/\<id\>/ | paths.skillCapsules[skill-id] o paths.skillsPath; Tekton invoca según contrato |
| scripts/skills-rs | paths.skillsRustPath |
| scripts/tools/..., scripts/tools/\<id\>/ | paths.toolCapsules[tool-id], paths.toolsPath |
| scripts/tools-rs | paths.toolsRustPath |
| scripts/tools/index.json | paths.toolsIndexPath |

---

## 2. Archivos pendientes

**Actions:** validate.md, execution.md, clarify.md, spec.md, planning.md, implementation.md — sustituir docs/audits/, docs/features/, docs/bugs/, SddIA/agents/ por paths.*.

**Process:** bug-fix-specialist.json (fixPath literal, scripts/skills/, docs/bugs/), create-tool.md y create-tool.json (scripts/tools/, SddIA/tools/), README.md (docs/features/, docs/bugs/).

**Skills:** skills-contract (scripts/skills-rs, scripts/skills/), finalizar-git (docs/features/, cápsula), invoke-command e iniciar-rama (scripts/skills-rs), security-audit y documentation (docs/audits/, docs/evolution/).

**Agents:** tekton-developer.json (scripts/skills/invoke-command, SddIA/skills/), qa-judge (docs/audits/), auditor/* (docs/audits/), performance-engineer (docs/audits/).

**Tools:** tools-contract, README — scripts/tools/, scripts/tools-rs → paths.toolsPath, paths.toolCapsules, paths.toolsRustPath.

**Templates:** spec-template.md — docs/audits/ACCESS_LOG.md → paths.auditsPath + paths.accessLogFile.

---

## 3. Orden recomendado

1. Actions (validate, execution, clarify, spec, planning, implementation).  
2. Process (bug-fix-specialist, create-tool, README).  
3. Skills (contract, finalizar-git, invoke-command, iniciar-rama, security-audit, documentation).  
4. Agents (tekton, qa-judge, auditor, performance).  
5. Tools (contract, README).  
6. Templates (spec-template).

---

## 4. Reglas

- No cambiar valores en cumulo.json (ahí los literales son SSOT).  
- En SddIA solo referencias paths.* o "según Cúmulo / skill".  
- Tras cada bloque, comprobar que las claves usadas existen en Cúmulo.
