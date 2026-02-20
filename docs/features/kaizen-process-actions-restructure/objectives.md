# Objetivos: Kaizen — Restructuración process y actions al patrón skills/tools

**Rama:** feat/kaizen-process-actions-restructure  
**Ruta (Cúmulo):** paths.featurePath/kaizen-process-actions-restructure  
**Proceso:** paths.processPath/refactorization.md (o feature.md)

## Objetivo

Reestructurar **SddIA/process** y **SddIA/actions** para que cumplan el mismo patrón que **SddIA/skills** y **SddIA/tools**: cada ítem en su carpeta (`<process-id>/`, `<action-id>/`) con `spec.md` y `spec.json`, y contratos explícitos en raíz (`process-contract.json`, `actions-contract.json`).

## Alcance

- **Process:** Contrato en SddIA/process/; carpetas feature/, bug-fix/, refactorization/, create-tool/ con spec.md + spec.json cada una; migrar contenido actual.
- **Actions:** Contrato en SddIA/actions/; carpetas por acción (spec/, clarify/, planning/, implementation/, execution/, validate/, finalize/, sddia-difusion/) con spec.md + spec.json cada una; migrar contenido actual.
- **Referencias:** Actualizar interaction-triggers, README, Cúmulo y consumidores a rutas por carpeta.

## Ley aplicada

- **L6_CONSULTATION:** Rutas solo vía Cúmulo; paths.processPath y paths.actionsPath siguen siendo la fuente; convención de entrada por carpeta (spec).
- **SSOT:** Análisis en SddIA/process/analisis-restructuracion-patron-skills-tools.md.
