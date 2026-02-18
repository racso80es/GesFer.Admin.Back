# Proceso: Feature

Este documento define el **proceso de tarea** para desarrollar una funcionalidad. Está ubicado en `SddIA/process/feature.md`. Las acciones que orquesta están en `SddIA/actions/`. La ruta de persistencia **{persist}** se obtiene de **Cúmulo** (`SddIA/agents/cumulo.json` → `paths.featurePath/<nombre_feature>`).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): solicita/genera en `{persist}/` al menos un **`.md`** (objectives.md, spec.md, clarify.md, plan, etc.) y al menos un **`.json`** (spec.json, clarify.json, implementation.json, validacion.json, etc.).

## Propósito

El proceso **feature** define el procedimiento formal de ciclo completo para desarrollar una funcionalidad o tarea: desde la creación de la rama hasta el cierre y la apertura del Pull Request. Orquesta las acciones **spec**, **clarify**, **planning**, **implementation**, **execution**, **validate** y **finalize** en secuencia, fija la ubicación de la documentación de la tarea y garantiza trazabilidad en los logs de evolución.

Proporciona un flujo repetible y auditado, alineado con las Leyes Universales (soberanía documental en AGENTE_CUMULO, no commits en `master`).

## Alcance del procedimiento

{persist} = AGENTE_CUMULO.featurePath/<nombre_feature>

| Fase | Nombre | Descripción |
| :--- | :--- | :--- |
| **0** | Preparar entorno | Crear rama feat/<nombre_feature> (o `fix/` si aplica) desde `master`. No trabajar en `master`. |
| **1** | Documentación con objetivos | Documentar objetivo, alcance y ley aplicada. La documentación de la tarea se ubica en **{persist}**/objectives.md. |
| **2** | Especificación | Ejecutar o generar SPEC (acción **spec**). Entrada: requerimiento o borrador, {persist}/objectives.md; salida: especificación técnica en `SddIA/actions/spec.md` y copia/canon en {persist}/spec.md y {persist}/spec.json |
| **3** | Clarificación | Ejecutar o generar clarificaciones (acción **clarify**). Especificación técnica: `SddIA/actions/clarify.md`. Entrada: {persist}/objectives.md, {persist}/spec.json; salida: {persist}/clarify.md, {persist}/clarify.json |
| **4** | Planificación | Ejecutar o generar plan (acción **plan**). Entrada: Especificación, Clarificación. Salida: {persist}/plan (y/o clarify según convención). |
| **5** | Implementación | Generar documento de implementación. Especificación técnica: `SddIA/actions/implementation.md`. Entrada: {persist}/objectives.md, {persist}/spec.json, {persist}/clarify.json; salida: {persist}/implementation.md, {persist}/implementation.json |
| **6** | Ejecución | Aplicar el plan al código (Tekton Developer). Especificación técnica: `SddIA/actions/execution.md`. Entrada: {persist}/implementation.json; salida: {persist}/execution.json |
| **7** | Validar | Ejecutar validación pre-PR. Especificación técnica: `SddIA/actions/validate.md`. Entrada: {persist}/; salida: {persist}/validacion.json |
| **8** | Finalizar | Cierre y PR. Especificación técnica: `SddIA/actions/finalize.md`. Entrada: {persist}/; salida: Evolution Logs y Pull Request. |

## Implementación

Este proceso **no** dispone de un único comando de consola; se implementa como **procedimiento** que combina:

*   **Comandos opcionales de consola** (cuando se requiera trazabilidad con token de auditor):
    *   `GesFer.Console --spec --token <AUDITOR_TOKEN> [--prompt <TEXT>] [--title <TITLE>]`
    *   `GesFer.Console --clarify --token <AUDITOR_TOKEN> [--spec-path <PATH>] [--context <TEXT>]`
    *   `GesFer.Console --plan --token <AUDITOR_TOKEN> --spec <PATH> [--clarify <PATH>]`
*   **Ubicación obligatoria de la documentación de la tarea:** `docs/features/<nombre_feature>/` (o `docs/Feature/<nombre_feature>/` según convención del proyecto).

### Contenido mínimo de `docs/features/<nombre_feature>/`

| Documento | Contenido |
| :--- | :--- |
| **objectives.md** (u OBJETIVO.md) | Objetivo, alcance, ley aplicada, resumen del proceso (fases 0–8), cierre y PR, referencias. |
| **spec.md** / SPEC-* | Especificación técnica (puede generarse con `--spec` y copiarse/adaptarse aquí). |
| **clarify.md** / SPEC-*_CLARIFICATIONS.md | Clarificaciones y decisiones (puede generarse con `--clarify` y copiarse aquí). |
| **PLAN-*** | Plan de implementación / task roadmap (puede generarse con `--plan` y copiarse aquí). |

### Actualización de Evolution Logs

Al cierre de la feature (fase 8):

*   **`docs/EVOLUTION_LOG.md`:** Añadir una línea con formato `[YYYY-MM-DD] [feat/<nombre>] [Descripción breve del resultado.] [Estado].`
*   **`docs/evolution/EVOLUTION_LOG.md`:** Añadir una sección con fecha y título de la feature, resumen de acción/alcance/resultado y referencia a `docs/features/<nombre_feature>/objectives.md` (u OBJETIVO.md).

## Integración con Agentes

*   **Arquitecto / Spec Architect:** Puede iniciar el procedimiento y asegurar que la fase 1 (documentación con objetivos) y la ubicación `docs/features/<nombre_feature>/` se respeten.
*   **Clarifier:** Responsable de la fase 3 (clarificación) y de persistir decisiones en el SPEC y en la carpeta de la feature.
*   **Tekton Developer:** Ejecuta las fases 4 (plan), 5 (implementación), 6 (ejecución), 7 (validación) y 8 (cierre/PR); aplica la SPEC como marco legal.
*   **Cúmulo:** Valida que la documentación de la tarea esté en `docs/features/<nombre_feature>/` como SSOT para esa feature.

## Dependencias con otras acciones

*   El proceso **feature** invoca o utiliza los resultados de las acciones **spec**, **clarify**, **plan**, **implementation**, **execution**, **validate** y **finalize** en `SddIA/actions/`.
*   La **documentación de la tarea** (objetivo, spec, clarifications, plan, validacion) debe residir en **`docs/features/<nombre_feature>/`** para aprobación y revisión humana.

## Estándares de Calidad

*   **Grado S+:** Trazabilidad desde el objetivo hasta el PR: rama → docs/features → spec/clarify/plan → implementación → execution → validación → Evolution Logs → PR.
*   **Ley GIT:** Ningún commit en `master`; todo el trabajo en rama `feat/` o `fix/` con documentación en `docs/features/<nombre_feature>/`.
*   **Single Source of Truth:** Para cada feature, la documentación canónica de la tarea es la carpeta `docs/features/<nombre_feature>/`; la referencia en PR y en Evolution Log es esa ruta.

## Alcance para Fix (bug)

El mismo patrón de persistencia se aplica a correcciones de bugs mediante el proceso **bug-fix**. La variable de persistencia y la ubicación de la documentación se obtienen del agente Cúmulo:

**{persist} = AGENTE_CUMULO.fixPath/<nombre_fix>**

(En este repositorio `fixPath` = `./docs/bugs/`, por tanto **{persist}** = `docs/bugs/<nombre_fix>/`.)

| Documento en {persist}/ | Contenido |
| :--- | :--- |
| **objectives.md** | Objetivo del fix, pasos de reproducción, causa raíz (o hipótesis), ley aplicada. |
| **spec.md / spec.json** | Especificación técnica del fix (generada con `--spec --context {persist}`). |
| **clarify.md / clarify.json** | Clarificaciones y decisiones (si aplica). |
| **implementation.md / implementation.json** | Touchpoints y plan de implementación del fix. |
| **validacion.json** | Resultado de la validación pre-PR. |

- Rama: `fix/<nombre_fix>` (nunca `master`).
- En la descripción del PR y en Evolution Logs, la referencia canónica es **`docs/bugs/<nombre_fix>/`** (SSOT para ese fix).
- El agente **Bug Fix Specialist** (`SddIA/process/bug-fix-specialist.json`) orquesta el ciclo del fix y asegura que toda la documentación viva en **{persist}/**.

## Referencia de ejecución

Procedimiento aplicado en la rama **feat/e2e-product-back-mocked** (2026-02-10). Documentación de la tarea: `docs/features/e2e-product-back-mocked/`. Acciones relacionadas: `SddIA/actions/spec.md`, `clarify.md`, `planning.md`, `execution.md`, `validate.md`, `finalize.md`.
