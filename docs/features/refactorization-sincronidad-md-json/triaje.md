# Triaje: Sincronidad MD/JSON (Protocolo Racso-Tormentosa v1.2)

**Proceso:** refactorization  
**Fase:** Triaje — identificación de skills, dependencias y discrepancias.

## Skills inspeccionadas (paths.skillsDefinitionPath)

| skill_id | spec.md | spec.json | Dependencias (procesos/acciones) | Discrepancia MD ↔ JSON |
|----------|---------|-----------|----------------------------------|--------------------------|
| invoke-command | Sí | Sí | finalize, spec, clarify, planning, execution; procesos feature, refactorization | **Resuelta:** JSON actualizado con parameters, command_file_routing (acción previa). |
| iniciar-rama | Sí | Sí | Procesos feature, bug-fix, refactorization (fase 0) | Estructura alineada (parameters, phases en JSON). |
| finalizar-git | Sí | Sí | Acción finalize; procesos feature, refactorization (fase 8) | Estructura alineada (phases pre_pr/post_pr). |
| documentation | Sí | Sí | Acciones spec, clarify, planning; agentes Cúmulo | Reglas en MD y JSON coinciden. |
| dotnet-development | Sí | Sí | Acción execution, validate | Reglas y commands en JSON; MD conciso. |
| git-operations | Sí | Sí | Norma git-via-skills-or-process; finalizar-git | Reglas y common_workflows alineados. |
| verify-pr-protocol | Sí | Sí | finalizar-git, validate | Definición técnica en ambos. |
| security-audit | Sí | Sí | validate, correccion-auditorias | Par. |
| frontend-build | Sí | Sí | execution, validate | Par. |
| frontend-test | Sí | Sí | validate | Par. |
| filesystem-ops | Sí | Sí | Varias acciones | Par. |

## Skill detectada con JSON desfasado (histórico)

- **invoke-command:** El JSON no incluía `parameters` ni `command_file_routing` hasta la refactorización reciente (rust-skills-tools-protocol). El MD ya describía --command-file y rutas; el JSON se ha sincronizado en esa tarea.

## Dependencias de proceso

- **refactorization** utiliza acciones: spec, clarify, planning, implementation, execution, validate, finalize (paths.actionsPath).
- Skills usadas por el proceso: iniciar-rama (fase 0), finalizar-git (fase 8), invoke-command (implícito en ejecución de comandos).

## Discrepancias listadas (para Análisis)

1. **Causa raíz a documentar:** Las acciones de edición (p. ej. al añadir --command-file en MD) no tenían un paso obligatorio de actualización del JSON.
2. **Riesgo:** Otras skills pueden tener campos en MD no reflejados en JSON (parámetros, rutas, workflows) si no se aplica la regla de sincronía.
