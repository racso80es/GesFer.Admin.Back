# Objetivo: Eliminar GesFer.Console

## Contexto
Se han identificado múltiples referencias a `GesFer.Console` en la documentación y scripts, herramienta que no existe y está descatalogada.

## Metas
1. Eliminar todas las referencias a `GesFer.Console`.
2. Reemplazar la ejecución de tests en `pr-skill.sh` por una herramienta Rust (`run_tests`) que cumpla con el contrato de skills.
3. Actualizar la documentación de acciones (`spec`, `clarify`, `planning`) para reflejar procesos manuales o el uso de herramientas alternativas si las hubiera (en este caso, manual por el momento).

## Alcance
- `scripts/skills-rs/`
- `scripts/skills/pr-skill.sh`
- `SddIA/actions/`
- `SddIA/agents/`
- `docs/DeudaTecnica/`
