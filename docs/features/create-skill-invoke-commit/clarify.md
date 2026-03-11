# Clarificaciones: create-skill-invoke-commit

**Rama:** feat/create-skill-invoke-commit  
**Persistencia:** paths.featurePath/create-skill-invoke-commit/

## Decisiones pendientes (consulta al usuario)

### C1. Alcance: ¿Incluir pr-create en invoke-commit?

**Contexto:** La skill podría cubrir solo **commit** o también **creación de PR** con --body parametrizable.

| Opción | Descripción | Impacto |
|--------|-------------|---------|
| **A** | Solo commit (add + commit) | Más simple; PR sigue con finalizar-git / push_and_create_pr |
| **B** | Commit + pr-create (push + gh pr create con --body) | Unifica flujo; push_and_create_pr ya tiene --title pero no --body explícito |

**Pregunta:** ¿La skill invoke-commit debe incluir la creación del PR con parámetro --body, o solo el commit?

---

### C2. push_and_create_pr: ¿Añadir --body para descripción del PR sin fichero?

**Contexto (ampliado):** Dado que invoke-commit será **solo commit** (C1), la creación de PRs sigue en **finalizar-git** (push_and_create_pr.exe). Actualmente:

- **--persist:** Si se pasa, el body del PR es `Documentación: ``{persist}```.
- **Sin --persist:** El body es `Rama: {rama}`.
- **No existe --body:** No se puede pasar una descripción libre (ej. resumen de cambios, checklist) sin generar un fichero.

**Problema:** Para flujos automatizados que quieren un body rico (resumen, referencias, checklist), hoy habría que escribir a un .txt y pasarlo de alguna forma — o conformarse con el body mínimo.

**Opciones:**

| Opción | Descripción | Cuándo se usa | Alcance |
|--------|-------------|---------------|---------|
| **A** | **Extender push_and_create_pr con --body** | Al crear PR (acción finalize, skill finalizar-git). Nuevo parámetro `--body "texto"`. Si se proporciona, se usa; si no, fallback a persist o rama. | Modificación en finalizar-git (misma tarea o tarea separada). |
| **B** | **Mantener solo --persist** | Sin cambios. El body sigue siendo construido automáticamente. | Ninguno. Se acepta la limitación. |
| **C** | **--body y --body-file** | `--body` para texto corto; `--body-file <ruta>` para cuerpos largos (evita límites de línea de comandos). | Mayor flexibilidad; más complejidad. |

**Recomendación:** Opción A (--body) cubre el caso "evitar fichero txt" para la mayoría de escenarios. Opción C solo si se prevén bodies muy largos.

**Pregunta:** ¿Extendemos push_and_create_pr con --body en esta tarea, lo dejamos para otra, o mantenemos solo --persist?

---

### C3. Formato de --files: ¿Lista separada por espacios o por comas?

**Contexto:** Para `--files file1 file2 file3` vs `--files "file1,file2,file3"`.

| Opción | Ejemplo | Ventaja |
|--------|---------|---------|
| **A** | `--files "a.md" "b.json"` (múltiples --files) | Más flexible |
| **B** | `--files "a.md,b.json"` (string separado por coma) | Un solo parámetro |
| **C** | `--file` repetido: `--file a.md --file b.json` | Estándar en muchos CLIs |

**Pregunta:** ¿Qué formato prefieres para la lista de archivos?

---

### C4. Integración con invoke-command

**Contexto:** invoke-command es obligatoria para comandos de sistema. invoke-commit sería una skill especializada.

| Opción | Descripción |
|--------|-------------|
| **A** | invoke-commit se invoca directamente; no pasa por invoke-command (es una skill de alto nivel) |
| **B** | invoke-commit internamente usa invoke-command para el git commit (doble capa) |

**Recomendación:** Opción A. invoke-commit es una skill que ejecuta git; no necesita pasar por invoke-command. Ambas son skills del mismo nivel.

**Pregunta:** ¿Confirmas que invoke-commit se invoca directamente, sin pasar por invoke-command?

---

### C5. Nombre de la skill: invoke-commit vs git-commit

| Opción | Pros | Contras |
|--------|------|---------|
| **invoke-commit** | Consistente con invoke-command; sugiere "invocar" un commit | Puede confundir con invoke-command |
| **git-commit** | Más explícito; dominio claro | Menos alineado con convención "invoke-*" |

**Pregunta:** ¿Prefieres **invoke-commit** o **git-commit** como skill_id?

---

## Decisiones tomadas

| ID | Decisión | Valor |
|----|----------|-------|
| **C1** | Alcance invoke-commit | **Solo commit** (funcionalidad atómica) |
| **C3** | Formato --files | **Lista separada por coma** |
| **C4** | Invocación directa | **Sí** — invoke-commit sustituye a invoke-command para commits |
| **C5** | skill_id | **invoke-commit** |

## Decisiones tomadas (C2)

| ID | Decisión | Valor |
|----|----------|-------|
| **C2** | push_and_create_pr | **A + C:** --body para texto directo; --body-file para cuerpos largos (evita límites de línea de comandos) |

---

*Todas las clarificaciones resueltas. Listo para planificación e implementación.*
