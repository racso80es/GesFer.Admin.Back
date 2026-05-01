---
feature_name: start-api-contract-alignment
branch: feat/start-api-contract-alignment
base_branch: main
global: pass
blocking: false
checks:
  - name: cargo_release_start_api
    result: pass
    message: cargo build --release --bin start_api (gesfer-tools) sin errores
  - name: contract_alignment
    result: pass
    message: Parámetros CLI y request overlay alineados a spec; exitCode 1–8 documentados en código; feedback init/port-check/build/launch/healthcheck/done
  - name: evolution_sddia
    result: pass
    message: SddIA/evolution/e4d5f6a7-b8c9-40d0-9f1a-2b3c4d5e6f7a.md e índice actualizado
  - name: install_capsule
    result: pass
    message: scripts/tools-rs/install.ps1 copió start_api.exe a scripts/tools/start-api/
  - name: e2e_runtime
    result: skipped
    message: No ejecutado en esta sesión; requiere prepare-full-env + invoke-mysql-seeds y API levantando health 200
git_changes:
  files_modified:
    - scripts/tools-rs/src/bin/start_api.rs
    - scripts/tools-rs/Cargo.toml
    - scripts/tools/start-api/start-api-config.json
    - SddIA/tools/start-api/spec.md
    - SddIA/evolution/Evolution_log.md
    - docs/features/start-api-contract-alignment/objectives.md
  files_added:
    - SddIA/evolution/e4d5f6a7-b8c9-40d0-9f1a-2b3c4d5e6f7a.md
    - docs/features/start-api-contract-alignment/implementation.md
    - docs/features/start-api-contract-alignment/execution.md
    - docs/features/start-api-contract-alignment/validacion.md
---

# Validación: start-api-contract-alignment

## Resumen

La herramienta **start-api** compila en release y cumple el diseño acordado en plan/clarify. La verificación de arranque real con **health 200** queda como prueba manual o de CI cuando el entorno tenga base de datos y el `.exe` publicado en la cápsula.

## Criterios spec cubiertos (revisión estática)

- Entradas: config path, no-build, profile, port, port-blocked, output-json/path, merge con JSON camelCase/PascalCase/snake_case.
- Prioridad `portBlocked` CLI > request > config.
- Puertos 2 y 3; directorio API 4; build 5; launch 6; health 7; MySQL 8.
- Heartbeat en stderr durante `dotnet build`.

## Recomendación pre-PR

1. `.\scripts\tools-rs\install.ps1` para actualizar `scripts/tools/start-api/start_api.exe`.
2. Smoke local una vez con infra lista; revisar que ningún cliente dependa del stub (`working_dir`/`command` eliminados del contrato público).
