---
feature_name: refactorization-skills-exe-root-json-io
created: 2026-03-20
items_applied:
  - id: IMP-01
    path: scripts/gesfer-capsule/
    action: Nuevo crate envelope JSON compartido
  - id: IMP-02
    path: scripts/skills-rs/
    action: Dependencia gesfer-capsule; bins con JSON + legacy TTY; install.ps1 → raíz cápsula
  - id: IMP-03
    path: scripts/tools-rs/
    action: Dependencia gesfer-capsule; emit CapsuleResponse; stdin JSON en 3 tools; limpieza Cargo.toml bins inexistentes
  - id: IMP-04
    path: scripts/skills/
    action: manifest v2, index.json, README, .bat, Push-And-CreatePR.bat; eliminados exes obsoletos en bin/
  - id: IMP-05
    path: scripts/tools/
    action: .bat sin fallback ps1 donde hay Rust; Start-Api, Prepare-FullEnv, Invoke-MySqlSeeds, Run-Tests-Local, Postman-Mcp-Validation
  - id: IMP-06
    path: scripts/skills/Invoke-Command.json
    action: location → invoke_command.exe
---

# Implementation (aplicado): contrato cápsula v2

## Resumen técnico

| Área | Cambio |
|------|--------|
| SSOT JSON | `SddIA/norms/capsule-json-io.md` |
| Rust compartido | `scripts/gesfer-capsule` (`gesfer-capsule`) |
| Skills | Exe en `scripts/skills/<id>/*.exe`; IA usa stdin/stdout JSON |
| Tools | Mismo envelope; `result` sustituye `data` en serialización v2 |
| Humanos | `.bat` ejecuta `.exe` con argumentos CLI existentes |

## Verificación manual realizada

- `invoke_command.exe` con JSON vía pipe devuelve `CapsuleResponse` válido (prueba desde raíz del repo).

## Compilación

- `cargo build --release` en `scripts/skills-rs` y `scripts/tools-rs` — OK.
- `install.ps1` en ambos proyectos — copia a cápsulas.

---

*Artefacto implementation según proceso feature.*
