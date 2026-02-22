# Contrato de Seguridad (SddIA/security/)

**Alcance:** Todas las entidades bajo `SddIA/security/`.

**Objetivo:** Centralizar el conocimiento, políticas y prácticas de seguridad para ser consumidos y aplicados por los agentes de SddIA (especialmente `security-engineer`, `auditor`, `architect` y `tekton-developer`).

---

## 1. Estructura por Item de Seguridad

Cada item de seguridad debe residir en una carpeta nombrada con su **UUID**: `paths.securityPath/<uuid>/` (Cúmulo). La ruta canónica se obtiene de Cúmulo (paths.securityPath).

### Archivos Obligatorios

1.  **`spec.md`**
    *   **Contenido:** Descripción detallada del concepto de seguridad, vulnerabilidad, riesgo o política, junto con sus estrategias de mitigación.
    *   **Idioma:** Español (es-ES).
    *   **Formato:** Markdown estándar con encabezados claros (e.g., Descripción, Riesgo, Mitigación, Referencias).

2.  **`spec.json`**
    *   **Contenido:** Metadatos estructurados para consumo por agentes.
    *   **Esquema:** Definido en `security-contract.json`.
    *   **Campos Clave:**
        *   `id`: UUID del item.
        *   `title`: Título del item.
        *   `category`: Categoría (e.g., "Ciberseguridad", "DevSecOps", "Seguridad de Aplicaciones").
        *   `tags`: Etiquetas relevantes.
        *   `metadata`: Objeto con `difficulty` (Beginner, Intermediate, Advanced) y `status`.
        *   `interested_agents`: Lista de agentes de SddIA interesados en este item.

---

## 2. Agentes Interesados

La lista de `interested_agents` en `spec.json` debe basarse en la categoría del item y su impacto:

*   **Ciberseguridad (Ofensiva/Defensiva):** `security-engineer`, `auditor`, `architect`.
*   **DevSecOps / Infraestructura:** `tekton-developer`, `infrastructure-architect`, `security-engineer`.
*   **Seguridad de Aplicaciones:** `architect`, `tekton-developer`, `security-engineer`.
*   **Educación / Formación:** `security-engineer`, `auditor`.

---

## 3. Modelo de Seguridad (Karma2Token)

Todos los items de seguridad operan bajo el contexto de `Karma2Token`. Cualquier acción de creación, lectura o aplicación de estos items debe estar firmada y validada según `SddIA/Tokens/karma2-token/spec.json`.

---

## 4. Referencias

*   **Esquema JSON:** paths.securityPath + security-contract.json (Cúmulo)
*   **Ruta:** paths.securityPath (consultar SddIA/agents/cumulo.paths.json). Custodio: security-engineer.
