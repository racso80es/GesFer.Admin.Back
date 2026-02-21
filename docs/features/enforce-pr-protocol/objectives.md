# Objectives: Enforce PR Protocol

## Contexto
El sistema actual permite la creación de Pull Requests (PR) sin validar explícitamente una serie de requisitos críticos (compilación, tests, nomenclatura). Esto aplica tanto a agentes de IA (Jules) como a entornos locales (Cursor) y la plataforma de CI (GitHub). El objetivo es asegurar que **ningún PR sea aceptado** sin antes haber pasado un protocolo estricto de validación.

## Objetivos
1.  **Protocolo Unificado:** Definir un protocolo único de aceptación de PR que incluya:
    - Compilación exitosa (.NET y Rust).
    - Ejecución exitosa de todos los tests.
    - Validación de nomenclatura (ramas y commits).
    - Existencia de documentación requerida (SddIA).
2.  **Disparadores Obligatorios:**
    - **Jules / Agentes:** Integrar la validación en la acción `finalize` para que sea imposible cerrar una tarea sin pasar el protocolo.
    - **Cursor:** Configurar reglas locales (`.cursorrules` o hooks) que fuercen la ejecución del protocolo antes de un push/PR.
    - **GitHub:** Implementar un Workflow que ejecute el protocolo en cada PR, bloqueando el merge si falla.
3.  **Implementación Robusta:** Crear una skill en Rust (`verify-pr-protocol`) que encapsule toda la lógica de validación para ser reutilizada en todos los entornos.

## Key Results (KR)
- **KR1:** Skill `verify-pr-protocol` implementada en Rust y funcional.
- **KR2:** Acción `finalize` actualizada para invocar `verify-pr-protocol` antes del push/PR.
- **KR3:** GitHub Action `.github/workflows/pr-validation.yml` operativa y bloqueante.
- **KR4:** Configuración de Cursor (`.cursorrules`) actualizada con la norma.
