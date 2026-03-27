---
fix_id: start-api-client-timeout
title: "start-api falla desde cliente con Timeout"
process: bug-fix
paths_ref: Cúmulo paths.fixPath
spec_version: 1.0.0
created: 2026-03-26
---

# Objetivos

## Problema

La herramienta **start-api** se invoca desde un **cliente** (orquestador, IDE, agente) y el cliente reporta **Timeout** antes de recibir respuesta JSON.

## Metas

1. Identificar y eliminar causas **evitables** de espera indefinida o de duración superior al tiempo razonable del cliente.
2. Emitir **salida de progreso** en fases largas (sobre todo `dotnet build`) para que supervisores con “watchdog por actividad” no corten la ejecución.
3. Devolver en el JSON de resultado **códigos y campos estructurados** (`error_type`, mensajes accionables) cuando falle health, BD o build, en lugar de un fallo genérico.

## Fuera de alcance

- Aumentar el timeout del propio cliente (producto externo); sí documentar tiempos típicos y parámetros (`healthCheckTimeoutSeconds`, `--no-build`).

## Estado

Implementación aplicada en `gesfer-capsule` (peek stdin + argv), `start_api` (stderr de progreso + `error_type` en `result`). Detalle: `spec.md` y `validacion.md`.
