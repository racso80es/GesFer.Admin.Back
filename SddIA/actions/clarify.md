# Action: Clarify

## Propósito
La acción **clarify** (clarificación) es la fase crítica de eliminación de ambigüedades. Su objetivo es interrogar la especificación (`SPEC`) para detectar "gaps", suposiciones no verificadas o riesgos técnicos antes de que se escriba una sola línea de código.

## Implementación
Esta acción se realiza de forma **manual** mediante revisión y discusión (o auto-reflexión del agente).

### Procedimiento
1.  **Análisis**: Revisar la especificación (`SPEC`) existente.
2.  **Identificación de Gaps**: Listar preguntas, dudas o puntos ambiguos.
3.  **Resolución**: Responder a las preguntas mediante investigación o consulta con el usuario.
4.  **Documentación**:
    -   Crear un archivo `CLARIFY-{Titulo}.md` (o `clarify.md` dentro de la carpeta de la feature/fix).
    -   Incluir las preguntas y respuestas.
    -   Incluir las decisiones tomadas.

### Flujo de Ejecución (Manual)
1.  **Lectura Profunda**: El agente Clarifier lee la especificación.
2.  **Generación de Preguntas**: Se formulan preguntas tipo "¿Qué pasa si...?", "¿Cómo se maneja el error X?", etc.
3.  **Iteración**: Se resuelven las dudas.
4.  **Persistencia**: Se guarda el resultado en la carpeta correspondiente (`docs/features/...` o `docs/bugs/...`).
5.  **Auditoría**: Registrar la acción en `docs/audits/ACCESS_LOG.md`.

## Integración con Agentes
*   **Clarification Specialist:** Ejecuta esta acción.
*   **Spec Architect:** Provee la especificación base.
*   **Tekton Developer:** Utiliza las clarificaciones para ajustar la implementación.

## Estándares de Calidad
*   **Zero-Ambiguity Rule:** No deben quedar preguntas abiertas críticas antes de pasar a planificación.
