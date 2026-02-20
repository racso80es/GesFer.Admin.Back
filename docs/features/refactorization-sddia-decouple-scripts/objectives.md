# Refactorización: Desacoplar dominio SddIA del uso de scripts

**Rama:** feat/refactorization-sddia-decouple-scripts  
**Ruta (Cúmulo):** paths.featurePath/refactorization-sddia-decouple-scripts  
**Proceso:** paths.processPath/refactorization.md

---

## Objetivo

Desacoplar el **dominio SddIA** del uso directo del concepto «script». Desde SddIA **no existe el script**: solo existen **skills** (paths.skillsDefinitionPath, paths.skillCapsules) y **herramientas** (paths.toolsDefinitionPath, paths.toolCapsules). Toda referencia en normas, acciones, procesos y documentación de SddIA debe hablar de **skills** o **herramientas** (y sus contratos/cápsulas), no de scripts. La capa de implementación (PowerShell, .bat, binarios) vive en las cápsulas; SddIA solo referencia contratos y rutas vía Cúmulo.

## Alcance

- **Normas (SddIA/norms/):** Eliminar o reemplazar referencias a «script» por skill o herramienta según corresponda. Ejemplo: interaction-triggers.md y paths-via-cumulo.md no deben nombrar scripts; sí paths.skillCapsules, paths.toolCapsules, definiciones.
- **Acciones (SddIA/actions/):** Referenciar solo skills/herramientas por contrato (paths.skillsDefinitionPath, paths.toolsDefinitionPath) e invocación vía cápsula (paths.skillCapsules, paths.toolCapsules); Tekton ejecuta la implementación. No nombres de archivos .ps1/.bat en la capa SddIA.
- **Procesos (SddIA/process/):** Igual: solo skills/herramientas y contratos; no scripts.
- **Skills y tools (SddIA/skills/, SddIA/tools/):** En la **definición** (spec.md, spec.json) se puede describir la implementación (ej. «launcher .bat o .ps1 en la cápsula») como detalle del contrato, pero el identificador es el skill-id o tool-id; la ruta se resuelve por Cúmulo (implementation_path_ref).
- **Resultado:** Un agente o documentación que lea solo SddIA no ve «script»; ve skills y herramientas referenciadas por contrato y por paths de Cúmulo.

## Ley aplicada

- **L6_CONSULTATION (AGENTS.md):** Rutas y capacidades solo vía Cúmulo; no rutas literales.
- **Desacoplamiento dominio / implementación:** SddIA = dominio (normas, acciones, procesos, definiciones de skills y tools). Implementación = cápsulas (scripts, binarios) referenciadas por paths.skillCapsules / paths.toolCapsules. En SddIA no se nombran archivos de implementación (.ps1, .bat) salvo en la definición del contrato de la skill/tool cuando se describe el launcher.

---

## Análisis de situación actual

Inventario de referencias a «script» o archivos de implementación (.ps1, .bat) en SddIA que deben pasarse a lenguaje skill/herramienta o a referencia vía Cúmulo.

### 1. SddIA/norms/

| Archivo | Situación |
|--------|-----------|
| **interaction-triggers.md** | No menciona «script» explícitamente; ya referencia paths.skillsDefinitionPath y paths.skillCapsules. Revisar que ningún texto diga «script»; si se añade #Tool, referenciar paths.toolsDefinitionPath / paths.toolCapsules. |
| **paths-via-cumulo.md** | Ejemplo con «scripts/skills/» como literal: sustituir por «paths.skillsPath o paths.skillCapsules». Ya indica no usar literales docs/... ni scripts/...; reforzar que desde SddIA solo paths.*. |
| **interaction-triggers.json** | Machine-readable; revisar que list_source y behavior no contengan «script»; usar «skill»/«tool» y paths. |

### 2. SddIA/actions/

| Archivo | Situación |
|--------|-----------|
| **finalize.md** | Menciona **Push-And-CreatePR.ps1** por nombre. Objetivo: «Invocar la skill **finalizar-git** (fase pre_pr) según contrato; Tekton invoca la implementación (paths.skillCapsules[\"finalizar-git\"]). Parámetros: -Persist, etc.» Sin nombrar .ps1 en la acción. |
| **validate.md** | «Scripts existentes (ej. validate-pr.ps1)» y «acceso a scripts de validación». Sustituir por: skill o herramienta de validación según contrato; invoke-command si aplica. |
| **implementation.md** / **execution.md** | «Mediante scripts o el agente»: sustituir por «mediante la skill/herramienta correspondiente o el agente Tekton que consuma el documento». |

### 3. SddIA/process/

| Archivo | Situación |
|--------|-----------|
| **feature.md** | «Scripts y agentes» para spec, clarify, plan: reemplazar por «skills/herramientas y agentes según paths.actionsPath». |
| **refactorization.md** | Recién creado; ya redactado en términos de skills (paths.skillCapsules[\"iniciar-rama\"]) sin nombrar scripts. |

### 4. SddIA/skills/ (definiciones)

| Archivo | Situación |
|--------|-----------|
| **skills-contract.md** / **README.md** | Hablan de «implementación (scripts, config, ejecutables)» y «script .ps1». Aceptable en **definición** del contrato (describen qué hay en la cápsula). Opcional: sustituir «scripts» por «implementación en cápsula» (launcher .ps1/.bat o binario). |
| **finalizar-git/spec.md** | Nombra Push-And-CreatePR.ps1, Merge-To-Master-Cleanup.bat: es **especificación del contrato** de la skill; puede quedar como detalle del componente. Alternativa: «componente pre_pr» / «componente post_pr» sin extensión. |
| **invoke-command/spec.md**, **iniciar-rama/spec.md** | Ejemplos con rutas literales .\scripts\skills\...: sustituir por «invocar vía paths.skillCapsules[\"invoke-command\"]» (o similar) y launcher según contrato. |
| **security-audit/spec.md** | «Hooks referencian scripts en la cápsula»: cambiar a «hooks invocan la skill/herramienta en la cápsula cuando exista implementación». |

### 5. SddIA/tools/ (definiciones)

| Archivo | Situación |
|--------|-----------|
| **tools-contract.md** / **README.md** | «Implementación (scripts, config, ejecutables)»: en definición del contrato es aceptable; opcional «implementación técnica en cápsula». |
| **prepare-full-env/spec.md**, **invoke-mysql-seeds/spec.md** | «Implementación (scripts):»: sustituir por «Implementación: cápsula en paths.toolCapsules.<tool-id> (Cúmulo)»; no nombrar «scripts» como capa. |

### 6. Criterio de cambio

- **En normas y acciones:** Cero menciones a «script»; solo «skill», «herramienta», «cápsula», «contrato», paths.skillCapsules, paths.toolCapsules, paths.skillsDefinitionPath, paths.toolsDefinitionPath.
- **En definiciones de skill/tool (spec.md, spec.json):** Se puede describir que la cápsula contiene «launcher .bat o .ps1» como detalle del contrato; el identificador sigue siendo skill-id/tool-id y la ruta vía Cúmulo. Evitar ejemplos con rutas literales a scripts fuera de Cúmulo.

---

## Resumen del proceso (refactorization)

| Fase | Acción |
|------|--------|
| 0 | Rama feat/refactorization-sddia-decouple-scripts (skill iniciar-rama) |
| 1 | Documentación objetivos (este documento) y análisis de situación |
| 2 | Spec (spec.md, spec.json en carpeta de la tarea / Cúmulo) |
| 3–6 | Clarificación, plan, implementación (doc), ejecución |
| 7 | Validación (validacion.json) |
| 8 | Cierre y PR (skill finalizar-git) |

## Referencias

- Cúmulo: SddIA/agents/cumulo.json → pathsContract (cumulo.paths.json), instructionsContract (cumulo.instructions.json).
- Contrato skills: SddIA/skills/skills-contract.md, skills-contract.json.
- Contrato tools: SddIA/tools/tools-contract.md, tools-contract.json.
- Acciones: SddIA/actions/ (finalize.md, validate.md, etc.).
- Proceso: SddIA/process/refactorization.md.
