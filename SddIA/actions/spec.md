# Action: Spec

## Propósito
La acción **spec** (especificación) constituye el punto de entrada formal del ciclo de desarrollo. Su objetivo es transformar requerimientos brutos, ideas iniciales o necesidades de negocio en Especificaciones Técnicas Formales (SPECS) estructuradas. Proporciona el "Qué" de forma inequívoca, estableciendo la base sobre la cual actuarán las fases de clarificación y planificación.

## Implementación
Esta acción se realiza de forma **manual** siguiendo las pautas descritas a continuación.

### Procedimiento
1.  **Creación del Archivo**: Crear un archivo Markdown en la ruta correspondiente (ver Contexto más abajo).
    -   Nombre sugerido: `SPEC-{TituloBreve}.md`.
2.  **Contenido**: El archivo debe seguir la plantilla estándar de especificaciones (ver `SddIA/templates/spec-template.md` si existe, o basarse en specs existentes).
3.  **Contexto**:
    -   **Para fixes**: La ruta debe ser `{persist}/` (ej. `docs/bugs/admin-back-repeated-failures/`).
    -   **Para features**: La ruta debe ser `docs/features/<nombre_feature>/`.

### Flujo de Ejecución (Manual)
1.  **Identificación**: El auditor o arquitecto identifica la necesidad de una especificación.
2.  **Redacción**: Se redacta el documento asegurando cubrir:
    -   Contexto
    -   Arquitectura
    -   Seguridad
    -   Criterios de Aceptación
3.  **Revisión**: Se revisa el documento para asegurar que no haya ambigüedades (Zero-Ambiguity Rule).
4.  **Persistencia**: Se guarda el archivo en el repositorio.
5.  **Auditoría**: Se debe registrar la creación del documento en `docs/audits/ACCESS_LOG.md`.

## Integración con Agentes
*   **Cúmulo (agente documental):** Define la ubicación de los archivos.
*   **Spec Architect:** Responsable de redactar y formalizar la especificación.
*   **Clarification Specialist:** Revisa la especificación para detectar gaps.
*   **Tekton Developer:** Implementa el código basándose en la especificación aprobada.

## Estándares de Calidad
*   **Grado S+:** Garantiza la trazabilidad total.
*   **Zero-Ambiguity Rule:** Límites del sistema (Scope) claramente definidos.
