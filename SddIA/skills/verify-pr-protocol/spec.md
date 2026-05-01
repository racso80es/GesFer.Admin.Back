---
dependencies:
- dotnet
- pwsh
- cargo
implementation_path_ref: paths.skillCapsules.verify-pr-protocol
inputs: []
language: rust
outputs:
- description: 0 for success, non-zero for failure (modo agente 1 nomenclatura, 2 build, 3 tests).
  name: Exit Code
  type: integer
security_context: Karma2Token
skill_id: verify-pr-protocol
---

# Skill: verify-pr-protocol

## Propósito

Esta skill ejecuta de manera atómica los pasos requeridos por el **Protocolo de Aceptación de PR** (ver `SddIA/norms/pr-acceptance-protocol.md`).

## Responsabilidad

- Validar la nomenclatura de la rama actual (invocando `scripts/validate-nomenclatura.ps1`).
- Compilar la solución (.NET).
- Ejecutar todos los tests de la solución.
- Retornar **éxito** solo si todos los pasos son exitosos.

## Implementación

- **Lenguaje:** Rust.
- **Binario:** `verify_pr_protocol.exe` en la raíz de la cápsula (Cúmulo `paths.skillCapsules.verify-pr-protocol`).
- **Fuente:** `scripts/skills-rs/src/bin/verify_pr_protocol.rs`.
- **Instalación en cápsula:** `scripts/skills-rs/install.ps1` (release → copia a `scripts/skills/verify-pr-protocol/`).

## Uso

```powershell
# Desde la raíz del repositorio (humano)
.\scripts\skills\verify-pr-protocol\Verify-PR-Protocol.bat

# Desarrollo / compilación
cargo build --release --manifest-path scripts/skills-rs/Cargo.toml --bin verify_pr_protocol
```

**Agente:** envelope `capsule-json-io` v2 por stdin; salida JSON en stdout. Sin stdin válido: modo consola con logs `[VERIFY-PR-PROTOCOL]`.

## Salidas

- **Modo consola:** logs por fase (1/3, 2/3, 3/3).
- **Modo agente:** JSON según contrato skills; `exitCode` 0 si todo OK; 1 nomenclatura, 2 build, 3 tests.
