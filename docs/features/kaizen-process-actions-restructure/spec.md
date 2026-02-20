# SPEC: Restructuración process y actions al patrón skills/tools

**Feature:** kaizen-process-actions-restructure  
**Ruta (Cúmulo):** paths.featurePath/kaizen-process-actions-restructure  
**Origen:** objectives.md + SddIA/process/analisis-restructuracion-patron-skills-tools.md

---

## 1. Contexto y objetivo

Alinear **process** y **actions** con el patrón ya usado en **skills** y **tools**: contrato explícito en la raíz del ámbito, **carpeta por ítem** y **spec.md + spec.json** por ítem. Esto permite descubribilidad uniforme, referencias vía Cúmulo coherentes (paths.processPath/<process-id>/, paths.actionsPath/<action-id>/) y cumplimiento de contratos verificable.

## 2. Alcance técnico

### 2.1 Process

- **Añadir en SddIA/process/:** `process-contract.json` y `process-contract.md` (definición de definition_artefacts, process_interface_compliance, constraints, consumers).
- **Crear carpetas:** feature/, bug-fix/, refactorization/, create-tool/. En cada una: `spec.md` (contenido migrado desde el .md o .json actual) y `spec.json` (process_id, name, description, phases, persist_ref, related_actions, contract_ref).
- **Migrar:** feature.md → feature/spec.md + feature/spec.json; refactorization.md → refactorization/spec.md + refactorization/spec.json; create-tool.md + create-tool.json → create-tool/spec.md + create-tool/spec.json; bug-fix-specialist.json → bug-fix/spec.json + bug-fix/spec.md (especificación legible).
- **Eliminar** tras migración: feature.md, refactorization.md, create-tool.md, create-tool.json, bug-fix-specialist.json en la raíz de process.
- **README.md:** Actualizar para listar procesos como paths.processPath/<process-id>/ y referenciar process-contract.

### 2.2 Actions

- **Añadir en SddIA/actions/:** `actions-contract.json` y `actions-contract.md` (definition_artefacts, action_id, inputs/outputs, constraints, consumers).
- **Crear carpetas:** spec/, clarify/, planning/, implementation/, execution/, validate/, finalize/, sddia-difusion/. En cada una: `spec.md` (contenido migrado del .md actual) y `spec.json` (action_id, name, purpose, inputs, outputs, flow_steps, related_processes).
- **Migrar:** Cada <action>.md → <action>/spec.md; generar <action>/spec.json a partir del contenido.
- **Eliminar** tras migración: los .md sueltos en la raíz de actions.
- **README.md (opcional):** Crear o actualizar índice de acciones con rutas paths.actionsPath/<action-id>/.

### 2.3 Referencias y consumidores

- **interaction-triggers.md (#Process, #Action):** Listado y ejemplos usando paths.processPath/<process-id>/ y paths.actionsPath/<action-id>/ (convención: punto de entrada spec en cada carpeta).
- **AGENTS.md, process README, feature/refactorization/create-tool/bug-fix:** Enlaces a definiciones por carpeta.
- **Cúmulo:** paths.processPath y paths.actionsPath siguen apuntando a ./SddIA/process/ y ./SddIA/actions/; no cambian; la convención de “definición del ítem” pasa a ser la carpeta <id>/ con spec.
- **.cursor/rules (sddia-difusion):** Tras el cambio, las reglas que citan paths.processPath/feature.md deben citar paths.processPath/feature/ (o feature/spec.md).

## 3. Criterios de aceptación

- [ ] Existen process-contract.json y process-contract.md en SddIA/process/.
- [ ] Existen actions-contract.json y actions-contract.md en SddIA/actions/.
- [ ] Cada proceso tiene carpeta <process-id>/ con spec.md y spec.json; contenido migrado; ficheros antiguos en raíz eliminados.
- [ ] Cada acción tiene carpeta <action-id>/ con spec.md y spec.json; contenido migrado; ficheros .md antiguos en raíz eliminados.
- [ ] interaction-triggers.md y demás referencias actualizadas a rutas por carpeta.
- [ ] Listados #Process y #Action siguen funcionando (descubren procesos y acciones por carpeta o por índice).

## 4. Dependencias

- Análisis: SddIA/process/analisis-restructuracion-patron-skills-tools.md.
- Contratos de referencia: SddIA/skills/skills-contract.json, SddIA/tools/tools-contract.json.
- Cúmulo: paths.processPath, paths.actionsPath (sin cambio de valor; cambio de convención de uso).

## 5. Riesgos y mitigación

- **Referencias rotas:** Búsqueda exhaustiva de paths.processPath/feature.md, paths.actionsPath/finalize.md, etc., y sustitución por paths.processPath/feature/, paths.actionsPath/finalize/ (o /feature/spec.md si se documenta explícitamente).
- **Orden de migración:** Hacer process primero (menos ítems) y luego actions; actualizar referencias por fases.
