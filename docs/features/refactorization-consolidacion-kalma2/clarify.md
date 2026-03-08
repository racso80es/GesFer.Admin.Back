# Clarificación: Consolidación — Eliminar referencias a Kalma2

**Proceso:** refactorization  
**Ruta (Cúmulo):** paths.featurePath/refactorization-consolidacion-kalma2/  
**Especificación:** spec.md  

---

## 1. Gaps y ambigüedades resueltas

### 1.1 Nombre del proyecto en la Constitución

**Gap:** ¿Usar "GesFer.Admin.Back" explícito o redacción neutra "el proyecto" en CONSTITUTION.md?

**Resolución:**  
- **Título:** `# Constitución del Proyecto (GesFer.Admin.Back)` — identidad explícita del repositorio.  
- **Cuerpo:** Usar **"el proyecto"** como sujeto en la mayoría de los párrafos (mejor legibilidad). Usar **"este repositorio (GesFer.Admin.Back)"** solo cuando convenga identificación inequívoca (p. ej. §1 intro, o secciones "Estado Actual" en constitution/*).  
- No dejar ninguna aparición de "Kalma2" como nombre del proyecto.

### 1.2 Skill frontend-test: eliminar del alcance

**Decisión:** La skill **frontend-test** (SddIA/skills/frontend-test) queda **fuera del alcance** de esta refactorización. No se modifican `spec.md` ni `spec.json` de dicha skill. Las referencias a Kalma2 en frontend-test podrán abordarse en un proceso posterior si se desea.

### 1.3 Rutas "Estado Actual" en constitution/* sin equivalente en repo

**Gap:** ¿Sustituir por texto "N/A" genérico o por referencia explícita a Cúmulo/agentes?

**Resolución:** Usar **referencia explícita** cuando exista equivalente conceptual (p. ej. auditoría → paths.auditsPath, agente auditor; persistencia → Infrastructure/DbContext). Usar **"N/A (GesFer.Admin.Back)"** o **"No aplica en este repositorio"** cuando no exista (IPC Electron, Core/Memory, Core/Modes). Objetivo: que un lector entienda qué aplica aquí y qué es herencia de otro contexto.

### 1.4 Clarify/spec.md: redacción exacta para paths.featurePath

**Gap:** ¿Mantener "paths.featurePath (Cúmulo)" en prosa o usar ruta literal docs/features?

**Resolución:** En **SddIA/actions/clarify/spec.md** usar **"paths.featurePath (Cúmulo)"** y, entre paréntesis, **"p. ej. docs/features/<nombre_feature>/ en este repo"**. Así se cumple SSOT (Cúmulo como fuente) y se da una pista concreta a quien lee en GesFer.Admin.Back.

---

## 2. Sin gaps pendientes

No se identifican ambigüedades adicionales que bloqueen la fase de planning o implementation. Las reglas de sustitución y touchpoints del spec.md quedan acotados por estas resoluciones.

---

## 3. Resumen para planning

| Tema | Decisión |
|------|----------|
| Título CONSTITUTION.md | Constitución del Proyecto (GesFer.Admin.Back) |
| Sujeto en párrafos | "el proyecto"; "este repositorio (GesFer.Admin.Back)" cuando convenga |
| frontend-test | Fuera de alcance; no modificar en esta refactorización |
| Estado Actual constitution/* | Referencia Cúmulo/agentes si hay equivalente; "N/A (GesFer.Admin.Back)" si no |
| clarify/spec.md Feature path | paths.featurePath (Cúmulo); ej. docs/features/<nombre_feature>/ |

---

*Clarificación generada para refactorization-consolidacion-kalma2. Cumple interfaz de proceso (clarify.md + clarify.json).*
