---
feature_name: prepare-full-env-drop-start-api
created: 2026-04-28
purpose: Cerrar decisiones de alcance y coherencia de contrato
decisions:
  - id: D1
    decision: "StartApi/--start-api se elimina por completo; prepare-full-env no arranca la API."
  - id: D2
    decision: "Auditorías en docs/audits/ son inmutables; no se editan."
  - id: D3
    decision: "NoDocker no implica arranque de API; solo fases no-Docker (p. ej. clientes) si config lo define."
---

# Clarify: prepare-full-env-drop-start-api

## Preguntas resueltas

### ¿Qué significa “stra api” en este cambio?

Es el **arranque de la API** que estaba representado por `StartApi` / `--start-api` y por referencias a fase `api`.

### ¿Se actualizan auditorías previas?

No. `docs/audits/` es evidencia histórica y no se modifica.

## Implicaciones

- Hay que alinear **definición SddIA**, **cápsula** (manifest/doc/config/launcher) y **código Rust** para que no quede ninguna referencia a API ni a `api` como fase.

