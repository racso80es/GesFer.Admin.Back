# Clarificaciones para SPEC-RESOLVE-AUDIT-DEBT

**Fecha:** 2026-02-21
**Estado:** Resuelto

## Preguntas y Respuestas

### 1. Alcance y Priorización
*   **Pregunta:** ¿El alcance incluye solo la documentación o también la implementación? ¿Se deben priorizar los hallazgos críticos (compilación, arquitectura)?
*   **Respuesta (Usuario):** Sí a objetivos y especificaciones. Priorizar compilación y arquitectura crítica.

### 2. Nomenclatura de Feature
*   **Pregunta:** ¿Nombre de la feature? Propuesto: `docs/features/resolve-audit-debt`.
*   **Respuesta (Usuario):** OK.

### 3. Proceso
*   **Pregunta:** ¿Seguir proceso SddIA estricto (objectives -> spec -> clarify -> plan)?
*   **Respuesta (Usuario):** Sí, haciendo uso de acciones SddIA/actions.

### 4. Suposiciones Técnicas (Validadas en Plan)
*   **Suposición:** `LogController` está en `Api` y depende de `AdminDbContext` (Infrastructure). **Validado:** Sí, código revisado.
*   **Suposición:** Falta carpeta `DTOs/Logs` en `Application`. **Validado:** Sí, error de compilación lo confirma.
*   **Suposición:** Interfaces de servicio están en `Infrastructure`. **Validado:** El reporte de auditoría indica que las interfaces están definidas en `Infrastructure` o la implementación se usa directamente. Se extraerán interfaces a `Application`.

## Decisiones Clave
1.  **Prioridad Máxima:** Lograr que `dotnet build` funcione.
2.  **Arquitectura:** Usar `IApplicationDbContext` para desacoplar `Api` de `Infrastructure` (a nivel de controlador) y `Application` de `Infrastructure`.
