# Clarificaciones: Kaizen process/actions restructuración

**Feature:** kaizen-process-actions-restructure  
**Ruta (Cúmulo):** paths.featurePath/kaizen-process-actions-restructure  
**Entrada:** spec.md, spec.json

---

## 1. Decisiones tomadas

| Punto | Decisión | Motivo |
|-------|----------|--------|
| **Convención de entrada** | La “definición” de un proceso o acción es la carpeta `<id>/`; el punto de entrada canónico para lectura humana es `spec.md` y para máquina `spec.json`. | Alineado con skills/tools; las referencias pueden ser paths.processPath/feature/ o paths.processPath/feature/spec.md según contexto. |
| **Cúmulo paths** | No se cambian los valores de paths.processPath ni paths.actionsPath (siguen siendo ./SddIA/process/ y ./SddIA/actions/). Solo cambia la convención: dentro de cada path, cada ítem vive en una carpeta. | Evitar cambios en cumulo.paths.json y en todos los consumidores que solo leen la raíz; los que referencian ficheros concretos (feature.md) se actualizan a feature/spec.md o feature/. |
| **process_interface en Cúmulo** | Se mantiene en cumulo.json (process_interface: artefactos .md y .json en carpeta de la tarea). El process-contract en SddIA/process/ define la forma de la **definición** del proceso (spec en cada carpeta); no sustituye la interfaz de “qué genera un proceso en la tarea”. | Separación: contrato de definición (SddIA) vs interfaz de ejecución/tarea (Cúmulo). |
| **Orden de migración** | Primero process (4 ítems), luego actions (8 ítems). Dentro de process: orden sugerido feature → refactorization → create-tool → bug-fix (por dependencias y tamaño). | Reducir riesgo de referencias rotas; process tiene menos consumidores directos que actions. |
| **Listado #Process / #Action** | interaction-triggers y .cursor/rules pueden seguir listando por nombre (feature, bug-fix, …); la resolución a ruta es paths.processPath/feature/, paths.actionsPath/spec/, etc. Si se quiere descubrir por filesystem, listar directorios en paths.processPath y paths.actionsPath que contengan spec.json. | Compatibilidad con comportamiento actual de “listar ítems”; implementación puede ser por índice (README o JSON) o por escaneo de carpetas. |

## 2. Preguntas resueltas

- **¿Eliminar analisis-restructuracion-patron-skills-tools.md tras la migración?** No; queda como documento de análisis y referencia. Puede moverse a docs/ o mantenerse en SddIA/process/ como histórico.
- **¿README en actions?** Sí; crear SddIA/actions/README.md con tabla de acciones y rutas (paths.actionsPath/<action-id>/), análogo a process/README.md.
- **¿spec.json por acción con schema fijo?** Sí; campos mínimos: action_id, name, purpose, inputs, outputs, flow_steps (resumen), contract_ref, related_processes (opcional). Permite evolución posterior sin romper contrato.

## 3. Abiertos (opcional para siguientes fases)

- **Índice machine-readable:** ¿Añadir process-index.json y actions-index.json en la raíz de process/ y actions/ con array de { id, path, spec_ref } para consumidores que no quieran escanear carpetas?
- **Compatibilidad temporal:** ¿Mantener redirecciones o enlaces “feature.md → feature/spec.md” en un README durante una versión para herramientas que aún referencien el path antiguo?

---
*Clarificaciones para SPEC kaizen-process-actions-restructure. Fuente: spec.md, spec.json.*
