# Alcance: uso actual de rutas de ficheros en SddIA

**Objetivo del análisis:** Identificar dónde se usan rutas literales (ej. `docs/features/`, `scripts/skills/`) frente a referencias vía Cúmulo (`paths.*`), para definir un proceso que unifique el patrón "rutas solo desde Cúmulo" en todo el ecosistema SddIA.

**Principio a aplicar (misma senda que ejecución):** Así como la ejecución de scripts no corresponde a SddIA (se referencia por contrato de skill; Tekton invoca), las **rutas locales de ficheros** no deberían ser literales en SddIA: han de resolverse desde un único contexto de información, el **agente Cúmulo**.

---

## 1. Lo que ya está en Cúmulo (paths)

En `SddIA/agents/cumulo.json` → `paths` están definidos:

| Clave | Valor actual | Uso |
|-------|--------------|-----|
| featurePath | ./docs/features/ | Carpeta de la tarea para features (Cúmulo) |
| fixPath, bugPath | ./docs/bugs/ | Carpeta de la tarea para fixes (Cúmulo) |
| logPath | ./docs/logs/ | Logs |
| toolsPath | ./scripts/tools/ | Raíz herramientas |
| toolsDefinitionPath | ./SddIA/tools/ | Definición tools |
| toolsIndexPath | ./scripts/tools/index.json | Índice tools |
| toolCapsules | { tool-id: "./scripts/tools/<id>/" } | Cápsulas tools |
| skillsPath | ./scripts/skills/ | Raíz skills |
| skillsDefinitionPath | ./SddIA/skills/ | Definición skills |
| skillsIndexPath | ./scripts/skills/index.json | Índice skills |
| skillCapsules | { skill-id: "./scripts/skills/<id>/" } | Cápsulas skills |

**Conclusión:** Features, fixes, logs, tools y skills ya tienen rutas canónicas en Cúmulo. Falta extender y usar Cúmulo para el resto de rutas que hoy son literales.

---

## 2. Rutas literales en SddIA (por categoría)

### 2.1 docs/ (documentación general)

| Ruta literal | Ubicaciones (ejemplos) | Debería ser |
|--------------|------------------------|-------------|
| docs/features/&lt;nombre_feature&gt;/ | finalize.md, feature.md, validate.md, execution.md, clarify.md, spec.md, finalizar-git spec, interaction-triggers.md, process README, bug-fix-specialist | paths.featurePath/&lt;nombre_feature&gt;/ (ya existe; **sustituir literal** por referencia) |
| docs/bugs/&lt;nombre_fix&gt;/ | bug-fix-specialist.json, interaction-triggers.md, process README | paths.fixPath/&lt;nombre_fix&gt;/ (ya existe) |
| docs/evolution/ | finalize.md, feature.md, cumulo instructions (Map [EVO]) | **Nuevo en Cúmulo:** paths.evolutionPath (ej. ./docs/evolution/) |
| docs/evolution/EVOLUTION_LOG.md | finalize.md, documentation skill | paths.evolutionPath + nombre fichero o paths.evolutionLogFile |
| docs/audits/ | finalize.md, validate.md, clarify.md, spec.md, planning.md, implementation.md, qa-judge, process-interaction, front/back auditor, performance-engineer, security-audit skill, templates | **Nuevo en Cúmulo:** paths.auditsPath (ej. ./docs/audits/) |
| docs/audits/ACCESS_LOG.md | Varios agents y actions | paths.auditsPath + "ACCESS_LOG.md" o paths.accessLogFile |
| docs/audits/validacion-*.json | validate.md | paths.auditsPath + convención nombre |
| docs/architecture/ & docs/infrastructure/ | cumulo instructions (Map [TEC]) | **Nuevo en Cúmulo:** paths.architecturePath, paths.infrastructurePath (o uno solo paths.techPath) |
| docs/DeudaTecnica/ | cumulo instructions (Map [DEBT]) | **Nuevo:** paths.debtPath |
| docs/tasks/ | cumulo instructions (Map [OPS]) | **Nuevo:** paths.tasksPath |
| docs/logs/ | ya en Cúmulo como logPath | — |

### 2.2 scripts/ (ejecutables e implementación)

| Ruta literal | Ubicaciones | Debería ser |
|--------------|-------------|-------------|
| scripts/skills/..., scripts/skills/&lt;skill-id&gt;/ | finalize.md, feature.md, bug-fix-specialist, tekton-developer, process-interaction, skills-contract, finalizar-git, invoke-command, iniciar-rama, interaction-triggers, create-tool | **Ya en Cúmulo:** paths.skillCapsules[skill-id] o paths.skillsPath. En SddIA **no** escribir la ruta literal; solo referenciar "paths.skillCapsules[...]" o "según Cúmulo". |
| scripts/skills-rs | skills-contract, finalizar-git, invoke-command, iniciar-rama, skills-contract.md | **Nuevo en Cúmulo (opcional):** paths.skillsRustPath (./scripts/skills-rs) para referencias desde contrato; o dejar en contrato como "implementación Rust" sin ruta y que Tekton/Cápsula lo resuelvan. |
| scripts/tools/..., scripts/tools/&lt;tool-id&gt;/ | tools-contract, create-tool, README tools | paths.toolCapsules[tool-id], paths.toolsPath (ya en Cúmulo; **sustituir literales** en SddIA). |
| scripts/tools-rs | tools-contract, security-engineer | **Nuevo (opcional):** paths.toolsRustPath |

### 2.3 SddIA/ (autoreferencia del dominio)

| Ruta literal | Ubicaciones | Nota |
|--------------|-------------|------|
| SddIA/actions/ | finalize.md, cumulo instructions, interaction-triggers, finalizar-git | Podría ser paths.actionsPath en Cúmulo para homogeneidad, o aceptarse como "dominio SddIA" (nombre de contexto, no ruta de repo). |
| SddIA/process/ | feature.md, interaction-triggers, process README, cumulo | Idem: paths.processPath o convención de dominio. |
| SddIA/skills/..., SddIA/norms/ | Múltiples | skillsDefinitionPath y "SddIA/norms/" en instructions: podría paths.normsPath. |
| SddIA/agents/cumulo.json | constitution, AGENTS.md | Referencia al agente; puede quedar como identificador canónico del agente (no necesariamente una ruta de disco en SddIA). |

**Criterio propuesto:** Dentro de SddIA, las rutas que son "dentro del propio árbol SddIA" (actions, process, norms, agents) pueden seguir como nombres de contexto (ej. "SddIA/actions/") **o** centralizarse en Cúmulo como paths.actionsPath, paths.processPath, paths.normsPath para que **toda** ruta pase por Cúmulo. Lo segundo es más coherente con "Cúmulo como única fuente de contexto de información para rutas".

---

## 3. Resumen por tipo de artefacto

| Artefacto | Rutas literales hoy | Acción recomendada |
|-----------|---------------------|---------------------|
| **SddIA/actions/** (finalize, validate, execution, spec, clarify, planning, implementation) | docs/features/..., docs/evolution/..., docs/audits/..., scripts/skills/... | Sustituir por paths.featurePath, paths.evolutionPath, paths.auditsPath, paths.skillCapsules; Cúmulo ampliado con evolutionPath, auditsPath (y opc. accessLogFile, evolutionLogFile). |
| **SddIA/process/** (feature.md, bug-fix-specialist.json, create-tool) | docs/features/..., docs/bugs/..., scripts/skills/..., scripts/tools/... | Idem: solo referencias a paths.*; procesos obtienen la ruta de la tarea y rutas de logs/audits desde Cúmulo. |
| **SddIA/skills/** (specs, skills-contract) | scripts/skills-rs, scripts/skills/..., paths.skillCapsules ya usado en parte | Eliminar literales scripts/skills/...; solo paths.skillCapsules, paths.skillsDefinitionPath; opcional paths.skillsRustPath. |
| **SddIA/agents/** (tekton, qa-judge, auditor/*, performance, security-engineer) | docs/audits/..., scripts/skills/... | Referencias a paths.auditsPath, paths.skillCapsules; sin rutas literales. |
| **SddIA/norms/** (interaction-triggers) | docs/features/..., docs/bugs/..., SddIA/... | Usar paths.featurePath, paths.fixPath; list_source puede ser "paths.actionsPath", "paths.processPath" si se añaden. |
| **SddIA/agents/cumulo.json** (instructions) | Map [EVO] -> docs/evolution/, Map [AUD] -> docs/audits/, Map [TEC], [DEBT], [OPS] con literales | Ampliar `paths` con evolutionPath, auditsPath, (architecturePath, debtPath, tasksPath); instructions usar paths.* en lugar de "docs/...". |
| **SddIA/tools/** (tools-contract, README) | scripts/tools/..., scripts/tools-rs | paths.toolsPath, paths.toolCapsules, opcional paths.toolsRustPath. |
| **SddIA/constitution.json** | Mención a docs/ y SddIA/agents/cumulo.json | Mantener "consult Cúmulo"; no necesita rutas literales. |
| **AGENTS.md** | paths.featurePath, etc. (ya correcto) | Revisar que no queden literales docs/features/ o scripts/; referencias a Cúmulo ya están. |

---

## 4. Propuesta de ampliación de Cúmulo (paths)

Añadir en `SddIA/agents/cumulo.json` → `paths`:

| Nueva clave | Valor propuesto | Uso |
|-------------|-----------------|-----|
| evolutionPath | ./docs/evolution/ | Evolution Logs, EVOLUTION_LOG.md |
| auditsPath | ./docs/audits/ | Auditorías, ACCESS_LOG.md, informes validación |
| architecturePath | ./docs/architecture/ | Map [TEC] (opcional: unificar con infrastructure) |
| infrastructurePath | ./docs/infrastructure/ | Map [TEC] |
| debtPath | ./docs/DeudaTecnica/ | Map [DEBT] |
| tasksPath | ./docs/tasks/ | Map [OPS] |
| actionsPath | ./SddIA/actions/ | Listado acciones (homogeneidad) |
| processPath | ./SddIA/process/ | Listado procesos |
| normsPath | ./SddIA/norms/ | Normas, interaction-triggers |
| skillsRustPath | ./scripts/skills-rs | Opcional: contrato skills |
| toolsRustPath | ./scripts/tools-rs | Opcional: contrato tools |

Y en `instructions`, sustituir toda ruta literal por la clave correspondiente (ej. "Map [EVO] -> paths.evolutionPath", "Map [AUD] -> paths.auditsPath").

---

## 5. Alcance del proceso futuro

1. **Ampliar Cúmulo** con las claves de paths que faltan (evolution, audits, tech, debt, tasks, y opcionalmente actions, process, norms, *RustPath).
2. **SddIA sin rutas literales:** En actions, process, skills, agents, norms, tools y constitution, sustituir cualquier ruta literal (docs/..., scripts/...) por la referencia a Cúmulo (paths.&lt;clave&gt; o paths.&lt;clave&gt;[id]). Excepción aceptable: el propio cumulo.json contiene los valores de paths (ahí sí son literales por ser la fuente de verdad).
3. **Norma explícita:** "En SddIA no se escriben rutas de ficheros literales; toda ruta se obtiene del agente Cúmulo (SddIA/agents/cumulo.json → paths). El ejecutor (Tekton u otro) resuelve la ruta real consultando Cúmulo."
4. **Documentar en SddIA/norms** (o en la feature sddia-ecosystem-independence / nueva feature) el principio "rutas vía Cúmulo" y la lista de claves paths disponibles.

Con este alcance, el siguiente paso sería definir la feature/proceso (objetivos, spec, fases) para implementar los cambios de forma ordenada.
