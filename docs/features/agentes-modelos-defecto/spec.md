# Modelos por Defecto para Agentes

## Objetivo

AGENTS.md ha de indicar esta característica para subagentes; para arquitecto ha de ser modelo Claude Opus 4.6; para Tekton el modelo ha de ser Claude Sonnet 4.5; para el resto de agentes, sugerir opciones. En caso de no disponer del modelo configurado, optar por la mejor opción posible para la tarea encomendada al agente.

## Cambios a realizar

1.  **AGENTS.md**: Añadir sección `## 8. MODELOS DE IA RECOMENDADOS` con la lista de modelos permitidos.
2.  **Archivos JSON**: Actualizar las definiciones de agentes en `SddIA/agents/` para que incluyan un campo `default_model`.
