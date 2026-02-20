# Clarificación: Rutas vía Cúmulo (unificación en SddIA)

**Contexto:** Análisis en `alcance-rutas.md`. Rama `feat/sddia-paths-cumulo`. Persist: `docs/features/sddia-ecosystem-independence/`.

**Objetivo de esta clarificación:** Fijar decisiones y prioridades antes de plan/implementación para que "rutas solo desde Cúmulo" quede sin ambigüedades.

---

## 1. Decisiones a confirmar

### 1.1 Ampliación de Cúmulo (paths nuevos)

| Clave propuesta | Valor | Decisión |
|-----------------|--------|----------|
| evolutionPath | ./docs/evolution/ | ¿Añadir? (recomendado: sí) |
| auditsPath | ./docs/audits/ | ¿Añadir? (recomendado: sí) |
| architecturePath | ./docs/architecture/ | ¿Añadir o unificar con infrastructure en techPath? |
| infrastructurePath | ./docs/infrastructure/ | ¿Añadir o techPath único? |
| debtPath | ./docs/DeudaTecnica/ | ¿Añadir? |
| tasksPath | ./docs/tasks/ | ¿Añadir? |
| actionsPath | ./SddIA/actions/ | ¿Añadir para homogeneidad o dejar "SddIA/actions/" como contexto? |
| processPath | ./SddIA/process/ | ¿Añadir o dejar como contexto? |
| normsPath | ./SddIA/norms/ | ¿Añadir o dejar como contexto? |
| skillsRustPath | ./scripts/skills-rs | ¿Añadir (opcional) o no; contrato sin ruta? |
| toolsRustPath | ./scripts/tools-rs | ¿Añadir (opcional) o no? |
| evolutionLogFile | EVOLUTION_LOG.md (relativo a evolutionPath) | ¿Clave explícita o siempre evolutionPath + nombre? |
| accessLogFile | ACCESS_LOG.md (relativo a auditsPath) | ¿Clave explícita o siempre auditsPath + nombre? |

**Propuesta mínima (fase 1):** Añadir solo `evolutionPath` y `auditsPath`; sustituir en SddIA todas las apariciones de `docs/evolution/` y `docs/audits/` por paths. Dejar ficheros concretos como convención (evolutionPath + "EVOLUTION_LOG.md").  
**Propuesta completa:** Añadir todas las claves anteriores y que las instructions de Cúmulo usen solo paths.*.

### 1.2 Rutas dentro de SddIA (actions, process, norms)

- **Opción A:** No añadir actionsPath, processPath, normsPath; en SddIA se sigue usando "SddIA/actions/", "SddIA/process/" como nombre de contexto (no como ruta de disco que deba resolver Cúmulo).
- **Opción B:** Añadir actionsPath, processPath, normsPath en Cúmulo y que toda referencia en SddIA pase por paths.* (máxima homogeneidad).

**Recomendación:** Opción A para reducir cambios; Opción B si se quiere "cero rutas literales" incluso dentro del árbol SddIA.

### 1.3 Orden de sustitución (prioridad)

1. **Fase 1:** Cúmulo + instructions: añadir evolutionPath, auditsPath; cambiar Map [EVO] y Map [AUD] a paths.evolutionPath y paths.auditsPath.
2. **Fase 2:** Actions (finalize, validate, execution, spec, clarify, planning, implementation): sustituir docs/evolution/, docs/audits/, docs/features/ por paths.*.
3. **Fase 3:** Process (feature.md, bug-fix-specialist.json, create-tool): sustituir literales por paths.*.
4. **Fase 4:** Skills y agents: sustituir scripts/skills/... y docs/audits/... por paths.skillCapsules, paths.auditsPath.
5. **Fase 5:** Norms, tools, constitution: revisión final de literales.

¿Confirmar este orden o priorizar solo Cúmulo + actions (Fase 1 y 2)?

### 1.4 Feature / proceso

- **Opción A:** Este trabajo es una **extensión** de la feature "sddia-ecosystem-independence" (misma carpeta persist, misma norma "desacoplar SddIA del ecosistema").
- **Opción B:** Es una **feature/proceso separada** (ej. "sddia-paths-cumulo") con su propia carpeta en docs/features/sddia-paths-cumulo/ y objetivos/spec/validación propios.

**Recomendación:** Opción A (misma feature, documento de alcance y clarificación en la carpeta existente); la rama puede seguir siendo feat/sddia-paths-cumulo para trazabilidad.

---

## 2. Preguntas abiertas

1. ¿Ficheros concretos (EVOLUTION_LOG.md, ACCESS_LOG.md) como claves en Cúmulo (paths.evolutionLogFile, paths.accessLogFile) o siempre path base + convención de nombre?
2. ¿Incluir en esta tarea la sustitución en **AGENTS.md** de cualquier ruta literal restante por "consultar Cúmulo" o dejarlo para una pasada posterior?
3. ¿Norma explícita en SddIA/norms (ej. "Toda ruta de fichero en SddIA se obtiene de Cúmulo paths") en esta misma feature o en una norma genérica posterior?

---

## 3. Resumen de clarificación

- **Alcance:** Unificar uso de rutas en SddIA: Cúmulo como única fuente; eliminar rutas literales docs/..., scripts/... en actions, process, skills, agents, norms.
- **Mínimo viable:** evolutionPath + auditsPath en Cúmulo; instructions actualizadas; actions (finalize, validate, etc.) y process (feature, bug-fix) usando paths.*.
- **Pendiente de confirmación:** Lista exacta de claves paths a añadir; orden de fases; si SddIA interno (actions/process/norms) va a paths.* o se queda como contexto; y si el trabajo es extensión de sddia-ecosystem-independence o feature nueva.

Cuando se confirmen las decisiones anteriores, se puede generar el plan (planning) y la implementación por fases.
