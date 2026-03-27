---
feature_name: refactor-automatic-task-kaizen-queue
created: 2026-03-27
purpose: Hoja de ruta ejecutable tras clarify; cierre documental y validación sin código aplicativo obligatorio.
spec_ref: docs/features/refactor-automatic-task-kaizen-queue/spec.md
clarify_ref: docs/features/refactor-automatic-task-kaizen-queue/clarify.md
plan_version: "1.0"
phases:
  - id: P1
    name: Verificación SSOT (automatic_task vs clarify)
  - id: P2
    name: Sincronizar documentación de la feature
  - id: P3
    name: Difusión opcional y carpeta KAIZEN
  - id: P4
    name: Implementation / execution / validación / finalize
---

# Plan — Cola Kaizen en automatic_task

Plan alineado con **clarify.md** (CL-001 … CL-006) y con el estado real del repo: **`SddIA/process/automatic_task/spec.md` ya está en v1.1.0**; el trabajo restante es **coherencia documental, comprobaciones y cierre** del ciclo feature.

---

## Contexto Cúmulo (recordatorio)

| Clave | Valor en `cumulo.paths.json` |
| :--- | :--- |
| `paths.tasksPath` | `./docs/tasks/` |
| `paths.evolutionPath` | `./docs/evolution/` |
| `paths.evolutionLogFile` | `EVOLUTION_LOG.md` |
| `paths.sddiaEvolutionPath` | `./SddIA/evolution/` (solo cambios normativos SddIA; ver CL-005) |

---

## Fase P1 — Verificación SSOT

| ID | Tarea | Criterio de hecho |
| :--- | :--- | :--- |
| P1.1 | Leer `SddIA/process/automatic_task/spec.md` (v1.1.0) y contrastar con **CL-001** (carpeta `KAIZEN/`), **CL-002** (empate lexicográfico implícito en “más antiguo”), **CL-003** (nombre vs frontmatter). | Sin contradicción; si falta una frase explícita sobre empate, añadir **una línea** en el proceso o dejar solo en clarify (aceptable si el proceso dice “más antiguo” y clarify refina). |
| P1.2 | Confirmar que §4 Finalización cita log de producto vía Cúmulo (`paths.evolutionPath` / `paths.evolutionLogFile`) alineado con **CL-005**. | Texto presente en `automatic_task/spec.md`. |

**Nota:** Registro evolution **SddIA** por el cambio ya realizado en `SddIA/process/`: existe `SddIA/evolution/f8e2d4c1-7b3a-4f9e-8c6d-1a2b3c4d5e6f.md`. No duplicar entrada salvo nueva modificación bajo `./SddIA/`.

---

## Fase P2 — Sincronizar documentación de la feature

| ID | Tarea | Entregable |
| :--- | :--- | :--- |
| P2.1 | Actualizar `spec.md` de la feature: §4 “Cambios en artefactos SSOT” debe reflejar **estado completado** (proceso ya actualizado a v1.1.0; enlace al fichero). | `docs/features/refactor-automatic-task-kaizen-queue/spec.md` |
| P2.2 | Ajustar frontmatter del `spec.md` (`status: planned` o `ready_for_validation` según convención del equipo). | Mismo fichero |
| P2.3 | Mantener `objectives.md` al día (fase Planning hecha). | `objectives.md` |

---

## Fase P3 — Difusión opcional y carpeta `KAIZEN/`

| ID | Tarea | Notas |
| :--- | :--- | :--- |
| P3.1 | **Opcional:** `grep` / búsqueda en `AGENTS.md`, `SddIA/norms/`, `.cursor/rules` por `docs/TASKS`, “Kaizen” solo en raíz, o literales que contradigan `paths.tasksPath`. | Solo si aparece contradicción; entonces PR pequeño de difusión (acción **sddia-difusion** si aplica). |
| P3.2 | **Opcional:** crear `docs/tasks/KAIZEN/` vacía con **`.gitkeep`** para que Git versione la carpeta. | No obligatorio: el proceso funciona sin carpeta hasta que exista el primer fichero Kaizen en cola. |

---

## Fase P4 — Implementation, execution, validación, finalize

| ID | Tarea | Notas |
| :--- | :--- | :--- |
| P4.1 | **`implementation.md`:** lista tocados reales — al menos `SddIA/process/automatic_task/spec.md`, `SddIA/evolution/*` (registro previo), carpeta feature; sin cambios en ensamblados .NET salvo alcance ampliado explícito. | Doc único; frontmatter YAML |
| P4.2 | **`execution.md`:** breve registro de lo aplicado en el repo (o “sin cambios adicionales tras plan” si P1–P3 no alteran más ficheros). | Idem |
| P4.3 | **`validacion.md`:** checklist — proceso legible, Cúmulo con `tasksPath`, clarify y plan enlazados, sin duplicar reglas en conflicto. | Idem |
| P4.4 | **Finalize:** rama `feat/refactor-automatic-task-kaizen-queue` (skill **iniciar-rama**), commits convencionales, PR, skill **finalizar-git** según proceso. | Ley GIT |

---

## Orden sugerido

```text
P1 → P2 → (P3 si aplica) → P4.1 → P4.2 → P4.3 → P4.4
```

---

## Riesgos y mitigación

| Riesgo | Mitigación |
| :--- | :--- |
| Rama aún no creada | Ejecutar **iniciar-rama** antes de commits de cierre. |
| Doble registro evolution SddIA | Solo un detalle por intervención; ampliar solo si se vuelve a tocar `./SddIA/` en esta misma feature. |

---

## Definition of Done (feature)

- `automatic_task` v1.1.0 vigente y coherente con **clarify**.
- Documentación en `paths.featurePath/refactor-automatic-task-kaizen-queue/` completa hasta **validacion.md** (y finalize según política del repo).
- PR fusionado o en revisión con descripción que cite esta feature y el proceso `automatic_task`.
