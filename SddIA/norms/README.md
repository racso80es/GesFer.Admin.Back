# Normas SddIA

Este directorio contiene **normas de comportamiento** del agente definidas en el dominio SddIA. El agente debe consultarlas para responder ante ciertos disparadores del usuario.

| Documento | Descripción |
|------------|-------------|
| **interaction-triggers.md** | Disparadores de interacción (#Skill, #Action, #Process): cuándo aplican y qué comportamiento seguir. |
| **interaction-triggers.json** | Versión machine-readable de los disparadores. |
| **paths-via-cumulo.md** | Rutas solo desde Cúmulo (contrato de paths); no rutas literales. |
| **git-via-skills-or-process.md** | La IA nunca ejecuta git directamente; solo vía skill, herramienta, acción o proceso. |
| **agents-principles-contract.md** | Implementación del contrato de principios en agentes (principlesContract). |
| **patterns-in-planning-implementation-execution.md** | Aplicación de patrones en planning, implementation, execution. |

**Referencia en protocolo:** AGENTS.md (leyes universales, disparadores) indica que el comportamiento lo define SddIA y remite a este directorio.
