# Objetivos: Consolidación — Eliminar referencias a Kalma2

## Objetivo principal

**Eliminar todas las referencias al nombre de proyecto "Kalma2" y a rutas/estructuras asociadas (`src/Kalma2`, `Kalma2/Docs/`, `Kalma2/Interfaces/`) en la solución actual (GesFer.Admin.Back), consolidando la identidad del repositorio y la documentación SddIA.**

## Alcance

- **In scope:** Referencias al **proyecto/sistema Kalma2** (nombre, rutas de código y documentación que aluden a un núcleo "Kalma2" o a interfaces "Kalma2 Desktop/Mobile").
- **Out of scope (por defecto):** El **Karma2Token** (token de seguridad en SddIA) se mantiene; es un concepto distinto y parte del modelo de trazabilidad. Si se desea revisar el token en otro proceso, se tratará aparte.

## Objetivos derivados

1. **Constitución y SddIA**
   - Sustituir "Kalma2" como nombre del proyecto en `SddIA/CONSTITUTION.md` por la identidad del repo (p. ej. GesFer.Admin.Back / GesFer) o por una redacción neutra (ej. "el proyecto").
   - Actualizar los artefactos en `SddIA/constitution/` (constitution.architect, constitution.audity, constitution.cognitive, constitution.duality) para que las rutas y descripciones no referencien `src/Kalma2` ni "Kalma2" como producto, alineándolas a la estructura real del repo (src con proyectos .NET) o marcándolas como referencia histórica/opcional.

2. **Rutas y paths**
   - Eliminar o reemplazar referencias a `src/Kalma2/...`, `Kalma2/Interfaces/Desktop`, `Kalma2/Docs/Feature/` en specs, skills y acciones por rutas canónicas del Cúmulo (paths.featurePath, paths del proyecto actual) o por placeholders coherentes con GesFer.Admin.Back.

3. **Skills y acciones**
   - Ajustar `SddIA/actions/clarify/spec.md` y `SddIA/skills/frontend-test/spec.md` y `spec.json` para que no dependan de rutas ni nombres "Kalma2".
   - Mantener coherencia con Cúmulo (paths) en toda la documentación tocada.

4. **Consistencia SSOT**
   - Garantizar que, tras los cambios, la única fuente de verdad de rutas siga siendo Cúmulo y que no queden rutas literales "Kalma2" en documentos activos.

5. **Trazabilidad**
   - Dejar documentado en esta carpeta (refactorization-consolidacion-kalma2) el análisis, el spec y el plan de cambios para auditoría y evolución.

## Criterios de éxito

- Búsqueda de "Kalma2" (y de rutas `src/Kalma2`, `Kalma2/`) en el repo no devuelve referencias activas en documentación SddIA ni en constitución, salvo que se decida conservar explícitamente alguna mención histórica en un apartado identificado.
- Los contratos y specs que hoy citan rutas Kalma2 referencian en su lugar paths del Cúmulo o la estructura real del proyecto (GesFer.Admin.Back).
- Build y documentación del proyecto siguen siendo coherentes; no se introduce rotura de compilación ni de procesos (las skills que referencian Kalma2 se adaptan o se marcan como N/A para este repo si aplica).

## Nota sobre Karma2

**Karma2** (Karma2Token) es el token de contexto de seguridad/trazabilidad en SddIA. No forma parte de este objetivo de consolidación a menos que se decida explícitamente un proceso aparte para renombrar o sustituir el token.
