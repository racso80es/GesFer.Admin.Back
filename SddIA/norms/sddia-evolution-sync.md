---
norm_id: sddia-evolution-sync
contract_ref: SddIA/evolution/evolution_contract.md
paths_ref: SddIA/agents/cumulo.paths.json
related:
  - SddIA/norms/paths-via-cumulo.md
  - SddIA/norms/entidades-dominio-ecosistema-sddia.md
schema_version: "1.0"
---

# Norma: Sincronismo y trazabilidad SddIA (evolution)

## Objetivo

Garantizar que **toda intervención** (alta, baja o modificación) al protocolo de actuación de la IA bajo `./SddIA/` quede **registrada de forma inmediata y auditable**, con un **Contrato de Cambio YAML v1.0** replicable en otros entornos SSDD IA.

## Ámbito

- Incluye: normas, procesos, acciones, agentes, skills (definición), patrones, principios, tokens y demás artefactos normativos bajo `./SddIA/`.
- Excluye: código de aplicación y documentación de producto en `docs/` salvo cuando el cambio sea consecuencia directa de una norma SddIA (entonces puede existir entrada duplicada en `paths.evolutionPath` por cierre de feature).

## Rutas (Cúmulo)

Consultar **únicamente** claves del contrato de paths:

- `paths.sddiaEvolutionPath` — carpeta de evolución del protocolo.
- `paths.sddiaEvolutionLogFile` — nombre del índice maestro (`Evolution_log.md`).
- `paths.sddiaEvolutionContractFile` — especificación del contrato (`evolution_contract.md`).

No usar rutas literales en documentación de comportamiento; referir `paths.*` (norma `paths-via-cumulo.md`).

## Contrato obligatorio

Cada intervención debe producir:

1. **`id_cambio`:** `SSDD-LOG-YYYYMMDD-HHMM`.
2. **Índice:** una línea o fila en `Evolution_log.md` con ID, fecha y `descripcion_breve`.
3. **Detalle atómico:** `{id_cambio}.md` con frontmatter YAML según `evolution_contract.md` (v1.0), incluyendo **tipología** (alta / baja / modificación) según versión vigente del contrato, y cuerpo Markdown con resumen.

## Trigger (proceso)

| Modo | Descripción |
| :--- | :--- |
| **Binario Rust** | Registro y hash mediante ejecutable en cápsula (paths.skillCapsules o paths.toolCapsules), **stdin/stdout JSON** según `SddIA/norms/capsule-json-io.md`. **Prohibido** usar `.ps1` como implementación del registro. |
| **Watcher** | Proceso automático en alcance v1 que observe `./SddIA/` y dispare registro o validación según `docs/features/sddia-evolution-sync-norma/plan.md`. |
| **Agente IDE (Jules, Cursor, etc.)** | Antes de dar por cerrada una tarea que haya alterado `./SddIA/`, el agente **debe** completar el triple registro (ID, índice, detalle) en la misma sesión o en el mismo PR. |
| **CI / `.github`** | Workflows que validen PRs con cambios en `./SddIA/` frente a `Evolution_log.md` y el contrato; ver `SddIA/norms/touchpoints-ia.md` y acción **sddia-difusion**. |

## Gobernanza

- Esta norma es **inegociable** para flujos que alteren el núcleo SddIA: la ausencia de registro equivale a incumplimiento del proceso de evolución.
- Los IDEs y agentes deben leer `SddIA/CONSTITUTION.md` y esta norma como restricción de comportamiento.

## Difusión

Mantener alineada la regla de Cursor `.cursor/rules/sddia-evolution-sync.mdc` con esta norma (acción **sddia-difusion** cuando proceda).
