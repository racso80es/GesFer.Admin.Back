---
module: Refactorización Geo DTOs
description: "Especificación de corrección de hallazgos de auditoría (Nomenclatura y Convenciones)."
status: active
type: spec
---

# Especificación de Corrección - 2026-03-23

## 1. Contexto SddIA
Este documento formaliza el resultado de la auditoría diaria reportada en `docs/audits/AUDITORIA_2026_03_23.md`, y aborda el único pain point medio detectado en la métrica de **Nomenclatura/Convenciones**.

## 2. Alcance
* **Nuevas Funcionalidades:** Ninguna.
* **Correcciones:** Cambio de tipos de Data Transfer Objects (DTOs) en `src/GesFer.Admin.Back.Application/DTOs/Geo/GeoDtos.cs` de `public class` a `public record`.
* **Proceso Activo:** `SddIA/process/correccion-auditorias`.

## 3. Hoja de Ruta del Ejecutor
1. Identificar y modificar las definiciones de DTOs en `src/GesFer.Admin.Back.Application/DTOs/Geo/GeoDtos.cs` reemplazando `public class` con `public record`.
2. Verificar que el sistema sigue compilando correctamente (Integridad Estructural - The Wall).
3. Confirmar que no hay deuda técnica mediante la ejecución de la suite de pruebas automatizada `dotnet test src/GesFer.Admin.Back.sln`.
4. Registrar el informe de validación en `validacion.md`.
5. Finalizar el proceso con commit en la rama de la tarea.

## 4. Definition of Done
* El proyecto compila.
* Todos los DTOs en `GeoDtos.cs` están definidos como `record`.
* Los tests pasan al 100%, incluyendo UnitTests, IntegrationTests, y E2ETests.
* El informe de validación en `validacion.md` refleja métricas de salud al 100%.