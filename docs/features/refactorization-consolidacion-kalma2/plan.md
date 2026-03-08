# PLAN: Consolidación — Eliminar referencias a Kalma2

**Proceso:** refactorization  
**Ruta (Cúmulo):** paths.featurePath/refactorization-consolidacion-kalma2/  
**Especificación:** spec.md | **Clarificación:** clarify.md  

---

## 1. Objetivos del plan

- Eliminar todas las referencias a "Kalma2" y a rutas `src/Kalma2`, `Kalma2/` en los 10 archivos en alcance.
- Consolidar identidad: Constitución del Proyecto (GesFer.Admin.Back); cuerpo con "el proyecto" / "este repositorio (GesFer.Admin.Back)".
- Sustituir rutas inexistentes por Cúmulo/agentes o "N/A (GesFer.Admin.Back)" según clarify.
- **No tocar** SddIA/skills/frontend-test (fuera de alcance). **No tocar** Karma2Token ni karma2-token.

---

## 2. Fases y tareas técnicas

### Fase 1 – Constitución principal

| Id | Tarea | Criterio |
|----|-------|----------|
| 1.1 | Actualizar SddIA/CONSTITUTION.md | Título → "Constitución del Proyecto (GesFer.Admin.Back)". §1: "El proyecto se rige...". §1.1: "Core (el Núcleo)" sin Kalma2. §1.2: "Desktop" / "Interfaz Desktop (cuando exista)"; "Otras interfaces (p. ej. Mobile)". §3: "El proyecto adopta..."; "El sistema carga"; "Configuración... según Cúmulo (paths)." §5: Todas las "Kalma2" → "el proyecto" o "este repositorio". |

### Fase 2 – Constitution architect

| Id | Tarea | Criterio |
|----|-------|----------|
| 2.1 | Actualizar SddIA/constitution/constitution.architect.md | Propósito: "el proyecto". Directrices: "src/" con proyectos Api, Application, Domain, Infrastructure. Estado Actual: GesFer.Admin.Back, src/, no Interface/Desktop; contratos en Domain/Application. |
| 2.2 | Actualizar SddIA/constitution/constitution.architect.json | guidelines: "Code hierarchy under src/ (Api, Application, Domain, Infrastructure)." implementation.location y contractsPath según spec §3.3. |

### Fase 3 – Constitution audity

| Id | Tarea | Criterio |
|----|-------|----------|
| 3.1 | Actualizar SddIA/constitution/constitution.audity.md | Estado Actual: auditoría vía SddIA (paths.auditsPath, agente auditor); Reglas: SddIA/norms y agentes (auditor, process-interaction). |
| 3.2 | Actualizar SddIA/constitution/constitution.audity.json | loggerPath → "N/A (GesFer.Admin.Back); audit via paths.auditsPath, SddIA/agents/auditor." rulesPath → "SddIA/norms, SddIA/agents/auditor." |

### Fase 4 – Constitution cognitive

| Id | Tarea | Criterio |
|----|-------|----------|
| 4.1 | Actualizar SddIA/constitution/constitution.cognitive.md | Propósito: "Dotar al proyecto de...". Estado Actual: persistencia en Infrastructure; no aplica Core/Memory. |
| 4.2 | Actualizar SddIA/constitution/constitution.cognitive.json | storagePath → "N/A (GesFer.Admin.Back); persistence via Infrastructure/DbContext." |

### Fase 5 – Constitution duality

| Id | Tarea | Criterio |
|----|-------|----------|
| 5.1 | Actualizar SddIA/constitution/constitution.duality.md | Estado Actual: no aplica IPC Electron; Modos N/A en este repo. |
| 5.2 | Actualizar SddIA/constitution/constitution.duality.json | ipcBridge → "N/A (GesFer.Admin.Back; backend only)." modesPath → "N/A (GesFer.Admin.Back)." |

### Fase 6 – Acción clarify

| Id | Tarea | Criterio |
|----|-------|----------|
| 6.1 | Actualizar SddIA/actions/clarify/spec.md | Flujo paso 3: "Feature" → carpeta en paths.featurePath (Cúmulo), e.g. paths.featurePath/<nombre_feature>/ (p. ej. docs/features/<nombre_feature>/ en este repo). Mantener lógica de migración; referenciar paths.featurePath/<nombre_feature>/. |

---

## 3. Orden de ejecución recomendado

1. Fase 1 (CONSTITUTION.md).  
2. Fases 2–5 (constitution/*) en cualquier orden; por consistencia: architect → audity → cognitive → duality, .md antes que .json en cada par.  
3. Fase 6 (clarify/spec.md).

---

## 4. Verificación

- Búsqueda: `Kalma2`, `src/Kalma2`, `Kalma2/` no deben aparecer en los 10 archivos modificados (ni en SddIA/constitution ni en CONSTITUTION.md ni en actions/clarify/spec.md).
- No se modifica ningún archivo bajo SddIA/skills/frontend-test.
- No se modifica ninguna referencia a Karma2Token ni karma2-token.
- Build: `dotnet build` sin errores (documentación únicamente; código no tocado).

---

## 5. Alcance cerrado

- **In scope:** 10 archivos (CONSTITUTION.md, constitution.architect/audity/cognitive/duality .md y .json, actions/clarify/spec.md).
- **Out of scope:** frontend-test (clarify 1.2); Karma2Token en todo el repo.

---

*Plan generado a partir de spec.md y clarify.md. Siguiente fase: implementation (doc).*
