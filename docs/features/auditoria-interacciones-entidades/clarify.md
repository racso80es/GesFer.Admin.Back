# CLARIFY: Auditoría de Interacciones entre Entidades

**Feature:** auditoria-interacciones-entidades
**Fecha:** 2026-02-24
**Autor:** Jules (Agent)

## Entendimiento del Requerimiento

El objetivo es implementar la auditoría de interacciones entre entidades del ecosistema SddIA (skills, tools, actions, process) y corregir las deficiencias arquitecturales y de compilación detectadas en el reporte de auditoría `AUDITORIA_2026_02_24.md`.

### Puntos Clave

1.  **The Wall (Compilación):** El backend `GesFer.Admin.Back` no compila debido a la falta de comandos y queries CQRS (`CreateLogCommand`, `CreateAuditLogCommand`, `GetLogsQuery`, `PurgeLogsCommand`) que son invocados en `LogController`.
2.  **Violación de Clean Architecture:** El proyecto `Api` referencia directamente a `Infrastructure` y paquetes de implementación de base de datos y logging (`Serilog.Sinks.MySQL`, `Pomelo.EntityFrameworkCore.MySql`). Esto debe ser refactorizado para invertir las dependencias.
3.  **Fuga de Lógica:** El controlador `LogController` contiene lógica de negocio que debe residir en la capa de Aplicación (Handlers/Validators).
4.  **Feature Auditoría:** La implementación de estos comandos y la refactorización habilitarán la funcionalidad de auditoría de interacciones descrita en `spec.md`.

## Suposiciones Confirmadas

1.  Se deben crear los artefactos de documentación faltantes (`clarify.md`, `plan.md`) en `docs/features/auditoria-interacciones-entidades/`.
2.  Se deben corregir los puntos detectados en `AUDITORIA_2026_02_24.md`.
3.  El objetivo final es tener el backend compilando y respetando Clean Architecture.

## Riesgos

-   **Regresiones:** Al mover la configuración de Serilog y la base de datos a `Infrastructure`, se debe asegurar que la aplicación siga iniciando correctamente y conectando a los servicios.
-   **Dependencias:** Asegurar que todos los paquetes necesarios estén disponibles en `Infrastructure` y eliminados de `Api`.

## Criterios de Aceptación

-   `GesFer.Admin.Back.sln` compila sin errores.
-   `LogController` no contiene lógica de negocio ni referencias directas a entidades o `DbContext`.
-   `GesFer.Admin.Back.Api` no tiene referencia a `GesFer.Admin.Back.Infrastructure` en su `.csproj`.
-   La configuración de Serilog y base de datos se realiza en la capa de `Infrastructure` y se expone vía métodos de extensión.
-   Los comandos `CreateLogCommand`, `CreateAuditLogCommand`, `PurgeLogsCommand` y la query `GetLogsQuery` están implementados en `Application`.
