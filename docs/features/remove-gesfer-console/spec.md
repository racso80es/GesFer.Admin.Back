# Action: Spec (remove-gesfer-console)

## Requerimientos
1. **Eliminación de Referencias**:
   - `SddIA/actions/*.md` deben actualizarse para no referenciar `GesFer.Console`.
   - `SddIA/agents/*.json` deben actualizarse para no referenciar `GesFer.Console`.
   - `docs/DeudaTecnica/DT-2025-001-MissingConsoleTool.md` debe cerrarse o actualizarse indicando que la herramienta fue descatalogada.
2. **Reemplazo de Funcionalidad**:
   - Se debe crear un binario Rust `run_tests` en `scripts/skills-rs/src/bin/run_tests.rs`.
   - `scripts/skills-rs/Cargo.toml` debe incluir el nuevo binario.
   - `scripts/skills/pr-skill.sh` debe invocar `cargo run --bin run_tests` (o el ejecutable compilado) en lugar de `dotnet run ... GesFer.Console`.
3. **Criterios de Aceptación**:
   - No deben existir referencias a `GesFer.Console` en el repositorio.
   - `pr-skill.sh` debe ejecutar los tests correctamente.
   - La documentación debe reflejar procesos manuales donde antes se automatizaba con `GesFer.Console`.

## Implementación Técnica
- **Rust Tool (`run_tests`)**:
   - Ejecutará `dotnet test`.
   - Retornará código de salida 0 si éxito, != 0 si fallo.
   - Cumplirá (o al menos emulará) el contrato básico de salida si es necesario, aunque `pr-skill.sh` actualmente solo verifica el código de salida.

- **Scripts**:
   - Modificar `pr-skill.sh` para llamar al nuevo binario.

- **Documentación**:
   - Search & Replace inteligente en los archivos afectados.
