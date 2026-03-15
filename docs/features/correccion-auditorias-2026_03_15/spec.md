# Especificación de Corrección - 2026-03-15

## 1. Contexto SddIA
Este documento formaliza el alcance de la corrección derivada de la auditoría `docs/audits/AUDITORIA_2026_03_15.md`.
El estado del proyecto se vio afectado por un error en `LogController.cs` donde `GetAuditLogsQuery` no existía, y se procedió a implementarlo.

## 2. Alcance
* **Nuevas Funcionalidades:** Ninguna.
* **Correcciones:** Implementación de `GetAuditLogsQuery` y su handler `GetAuditLogsHandler`.
* **Proceso Activo:** `SddIA/process/correccion-auditorias`.

## 3. Hoja de Ruta del Ejecutor
1. Consolidar el informe en `AUDITORIA_2026_03_15.md`.
2. Crear esta documentación (`objectives.md`, `spec.md`, `spec.json`, `validacion.json`).
3. Implementar la clase de `GetAuditLogsQuery` requerida por la API.
4. Confirmar que no hay deuda técnica mediante `dotnet test src/GesFer.Admin.Back.sln`.
5. Finalizar el proceso con commit en la rama actual.

## 4. Definition of Done
* El proyecto compila.
* Los tests pasan.
* No hay hallazgos pendientes en la auditoría del día.