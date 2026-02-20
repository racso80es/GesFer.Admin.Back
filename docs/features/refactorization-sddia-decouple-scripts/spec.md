# SPEC: Desacoplar dominio SddIA del uso de scripts

## 1. Información General

| Campo | Detalle |
| :--- | :--- |
| **ID de Especificación** | SPEC-REF-2026-001 |
| **Rama Relacionada** | feat/refactorization-sddia-decouple-scripts |
| **Estado** | Draft |
| **Responsable** | Spec Architect / Tekton |
| **Ruta (Cúmulo)** | paths.featurePath/refactorization-sddia-decouple-scripts |

## 2. Propósito y Contexto

### 2.1. Objetivo (Goal)

Eliminar del **dominio SddIA** toda referencia al concepto «script». En SddIA solo existen **skills** (paths.skillsDefinitionPath, paths.skillCapsules) y **herramientas** (paths.toolsDefinitionPath, paths.toolCapsules). La capa de implementación (PowerShell, .bat, binarios) pertenece a las cápsulas; SddIA referencia únicamente contratos e identificadores, resolviendo rutas vía Cúmulo. Cualquier documento o agente que lea solo SddIA no debe encontrar la palabra «script» en normas, acciones o procesos; en definiciones de skill/tool solo se admite como detalle del contrato (ej. «launcher en cápsula») sin rutas literales.

### 2.2. Alcance (Scope)

**Incluido:**

- SddIA/norms/: sustituir o eliminar referencias a «script»; usar skill/herramienta y paths (paths.skillCapsules, paths.toolCapsules, paths.skillsDefinitionPath, paths.toolsDefinitionPath).
- SddIA/actions/: referenciar solo skills/herramientas por contrato; no nombres de archivos .ps1/.bat; Tekton invoca implementación vía paths.skillCapsules / paths.toolCapsules.
- SddIA/process/: mismo criterio; en feature.md reemplazar «Scripts y agentes» por «skills/herramientas y agentes».
- SddIA/skills/ (definiciones): opcional sustituir «scripts» por «implementación en cápsula»; en specs de skills eliminar rutas literales (.\scripts\skills\...) y usar paths.skillCapsules; en componentes (finalizar-git, etc.) se puede mantener nombre de componente sin extensión (.ps1/.bat) o dejarlo como detalle del contrato.
- SddIA/tools/ (definiciones): «Implementación (scripts)» → «Implementación: cápsula en paths.toolCapsules.<tool-id> (Cúmulo)»; no nombrar «scripts» como capa.

**Fuera de alcance:**

- Cambiar la implementación real de las cápsulas (contenido de scripts/, binarios). Solo cambia la redacción y referencias en la capa SddIA.
- Añadir nuevas skills o herramientas; solo refactor de documentación y normas.

## 3. Arquitectura y Diseño Técnico

### 3.1. Componentes Afectados (SddIA)

| Zona | Archivos / Ámbito | Cambio |
| :--- | :--- | :--- |
| **norms** | interaction-triggers.md, interaction-triggers.json, paths-via-cumulo.md | Cero «script»; solo skill/tool y paths. Ejemplos sin literales scripts/.... |
| **actions** | finalize.md, validate.md, implementation.md, execution.md | Referencia a skill/herramienta por contrato; invocación por Tekton vía cápsula. Sin .ps1/.bat en texto de la acción. |
| **process** | feature.md | «Scripts y agentes» → «skills/herramientas y agentes según paths.actionsPath». |
| **skills** | skills-contract.md, README.md, finalizar-git/spec.md, invoke-command/spec.md, iniciar-rama/spec.md, security-audit/spec.md | Sin rutas literales; opcional «implementación en cápsula» en lugar de «scripts»; componentes por nombre sin extensión o como detalle. |
| **tools** | tools-contract.md, README.md, prepare-full-env/spec.md, invoke-mysql-seeds/spec.md | «Implementación (scripts)» → cápsula vía Cúmulo; no capa «scripts». |

### 3.2. Regla de Lenguaje (SddIA)

- **Normas y acciones:** Vocabulario permitido: skill, herramienta, cápsula, contrato, paths.skillCapsules, paths.toolCapsules, paths.skillsDefinitionPath, paths.toolsDefinitionPath, implementation_path_ref. Prohibido: script, .ps1, .bat como identificador de ejecución en la acción/norma (salvo en definición de contrato como detalle del launcher).
- **Definiciones (spec de skill/tool):** Se puede describir que la cápsula incluye «launcher .bat o .ps1»; no ejemplos con rutas literales fuera de Cúmulo (ej. .\scripts\skills\...).

## 4. Requisitos de Seguridad

- **Documentación únicamente:** Esta refactorización no introduce código ejecutable nuevo; solo cambios en documentación y redacción. No hay validación de input de usuario ni PII.
- **Auditoría:** Los cambios se rastrean en la carpeta de la tarea (Cúmulo) y en paths.evolutionPath + paths.evolutionLogFile al cierre (fase 8).

## 5. Criterios de Aceptación

Para dar por cerrada esta especificación (y su ejecución):

- [ ] En SddIA/norms/ no aparece «script» salvo en contexto permitido (definición de contrato describiendo launcher).
- [ ] En SddIA/actions/ no aparecen nombres de archivos .ps1/.bat; solo referencia a skills/herramientas y paths de Cúmulo.
- [ ] En SddIA/process/feature.md no figura «Scripts y agentes»; sí «skills/herramientas y agentes».
- [ ] En definiciones de skills/tools no hay ejemplos con rutas literales .\scripts\...; sí paths.skillCapsules / paths.toolCapsules o implementation_path_ref.
- [ ] Documentación de la refactorización en la carpeta de la tarea (Cúmulo) (objectives.md, spec.md, spec.json, implementation, validacion.json) completa y coherente con objectives.
- [ ] Evolution Log actualizado al cierre (paths.evolutionPath + paths.evolutionLogFile).

## 6. Trazabilidad

- **Origen:** docs/features/refactorization-sddia-decouple-scripts/objectives.md
- **Proceso:** paths.processPath/refactorization.md
- **Referencia de auditoría:** paths.auditsPath + paths.accessLogFile (Cúmulo)
