# Feature: Independencia de ecosistema SddIA (comportamiento IA)

**Rama:** feat/sddia-ecosystem-independence  
**Ruta (Cúmulo):** paths.featurePath/sddia-ecosystem-independence

## Objetivo

Desacoplar la **ejecución de comandos** de las **acciones** (#acciones) y del resto de SddIA. Las **skills** son las únicas responsables de realizar la acción concreta (scripts, binarios, comandos). En la **acción** solo se hace referencia al **contrato de la skill** (definición en SddIA/skills/&lt;skill-id&gt;/). El agente **Tekton** es el responsable de indicar e invocar la **implementación** de la skill: resolver la cápsula (Cúmulo) y ejecutar el launcher según el contrato, sin que la acción contenga rutas ni comandos literales.

## Alcance

- **Proceso:** SddIA/process/feature (fase 0: rama feat/sddia-ecosystem-independence; fases 1–8 según feature.md).
- **Norma de diseño:** En la **acción** solo se referencia el **contrato de la skill** (SddIA/skills/&lt;skill-id&gt;/ spec.md, spec.json). El **agente Tekton** es el responsable de indicar e invocar la implementación (resolver cápsula vía Cúmulo, ejecutar launcher); la acción no contiene rutas ni comandos.
- **Actions (SddIA/actions/):** Cada acción que requiera ejecución debe referenciar la **skill por su contrato** (ej. "Invocar la skill **finalizar-git** según su contrato (SddIA/skills/finalizar-git/ o paths.skillsDefinitionPath/finalizar-git/); parámetros: Persist, BranchName, etc. El agente **Tekton** es responsable de resolver e invocar la implementación."). No rutas a scripts ni Cúmulo de implementación en la acción.
- **Process (SddIA/process/):** Fase 0 y fases que ejecutan algo: referencia al **contrato** de la skill (iniciar-rama, finalizar-git); Tekton indica e invoca la implementación.
- **Tekton (ejecutor):** Es el agente que, dado el contrato de la skill referenciado en la acción, resuelve la implementación (paths.skillCapsules, launcher) e invoca la ejecución. La acción no define *cómo* ejecutar; Tekton lo hace según contrato y Cúmulo.
- **Documentación:** Especificar que la acción referencia solo el contrato; Tekton es responsable de la implementación. Opcional: norma en SddIA/norms/ o constitution.

## Ley aplicada

- **L6_CONSULTATION:** Rutas y capacidades de ejecución solo vía Cúmulo (paths.skillCapsules, paths.skillsDefinitionPath) y contrato de skills. No rutas literales en actions/process.
- **Desacoplamiento:** La acción referencia solo el contrato de la skill (qué hacer, qué skill, parámetros); el agente Tekton indica e invoca la implementación (cómo ejecutar); las skills definen contrato e implementación.

## Resumen del proceso

| Fase | Acción |
|------|--------|
| 0 | Rama feat/sddia-ecosystem-independence (skill iniciar-rama) |
| 1 | Documentación objetivos (este documento) |
| 2 | Spec (spec.md, spec.json en carpeta de la tarea / Cúmulo) |
| 3–6 | Clarificación, plan, implementación, ejecución |
| 7 | Validación (validacion.json) |
| 8 | Cierre y PR (skill finalizar-git) |

## Referencias

- Cúmulo: SddIA/agents/cumulo.json (paths.skillCapsules, skillsDefinitionPath).
- Contrato skills: SddIA/skills/skills-contract.md, skills-contract.json.
- Acciones: SddIA/actions/ (finalize.md, validate.md, execution.md, etc.).
- Proceso feature: SddIA/process/feature.md.
