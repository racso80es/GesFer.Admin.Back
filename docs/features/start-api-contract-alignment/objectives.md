---
feature_name: start-api-contract-alignment
created: 2026-05-01
process: feature
branch: feat/start-api-contract-alignment
status: validada-local
---

# Objetivos: start-api-contract-alignment

## Objetivo principal

Alinear la implementación ejecutable de la herramienta **start-api** (`paths.toolCapsules.start-api`, binario Rust) con el contrato canónico en **SddIA** (`SddIA/tools/start-api/spec.md`) y con la documentación de cápsula (`scripts/tools/start-api/start-api.md`), eliminando la deriva actual (stub que solo hace `spawn` de un comando sin healthcheck, puerto, build ni parámetros contractuales).

## Alcance

- **Dentro:** `scripts/tools-rs` (binario `start_api`), artefactos de cápsula bajo `scripts/tools/start-api/` si aplica, y ajustes documentales mínimos (p. ej. ruta de fuente en spec SddIA si sigue desactualizada).
- **Fuera:** Cambios al contrato funcional de la tool en SddIA salvo correcciones menores de coherencia (typos, rutas); nuevas herramientas; CI ajeno a esta tool.

## Ley aplicada

- Soberanía documental: la spec **SddIA/tools/start-api/spec.md** es la referencia de comportamiento y E/S.
- Comandos y git: operaciones de consola y git solo vía skills/proceso cuando corresponda (fases posteriores del proceso feature).

## Rama

Crear y trabajar en **`feat/start-api-contract-alignment`** tras **git-workspace-recon** y **git-branch-manager** (Cúmulo `paths.skillCapsules.git-branch-manager`), sin commits en `main`/`master`.

## Artefactos de proceso

Seguir `SddIA/process/feature/spec.md`: `spec.md` → `clarify.md` → `plan.md` → `implementation.md` → ejecución → `validacion.md` → cierre. Esta carpeta es la SSOT de la tarea (`paths.featurePath`).
