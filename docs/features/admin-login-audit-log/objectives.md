---
feature_name: admin-login-audit-log
created: 2026-03-14
process: feature
---

# Objetivo: Registrar en AuditLogs el evento de login Admin

## Objetivo principal

Siempre que se realice login en Admin, registrar en la tabla `AuditLogs` la información relevante del suceso (usuario, cursorId, acción, método HTTP, path, timestamp).

## Requisito de entrada

**El controlador NO será el encargado de esta responsabilidad.** La lógica de registro en AuditLogs debe residir en una capa inferior (Application/Infrastructure), no en el controlador.

## Decisiones confirmadas

| # | Decisión | Valor |
|---|----------|-------|
| 1 | Refactor CQRS | Sí — AdminLoginCommand + Handler |
| 2 | Audit intentos fallidos | Sí — registrar también 401 (credenciales inválidas) |
| 3 | IP y User-Agent en AdditionalData | Sí |
| 4 | Validación | **4b** — Validación en el Handler |

## Objetivos secundarios

1. **Refactor CQRS:** Crear `AdminLoginCommand` y `AdminLoginHandler`; controlador solo envía command.
2. **Audit exitoso y fallido:** Registrar en AuditLogs tanto login exitoso (200) como intentos fallidos (401) con acción diferenciada (ej. "LoginSuccess" / "LoginFailed").
3. **Datos adicionales:** Incluir IP y User-Agent en `AdditionalData` (JSON) del AuditLog.
4. **Validación:** En el Handler (Usuario/Contraseña no vacíos); sin FluentValidation.

## Alcance

- **Backend:** GesFer.Admin.Back (Application, Api).
- **Tabla:** AuditLogs (existente).
- **Servicio:** IAuditLogService.LogActionAsync (existente).

## Ley aplicada

- **SddIA/process/feature:** Ciclo completo de feature.
- **Clean Architecture:** El controlador no orquesta lógica de negocio ni efectos secundarios (audit).
- **CQRS (recomendado):** MediatR para casos de uso; el handler es responsable del flujo completo.

## Clarificación (punto 4) — Cerrada

Decisión: **4b** — Validación en el Handler. Ver `clarify.md` y `clarify.json`.

## Análisis previo

Ver **analysis.md** en esta carpeta para:
- Estructura actual del login (sin Commands, sin MediatR).
- Comparación con otros controladores.
- Opciones de diseño (A: Command/Handler, B: Pipeline Behavior, C: Decorador, D: Evento, E: Filtro).
- Recomendación: Opción A (AdminLoginCommand + Handler).
- Preguntas abiertas para definir objetivos finales.

## Resumen del proceso

1. **Fase 0:** Preparar entorno (rama `feat/admin-login-audit-log`).
2. **Fase 1:** Documentación con objetivos (este archivo) — **completada**.
3. **Fase 2-4:** Spec, Clarify, Plan.
4. **Fase 5-6:** Implementación y Ejecución.
5. **Fase 7-8:** Validación y Finalizar.

## Referencias

- `docs/features/admin-login-audit-log/analysis.md`
- `docs/features/admin-login-audit-log/clarify.md`
- `docs/features/admin-login-audit-log/spec.md`
- `docs/features/admin-login-audit-log/spec.json`
- `docs/features/admin-login-audit-log/clarify.json`
- `src/GesFer.Admin.Back.Api/Controllers/AdminAuthController.cs`
- `src/GesFer.Admin.Back.Application/Commands/Logs/CreateAuditLogCommand.cs`
- `src/GesFer.Admin.Back.Infrastructure/Services/AuditLogService.cs`
- `SddIA/process/feature/spec.md`
