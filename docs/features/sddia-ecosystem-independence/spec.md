# Especificación: Independencia de ecosistema SddIA (inyección de dependencias)

**Feature:** sddia-ecosystem-independence  
**Ruta (Cúmulo):** paths.featurePath/sddia-ecosystem-independence/

## 1. Principio: ejecución por referencia

- **Problema:** En SddIA (actions, process, agents) se mencionan rutas concretas a scripts (`.ps1`, `.bat`) y comandos. Eso acopla el comportamiento IA al ecosistema de ejecución.
- **Solución:** En la **acción** solo se referencia el **contrato de la skill** (SddIA/skills/&lt;skill-id&gt;/). El agente **Tekton** es el responsable de **indicar e invocar la implementación**: resolver la cápsula (Cúmulo) y ejecutar el launcher según el contrato. La acción no contiene rutas ni comandos.
- **Analogía:** Inyección de dependencias: la acción declara "usar la skill X según su contrato" (skill-id + parámetros); **Tekton** (ejecutor) resuelve la implementación y la invoca; el contrato y la cápsula definen qué/cómo.

## 2. Contrato de inyección: acción → contrato; Tekton → implementación

| Capa | Responsabilidad | Puede | No puede |
|------|-----------------|--------|----------|
| **Acción (SddIA/actions/)** | Referenciar qué skill usar y con qué parámetros. | Referenciar la skill por su **contrato** (paths.skillsDefinitionPath/&lt;skill-id&gt;/ o SddIA/skills/&lt;skill-id&gt;/); indicar parámetros de entrada (Persist, BranchName, etc.). | Escribir rutas a scripts, paths.skillCapsules en la acción, comandos concretos ni launcher. |
| **Agente Tekton (ejecutor)** | Indicar e invocar la **implementación** de la skill. | Resolver la implementación vía Cúmulo (paths.skillCapsules, paths.skillsDefinitionPath); leer el contrato de la skill; invocar el launcher de la cápsula con los parámetros indicados en la acción. | — |
| **Skills (definición: SddIA/skills/&lt;skill-id&gt;/)** | Contrato (spec.md, spec.json): entradas, salidas, flujo; implementation_path_ref. | — | — |
| **Skills (implementación: cápsula)** | Scripts, .bat, .exe, manifest; única fuente de *cómo* se ejecuta. | — | — |

- **En la acción:** Solo referencia al **contrato** de la skill (ej. "Invocar la skill **finalizar-git** según su contrato (SddIA/skills/finalizar-git/). Parámetros: Persist, BranchName, DeleteRemote. El agente **Tekton** es responsable de resolver e invocar la implementación.").
- **Tekton:** Dado el contrato referenciado, obtiene la ruta de implementación (Cúmulo paths.skillCapsules[skill-id]), determina el launcher (manifest/cápsula) e invoca la ejecución.

## 3. Cambios en SddIA/actions

- **finalize.md:** Sustituir menciones a scripts concretos por referencia al **contrato** de la skill: "Invocar la skill **finalizar-git** según su contrato (SddIA/skills/finalizar-git/ o paths.skillsDefinitionPath/finalizar-git/), fases pre_pr y post_pr. Parámetros: Persist (ruta de docs/features/&lt;nombre_feature&gt;/), BranchName (opcional), DeleteRemote (post_pr). El agente **Tekton** es responsable de indicar e invocar la implementación (resolver cápsula y launcher según contrato y Cúmulo)."
- **validate.md:** Reemplazar "scripts de validación" por referencia al contrato de la skill de validación si existe, o "el ejecutor (Tekton) utiliza la skill correspondiente según su contrato"; no rutas literales. Tekton indica la implementación.
- **execution.md:** "Consume implementation.json; aplica cambios. El agente **Tekton** utiliza las skills indicadas en el plan según sus **contratos** (paths.skillsDefinitionPath/&lt;skill-id&gt;/) y es responsable de invocar la implementación (Cúmulo, launcher)."
- **Resto de acciones:** Cualquier "ejecutar X" → "invocar la skill Y según su contrato; Tekton indica e invoca la implementación."

## 4. Cambios en SddIA/process

- **feature.md (y otros procesos):** Fase 0: "Invocar la skill **iniciar-rama** según su contrato (SddIA/skills/iniciar-rama/). Parámetros: BranchType, BranchName. El agente **Tekton** indica e invoca la implementación." No incluir rutas a scripts ni paths.skillCapsules en el proceso. Fase 8: "Invocar la skill **finalizar-git** según su contrato y según SddIA/actions/finalize.md; Tekton indica e invoca la implementación."
- Cualquier otro proceso que ejecute algo: referencia al **contrato** de la skill; Tekton responsable de la implementación.

## 5. Cambios en agentes y normas

- **Tekton (SddIA/agents/tekton-developer.json):** Definir explícitamente que Tekton es el agente responsable de **indicar e invocar la implementación** de las skills: dado el contrato de la skill referenciado en la acción (o proceso), Tekton resuelve la implementación vía Cúmulo (paths.skillCapsules, paths.skillsDefinitionPath) e invoca el launcher de la cápsula. Las acciones no definen rutas ni comandos; Tekton lo hace según contrato.
- **Otros agentes:** Si deben "ejecutar" algo, sus instrucciones deben decir "usar la skill X según su contrato"; la ejecución concreta la indica e invoca Tekton (o el agente ejecutor asignado, siendo Tekton el estándar).
- **Norms (SddIA/norms/):** Añadir que en la acción solo se referencia el **contrato** de la skill; el **agente Tekton** es responsable de indicar e invocar la implementación (resolver cápsula y launcher vía Cúmulo).

## 6. Documentación de la norma

- Añadir en **SddIA/norms/** una norma breve: "En la **acción** solo se referencia el **contrato** de la skill (SddIA/skills/&lt;skill-id&gt;/). El agente **Tekton** es responsable de indicar e invocar la **implementación** (resolver cápsula vía Cúmulo, ejecutar launcher). La acción no contiene rutas ni comandos."
- Opcional: entrada en constitution o skills-contract.md: la acción referencia el contrato; Tekton la implementación.

## 7. Restricciones

- No introducir rutas literales a scripts en SddIA/actions ni SddIA/process.
- Cúmulo sigue siendo la única fuente de rutas (paths.skillCapsules, paths.skillsDefinitionPath).
- Las skills existentes (iniciar-rama, finalizar-git, invoke-command, etc.) no cambian de implementación; solo cambia la forma en que SddIA las referencia.
