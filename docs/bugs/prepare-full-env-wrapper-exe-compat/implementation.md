---
bug_id: prepare-full-env-wrapper-exe-compat
toolId: prepare-full-env
cambios:
  - scripts/tools/prepare-full-env/Prepare-FullEnv.bat
  - scripts/tools/prepare-full-env/prepare-env.md
  - SddIA/tools/prepare-full-env/spec.md
fecha: 2026-04-23
---

## Implementación

- **Wrapper**: `scripts/tools/prepare-full-env/Prepare-FullEnv.bat`
  - Ejecuta `prepare_full_env.exe` **solo** en la raíz de la cápsula.
  - Si no existe, devuelve un error explícito y referencia `scripts/tools-rs/README.md`.

- **Documentación**: `scripts/tools/prepare-full-env/prepare-env.md`
  - Se corrige la explicación del wrapper para reflejar el comportamiento real.
  - Se aclara que la interfaz de automatización/MCP es el `.exe`.

- **Spec SddIA**: `SddIA/tools/prepare-full-env/spec.md`
  - Se ajusta la ubicación del binario a la raíz de la cápsula y la ruta de fuente Rust.

- **Implementación Rust**: `scripts/tools-rs/src/bin/prepare_full_env.rs`
  - Se añade el binario Rust para que `scripts/tools-rs/install.ps1` pueda compilarlo y copiarlo a la cápsula.

