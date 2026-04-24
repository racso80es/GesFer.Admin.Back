---
bug_id: prepare-full-env-wrapper-exe-compat
toolId: prepare-full-env
alcance: minimo
causa_raiz: La cápsula no garantiza la existencia del ejecutable `prepare_full_env.exe` en la **raíz** (estándar SddIA). Si el exe no está presente, el wrapper falla y no se puede preparar el entorno desde cero.
criterios_aceptacion:
  - El ejecutable `scripts/tools/prepare-full-env/prepare_full_env.exe` existe tras compilar/instalar tools Rust (`scripts/tools-rs/install.ps1`).
  - El wrapper `Prepare-FullEnv.bat` **solo** ejecuta el exe en la raíz de la cápsula (no `bin\`).
  - Si falta el exe en raíz, el error es explícito y apunta a `scripts/tools-rs/README.md`.
fecha: 2026-04-23
---

## Contexto

`prepare-full-env` debe ser la puerta de entrada “estable” para levantar un entorno Docker desde cero. El estándar SddIA exige que los binarios `.exe` estén en la **raíz** de la cápsula (sin `bin\`).

