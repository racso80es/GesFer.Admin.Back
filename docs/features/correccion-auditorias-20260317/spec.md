# Especificación de Corrección - 2026-03-17

## 1. Contexto SddIA
Este documento formaliza el alcance de la corrección derivada de la auditoría `docs/audits/AUDITORIA_2026_03_17.md`.
El proyecto sufre un bloqueo por logueo recursivo de EF Core originado desde `LogQueueLogger`.

## 2. Alcance
* **Nuevas Funcionalidades:** Ninguna.
* **Correcciones:** Filtro condicional en `LogQueueLogger` para evitar logs provenientes de System, Microsoft, o comandos internos de logueo.
* **Proceso Activo:** `SddIA/process/correccion-auditorias`.

## 3. Hoja de Ruta del Ejecutor
1. Consolidar el informe en `AUDITORIA_2026_03_17.md` identificando el Pain Point (crítico).
2. Crear esta documentación (`objectives.md`, `spec.md`, `spec.json`, `validacion.json`).
3. Aplicar el filtro anti-recursión de logs en `LogQueueLogger`.
4. Confirmar que no hay deuda técnica mediante `dotnet test src/GesFer.Admin.Back.sln`.
5. Finalizar el proceso con commit en la rama actual.

## 4. Definition of Done
* El proyecto compila.
* Los tests pasan, incluyendo IntegrationTests.
* El problema del Timeout por log recursivo ha desaparecido de CI.
