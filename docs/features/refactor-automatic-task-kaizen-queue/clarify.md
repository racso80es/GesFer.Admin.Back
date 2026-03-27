---
feature_name: refactor-automatic-task-kaizen-queue
created: 2026-03-27
purpose: Cerrar ambigüedades del SPEC (cola Kaizen, empates, rutas, logs) antes de planning.
decisions:
  - id: CL-001
    topic: Nombre de la subcarpeta de cola Kaizen
    decision: >-
      Usar exactamente `KAIZEN/` bajo `paths.tasksPath` (convención fijada en `SddIA/process/automatic_task/spec.md` v1.1.0). No alternar `Kaizen/` u otras variantes en documentación nueva.
  - id: CL-002
    topic: Empate de antigüedad en `KAIZEN/`
    decision: >-
      Si dos o más ficheros comparten la misma fecha deducible (mismo prefijo `Kaizen_YYYY_MM_DD` o mismo `created` en frontmatter), elegir el candidato con **orden lexicográfico ascendente del nombre de fichero completo** (desempate determinista).
  - id: CL-003
    topic: Orden de lectura fecha — nombre vs frontmatter
    decision: >-
      Preferir fecha extraída del **nombre** cuando siga el patrón `Kaizen_YYYY_MM_DD*.md`. Si el nombre no permite deducir fecha, usar `created` o `date` en frontmatter YAML del `.md`. Si ambos existen y discrepan, **priorizar el nombre de fichero** como fuente de verdad para la cola Kaizen (coherencia con el listado por sistema de ficheros).
  - id: CL-004
    topic: Kaizen históricos en la raíz de `paths.tasksPath`
    decision: >-
      Sin migración automática en v1. Los `.md` tipo Kaizen que sigan en raíz se tratan como **bandeja principal** (§1.1). Para alimentar solo la cola, deben **moverse manualmente** a `KAIZEN/` cuando alguien las encola; opcional documentar en `plan.md` un one-shot de limpieza si el repo tiene casos reales.
  - id: CL-005
    topic: Log en finalización (§4) vs evolution SddIA
    decision: >-
      El resumen de cierre de tarea automática va al log de evolución **de producto** indicado en Cúmulo (`paths.evolutionPath` / `paths.evolutionLogFile`). Los cambios normativos bajo `./SddIA/` siguen el protocolo `paths.sddiaEvolutionPath` y no sustituyen este cierre de tarea.
  - id: CL-006
    topic: SSOT del procedimiento
    decision: >-
      El texto normativo ya actualizado es `SddIA/process/automatic_task/spec.md` (v1.1.0). El `spec.md` bajo `paths.featurePath/refactor-automatic-task-kaizen-queue/` describe la intención y validación; en planning se sincroniza cierre de redacción (estado, referencias cruzadas) sin duplicar reglas contradictorias.
spec_ref: docs/features/refactor-automatic-task-kaizen-queue/spec.md
process_ref: SddIA/process/automatic_task/spec.md
status: cerrada
---

# Clarificación — Cola Kaizen en automatic_task

Documento de **clarify** para alinear criterios operativos antes de **planning**. Las decisiones **CL-001 … CL-006** dejan sin bloqueos el `plan.md` salvo tareas opcionales de limpieza documental.

---

## 1. Contexto

El proceso **automatic_task** ya incorpora el triaje en tres niveles (raíz → `KAIZEN/` por antigüedad → nueva Kaizen). Esta clarificación fija reglas que el SPEC dejaba abiertas (empates, legacy en raíz, logs).

---

## 2. Decisiones cerradas

| ID | Tema | Resolución breve |
| :--- | :--- | :--- |
| CL-001 | Carpeta cola | `paths.tasksPath/KAIZEN/` |
| CL-002 | Misma antigüedad | Desempate: orden lexicográfico del nombre de fichero |
| CL-003 | Nombre vs frontmatter | Preferir fecha en nombre `Kaizen_YYYY_MM_DD*`; si no hay, frontmatter; si conflicto, manda el nombre |
| CL-004 | Kaizen en raíz | No auto-migración; manual u one-shot opcional |
| CL-005 | Evolution log | Producto: `paths.evolutionPath` / `paths.evolutionLogFile`; SddIA: ruta y contrato propios |
| CL-006 | SSOT | `SddIA/process/automatic_task/spec.md` v1.1.0 |

---

## 3. Gaps resueltos respecto al borrador de `spec.md` (feature)

1. **Herramienta automatizada de ordenación:** confirmado fuera de v1; el ejecutor aplica CL-002 y CL-003 manualmente o con reglas explícitas en texto.
2. **“Opcional: AGENTS / normas”:** sigue siendo opcional; planning puede listar referencias si alguna norma aún cita solo `docs/TASKS` literal o Kaizen solo en raíz.

---

## 4. Próximo paso

Redactar **`plan.md`**: fases de verificación (lectura cruzada con Cúmulo), sincronización del `spec.md` de la feature (estado, alineación con v1.1.0), tareas opcionales de migración manual o difusión en normas, y criterios de validación final (`validacion.md`).
