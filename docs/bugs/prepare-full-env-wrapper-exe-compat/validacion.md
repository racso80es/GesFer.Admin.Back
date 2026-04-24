---
bug_id: prepare-full-env-wrapper-exe-compat
toolId: prepare-full-env
fecha: 2026-04-23
validacion:
  estado: pendiente
  notas: "Validación técnica completada (alineación contrato/ubicación). Validación de ejecución pendiente: requiere compilar/instalar tools Rust para generar scripts/tools/prepare-full-env/prepare_full_env.exe en la raíz de la cápsula, y Docker Desktop."
---

## Validación

### Validación técnica (sin ejecutar)

- Wrapper `.bat` solo busca el exe en raíz de cápsula.
- Implementación Rust existe en `scripts/tools-rs/src/bin/prepare_full_env.rs`.
- Documentación actualizada: salida v2 usa `result` (no `data`) y no referencia `.ps1`/`bin\`.

### Caso A: exe en raíz

- Colocar `prepare_full_env.exe` en `scripts/tools/prepare-full-env/`.
- Ejecutar `scripts/tools/prepare-full-env/Prepare-FullEnv.bat -DockerOnly`.
- Esperado: el wrapper imprime `[Usando prepare_full_env.exe]` y la tool procede según parámetros.

### Caso B: exe ausente

- Asegurar que no existe `prepare_full_env.exe` en la raíz de la cápsula.
- Ejecutar `scripts/tools/prepare-full-env/Prepare-FullEnv.bat -DockerOnly`.
- Esperado: error explícito indicando que no se encontró el exe y referencia a `scripts/tools-rs/README.md`.

