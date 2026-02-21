# Objetivo: Crear Items de Seguridad en SddIA/security

## 1. Objetivo Principal
Generar 20 items de seguridad en la ruta `SddIA/security/` siguiendo el patrón arquitectónico de SddIA (encapsulamiento por carpeta UUID, contrato JSON/Markdown). Estos items servirán como base de conocimiento y reglas para el Agente de Seguridad y serán consumidos por Arquitectos, Auditores y Desarrolladores Tekton.

## 2. Alcance
*   Creación de la estructura de directorio `SddIA/security/`.
*   Definición del contrato de seguridad (`security-contract.json` y `security-contract.md`) que incluye la obligatoriedad del contexto `Karma2Token`.
*   Implementación de 20 items de seguridad específicos (desde Inyección SQL hasta Actualizaciones Automáticas) con su respectiva carpeta UUID, `spec.json` y `spec.md`.
*   Mapeo de agentes interesados (`security-engineer`, `architect`, `auditor`, `tekton-developer`) según la categoría del item.

## 3. Ley Aplicada
*   **SddIA Patterns:** Todo item debe residir en su propia carpeta UUID y cumplir con un contrato definido.
*   **Karma2Token:** Todo item de seguridad requiere un contexto de ejecución seguro.
*   **Soberanía Documental:** La documentación de esta feature reside en `docs/features/security-items/`.

## 4. Referencias
*   `SddIA/patterns/patterns-contract.md` (Referencia de estructura)
*   `SddIA/Tokens/karma2-token/spec.json` (Modelo de seguridad)
