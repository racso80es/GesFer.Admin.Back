---
feature_name: prepare-full-env-drop-start-api
created: 2026-04-28
base: main
scope:
  tool_id: prepare-full-env
  areas:
    - SddIA/tools definition
    - tool capsule (scripts/tools)
    - Rust tool implementation (scripts/tools-rs)
---

# Especificación: prepare-full-env-drop-start-api

## Contexto

La tool `prepare-full-env` incluía una opción de “arrancar la API” (`StartApi` / `--start-api`) y referencias a una fase `api`. Esta funcionalidad queda fuera de su responsabilidad: **preparar entorno** debe limitarse a Docker + verificación MySQL + (opcional) clientes definidos por configuración.

## Requisitos

### R1 — Eliminación de parámetro de API

- No debe existir `StartApi` en la spec de `SddIA/tools/prepare-full-env/spec.md`.
- La cápsula (`scripts/tools/prepare-full-env/`) no debe documentar ni anunciar `-StartApi`/`--start-api`.

### R2 — Contrato de fases/feedback coherente

- El contrato de fases no debe incluir `api` en `phases_feedback`.
- La implementación no debe emitir entradas `feedback[].phase = "api"`.

### R3 — Configuración sin `startApi`

- La configuración de la cápsula (`prepare-env.json`) no debe incluir bloque `startApi`.
- La documentación de configuración debe retirar campos `startApi.*`.

### R4 — Compilación e instalación

- `scripts/tools-rs` debe compilar en Windows (msvc) y producir `prepare_full_env.exe`.
- `scripts/tools-rs/install.ps1` debe poder copiar el ejecutable a `scripts/tools/prepare-full-env/prepare_full_env.exe`.

## Fuera de alcance

- No modificar `docs/audits/**` (evidencia histórica).

