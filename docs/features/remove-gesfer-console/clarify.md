# Action: Clarify (remove-gesfer-console)

## Contexto
Eliminar referencias obsoletas a `GesFer.Console` y reemplazar la ejecución de tests automatizada.

## Gaps
1. **Tests Unitarios**: El plan inicial hablaba de ejecutar tests unitarios. `pr-skill.sh` actualmente ejecutaba una opción 11 ("Suite Completa") que aparentemente ejecutaba todo.
   - **Decisión**: El nuevo binario ejecutará `dotnet test` sobre la solución completa `src/GesFer.Admin.Back.sln`. Esto incluye UnitTests e IntegrationTests.

2. **Automatización vs Manual**: Al eliminar `GesFer.Console`, las acciones como `spec` y `clarify` ahora son manuales.
   - **Decisión**: Documentar el proceso como manual en `SddIA/actions/*.md`.

3. **Herramienta Rust**: Se requiere un binario Rust para mantener la consistencia con las *skills*.
   - **Decisión**: Se implementará `run_tests` (o `full_test_suite`) en Rust que envuelva `dotnet test`.

## Plan Actualizado
1. **Tool Rust**: `scripts/skills-rs/src/bin/run_tests.rs`.
2. **Script**: `scripts/skills/pr-skill.sh` usará el binario compilado (o `cargo run`).
3. **Docs**: Actualizar `SddIA/actions/`, `SddIA/agents/`, `scripts/skills/pr-skill.md`, `docs/DeudaTecnica/DT-2025-001-MissingConsoleTool.md`.
