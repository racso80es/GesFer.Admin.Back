---
feature_name: create-tool-run-test-e2e-local
created: 2026-04-17
items:
  - id: rust-bin
    action: add
    path: scripts/tools-rs/src/bin/run_test_e2e_local.rs
    location: gesfer-tools binary
    proposal: Pruebas HTTP E2E contra API Admin local (reqwest blocking, envelope capsule-json-io).
  - id: cargo
    action: modify
    path: scripts/tools-rs/Cargo.toml
    location: "[[bin]] run_test_e2e_local + dependencia uuid"
  - id: capsule
    action: add
    path: scripts/tools/run-test-e2e-local/
    location: paths.toolCapsules.run-test-e2e-local
    proposal: manifest, config, doc, bat, exe copiado desde target/release
  - id: sddia-spec
    action: add
    path: SddIA/tools/run-test-e2e-local/
    location: paths.toolsDefinitionPath
  - id: cumulo
    action: modify
    path: SddIA/agents/cumulo.paths.json
    location: toolCapsules.run-test-e2e-local
  - id: index
    action: modify
    path: scripts/tools/index.json
    location: tools[]
  - id: install
    action: modify
    path: scripts/tools-rs/install.ps1
    location: capsules array
  - id: evolution
    action: add
    path: SddIA/evolution/b6a79723-db30-461c-94aa-e98e8c1e90ec.md
    location: paths.sddiaEvolutionPath
---

# Implementación: run-test-e2e-local

## Resumen técnico

- **Binario:** `run_test_e2e_local.exe` (Rust), raíz de cápsula; lectura de petición vía `gesfer_tools::try_read_capsule_request` (stdin, `GESFER_CAPSULE_REQUEST` o CLI).
- **Config por defecto:** `run-test-e2e-local-config.json` (URL base, id/nombre empresa demo, timeout HTTP).
- **Secretos:** preferir `E2E_ADMIN_USER`, `E2E_ADMIN_PASSWORD`, `E2E_INTERNAL_SECRET`; no se emiten en salida JSON.

## Referencias

- Definición: `SddIA/tools/run-test-e2e-local/spec.md`
- Uso: `scripts/tools/run-test-e2e-local/run-test-e2e-local.md`
