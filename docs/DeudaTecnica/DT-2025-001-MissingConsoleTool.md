# DT-2025-001: Herramienta GesFer.Console Faltante

**Fecha:** 2026-05-22
**Estado:** Abierta
**Impacto:** Medio
**Área:** Herramientas de Desarrollo / Automatización

## Descripción
La documentación de procesos en `SddIA/actions/spec.md` y `SddIA/actions/clarify.md` hace referencia a una herramienta de línea de comandos llamada `GesFer.Console` (`src/Console/GesFer.Console.csproj`) para automatizar la creación de especificaciones y clarificaciones.

Sin embargo, esta herramienta no existe en el código fuente actual (`src/`).

## Consecuencias
- Los procesos de especificación y clarificación deben realizarse manualmente, lo que aumenta el riesgo de inconsistencias y reduce la eficiencia.
- La automatización prometida en `SddIA` no se cumple.

## Acción Correctiva
1.  Desarrollar la herramienta `GesFer.Console` según las especificaciones en `SddIA/actions`.
2.  O bien, actualizar la documentación de `SddIA` para reflejar el proceso manual actual si la herramienta no se va a implementar.

## Referencias
- `SddIA/actions/spec.md`
- `SddIA/actions/clarify.md`
