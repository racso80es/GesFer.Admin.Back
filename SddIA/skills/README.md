# Skills SddIA

Los **skills** son capacidades reutilizables que pueden ser consumidas por las **acciones** (`SddIA/actions/`) y por agentes. Este directorio contiene la definición y documentación de cada skill.

## Contrato de skills

Todo skill en `SddIA/skills/` debe cumplir el siguiente contrato (interfaz obligatoria):

| Artefacto | Propósito |
| :--- | :--- |
| **`<skill-id>.md`** | Documentación legible: objetivo, entradas/salidas, flujo de uso, reglas y notas. Lengua: español (es-ES). |
| **`<skill-id>.json`** | Metadatos machine-readable: identificador, nombre, descripción, reglas, workflows y datos que consuman acciones o agentes. |

- **`<skill-id>`** debe ser un identificador en kebab-case (ej. `finalizar-git`, `git-operations`).
- Un skill **no se considera completo** si falta alguno de los dos artefactos.
- Los agentes y acciones que invoquen un skill deben resolver primero el `.md` para el flujo y el `.json` para parámetros y reglas automatizables.

## Relación con acciones

Las acciones en `SddIA/actions/` pueden **referenciar** uno o más skills por `skill_id`. Al ejecutar una acción, el agente responsable debe aplicar los pasos y reglas del skill referenciado (leyendo `SddIA/skills/<skill_id>.md` y `SddIA/skills/<skill_id>.json`).

## Listado de skills

| skill_id | Descripción breve |
| :--- | :--- |
| `iniciar-rama` | Crea una rama nueva (feat/ o fix/) actualizada con master/main y posiciona el repo en ella; para el inicio de una acción. |
| `finalizar-git` | Centraliza las interacciones con git para aceptar PR a master, unificar, eliminar rama unificada y volver a master. |
| `git-operations` | Uso seguro y semántico de Git (ramas feat/fix, commits convencionales, pre-push). |
| `documentation` | Estándares SSOT y gestión de documentación (Markdown, jerarquía, Evolution Log). |
| *(otros en este directorio)* | Ver ficheros `.json` para `name` y `description`. |

## Estándares de calidad

- **Grado S+:** Cada skill es una única fuente de verdad para su capacidad; las acciones no duplican la lógica del skill.
- **Ley GIT:** Los skills que ejecuten operaciones git respetan AGENTS.md (no commit en master, ramas feat/fix).
- **Idioma:** Documentación en español (es-ES).

---
*Contrato de skills SddIA. Las acciones consumidoras deben respetar este contrato al invocar skills.*
