---
fix_id: start-api-client-timeout
artefact: validacion
process: bug-fix
spec_version: 1.0.0
created: 2026-03-26
---

# Validación — start-api / Timeout cliente

## Checks ejecutados

- [x] `cargo build --release` en `scripts/gesfer-capsule`, `scripts/tools-rs`, `scripts/skills-rs`.
- [x] `scripts/tools-rs/install.ps1` y `scripts/skills-rs/install.ps1` — binarios copiados a cápsulas.

## Checks manuales recomendados

1. `.\scripts\tools\start-api\start_api.exe --output-json --no-build` (puerto libre o `--port-blocked kill`) — debe responder JSON sin bloqueo previo en stdin.
2. Durante build real (sin `--no-build`), observar stderr cada ~10 s.
3. Con MySQL caído, `result.error_type` = `database_unavailable` (exit 8).
4. Health con timeout, `result.error_type` = `health_timeout` (exit 7).

## Notas

El **timeout global del cliente** (p. ej. 60 s) puede seguir cortando si el build + health supera ese límite; en ese caso usar `--no-build` tras compilar aparte, o aumentar el timeout del orquestador.
