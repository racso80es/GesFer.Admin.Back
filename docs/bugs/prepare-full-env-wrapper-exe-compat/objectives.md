---
bug_id: prepare-full-env-wrapper-exe-compat
toolId: prepare-full-env
tipo: bug-fix
objetivo: Garantizar que prepare-full-env se ejecute correctamente desde un entorno Docker limpio, respetando sus parámetros, y que el ejecutable Rust exista en la **raíz** de la cápsula según el estándar SddIA.
fecha: 2026-04-23
---

## Objetivo

- Asegurar que `prepare_full_env.exe` existe en `scripts/tools/prepare-full-env/` tras compilar/instalar tools Rust.
- Asegurar que `Prepare-FullEnv.bat` ejecuta `prepare_full_env.exe` en la **raíz de la cápsula** (sin `bin\`).
- Evitar fallos “no deterministas” al partir de un entorno limpio (sin contenedores): la herramienta debe poder levantar Docker según `DockerOnly` / `StartApi` / `NoDocker`.

