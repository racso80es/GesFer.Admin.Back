---
feature_name: create-tool-run-test-e2e-local
branch: feat/create-tool-run-test-e2e-local
base_branch: main
global: pass
blocking: false
checks:
  - name: cargo_release
    result: pass
    message: cargo build --release (scripts/tools-rs) compila run_test_e2e_local sin errores
  - name: smoke_envelope
    result: pass
    message: Ejecución con GESFER_CAPSULE_REQUEST (solo smoke) devuelve success true y exitCode 0
  - name: cli_flags
    result: pass
    message: CLI acepta --run-smoke false y omite fases correctamente
  - name: contract_paths
    result: pass
    message: Cúmulo incluye toolCapsules.run-test-e2e-local; index.json lista la herramienta
  - name: evolution_sddia
    result: pass
    message: Entrada en SddIA/evolution/Evolution_log.md y detalle b6a79723-db30-461c-94aa-e98e8c1e90ec.md
git_changes:
  files_added:
    - scripts/tools-rs/src/bin/run_test_e2e_local.rs
    - scripts/tools/run-test-e2e-local/manifest.json
    - scripts/tools/run-test-e2e-local/run-test-e2e-local-config.json
    - scripts/tools/run-test-e2e-local/run-test-e2e-local.md
    - scripts/tools/run-test-e2e-local/Run-Test-E2E-Local.bat
    - scripts/tools/run-test-e2e-local/run_test_e2e_local.exe
    - SddIA/tools/run-test-e2e-local/spec.md
    - SddIA/tools/run-test-e2e-local/spec.json
    - SddIA/evolution/b6a79723-db30-461c-94aa-e98e8c1e90ec.md
    - docs/features/create-tool-run-test-e2e-local/objectives.md
    - docs/features/create-tool-run-test-e2e-local/implementation.md
    - docs/features/create-tool-run-test-e2e-local/validacion.md
    - docs/features/create-tool-run-test-e2e-local/finalize.md
  files_modified:
    - scripts/tools-rs/Cargo.toml
    - scripts/tools-rs/install.ps1
    - SddIA/agents/cumulo.paths.json
    - scripts/tools/index.json
    - SddIA/evolution/Evolution_log.md
  files_deleted:
    - docs/features/create-tool-run-test-e2e-local/validacion.json
---

# Validación: create-tool run-test-e2e-local

## Resumen

La herramienta cumple el contrato de tools (salida JSON v2, feedback por fases, sin secretos en `result`). La validación empírica incluyó ejecución contra `http://localhost:5010` con API disponible.

## Notas

- **Rama:** nombre canónico del proceso `feat/create-tool-run-test-e2e-local`; crearla y asociar el commit antes del PR si aún no existe en el remoto.
- **PR:** pendiente de apertura según flujo del equipo (`pr_url` vacío hasta publicar).
