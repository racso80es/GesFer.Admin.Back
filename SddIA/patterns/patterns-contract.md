# Contrato de Patrones (SddIA/patterns/)

**Alcance:** Todas las entidades bajo `SddIA/patterns/`.

**Objetivo:** Centralizar el conocimiento de patrones de diseño y arquitectura para ser consumidos por los agentes de SddIA (especialmente `architect` y `tekton-developer`).

---

## 1. Estructura por Patrón

Cada patrón debe residir en una carpeta nombrada con su **UUID**: `SddIA/patterns/<uuid>/`.

### Archivos Obligatorios

1.  **`spec.md`**
    *   **Contenido:** Descripción legible del patrón, propósito, uso, pros/contras, ejemplos.
    *   **Idioma:** Español (es-ES).
    *   **Formato:** Markdown estándar.

2.  **`spec.json`**
    *   **Contenido:** Metadatos estructurados para consumo por agentes.
    *   **Esquema:** Definido en `patterns-contract.json`.
    *   **Campos Clave:**
        *   `id`: UUID del patrón.
        *   `title`: Título del patrón.
        *   `category`: Categoría (e.g., "Arquitectura de Software", "Patrones de Diseño").
        *   `tags`: Etiquetas relevantes.
        *   `metadata`: Objeto con `difficulty` y `status`.
        *   `interested_agents`: Lista de agentes de SddIA interesados en este patrón.

---

## 2. Agentes Interesados

La lista de `interested_agents` en `spec.json` debe basarse en la categoría del patrón:

*   **Arquitectura de Software:** `architect`, `infrastructure-architect`
*   **Patrones de Diseño:** `tekton-developer`, `architect`
*   **Sistemas Distribuidos:** `architect`, `infrastructure-architect`, `performance-engineer`
*   **Domain-Driven Design:** `architect`, `clarifier`
*   **General/Otros:** `architect`, `tekton-developer`

---

## 3. Referencias

*   **Esquema JSON:** `SddIA/patterns/patterns-contract.json`
*   **Proceso de Creación:** `SddIA/process/create-pattern.md`
