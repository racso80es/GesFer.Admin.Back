# DT-2025-001: Herramienta GesFer.Console Faltante (OBSOLETO)

**Estado:** CERRADO - RESUELTO
**Fecha:** 2026-05-22

## Descripción Original
La documentación de procesos en `SddIA/actions/spec.md` y `SddIA/actions/clarify.md` hacía referencia a una herramienta de línea de comandos llamada `GesFer.Console` para automatizar la creación de especificaciones y clarificaciones. Esta herramienta no existía en el código fuente.

## Resolución
Se ha tomado la decisión de **descatalogar** la herramienta `GesFer.Console` y no proceder con su desarrollo.
- Las acciones de `spec`, `clarify` y `planning` se realizarán de forma manual siguiendo las plantillas documentadas.
- La ejecución de tests, que dependía de esta herramienta en `pr-skill.sh`, ha sido reemplazada por una herramienta específica en Rust: `run_tests`.
- Se han actualizado todas las referencias en `SddIA/actions/` y `SddIA/agents/` para reflejar el proceso manual.

## Acciones Realizadas
1.  Eliminación de referencias a `GesFer.Console` en documentación.
2.  Creación de `scripts/skills-rs/src/bin/run_tests.rs`.
3.  Actualización de `scripts/skills/pr-skill.sh`.

Esta deuda técnica se considera saldada.
