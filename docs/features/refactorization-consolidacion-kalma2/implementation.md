# Implementación: Consolidación — Eliminar referencias a Kalma2

**Proceso:** refactorization  
**Ruta (Cúmulo):** paths.featurePath/refactorization-consolidacion-kalma2/  
**Plan:** plan.md | **SPEC:** spec.md | **Clarify:** clarify.md  

---

## Ítems de implementación

Cada ítem: **Id** | **Acción** | **Ruta** | **Ubicación** | **Propuesta** | **Dependencias**

### Fase 1 – Constitución principal

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| IMPL-1.1 | Modificar | SddIA/CONSTITUTION.md | Todo el archivo | Título: `# Constitución del Proyecto (GesFer.Admin.Back)`. §1: "El proyecto se rige..." (o "Este repositorio (GesFer.Admin.Back) se rige..."). §1.1: "Core (el Núcleo)" sin "Kalma2". §1.2: "Desktop" / "Interfaz Desktop (cuando exista)"; "Otras interfaces (p. ej. Mobile)" sin Kalma2 Mobile. §3: "El proyecto adopta..."; "El sistema carga"; "Configuración de proyectos gestionados según Cúmulo (paths)." §5: Sustituir todas las apariciones de "Kalma2" por "el proyecto" o "este repositorio". Mantener Consciencia, Juez, Auditor, Dualidad, Kaizen. | — |

### Fase 2 – Constitution architect

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| IMPL-2.1 | Modificar | SddIA/constitution/constitution.architect.md | Propósito, Directrices, Estado Actual | Propósito: "el proyecto". Directrices: "El código debe residir en `src/`, con proyectos claramente separados (Api, Application, Domain, Infrastructure)." Estado Actual: "En GesFer.Admin.Back: ubicación en `src/` (proyectos .NET). No aplica Interface/Desktop (repo backend). Contratos: interfaces en Domain/Application; convención del proyecto." | — |
| IMPL-2.2 | Modificar | SddIA/constitution/constitution.architect.json | guidelines, implementation | guidelines: "Code hierarchy under src/ (Api, Application, Domain, Infrastructure)." implementation.location: "src/ (GesFer.Admin.Back.*; no aplica Desktop en este repo)". implementation.contractsPath: "Convention: Domain/Application interfaces (paths from Cúmulo when applicable)." | — |

### Fase 3 – Constitution audity

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| IMPL-3.1 | Modificar | SddIA/constitution/constitution.audity.md | Estado Actual | "En GesFer.Admin.Back: auditoría vía SddIA (paths.auditsPath, agente auditor). No aplica Logger.ts/Electron. Reglas: SddIA/norms y agentes (auditor, process-interaction)." | — |
| IMPL-3.2 | Modificar | SddIA/constitution/constitution.audity.json | implementation | loggerPath: "N/A (GesFer.Admin.Back); audit via paths.auditsPath, SddIA/agents/auditor." rulesPath: "SddIA/norms, SddIA/agents/auditor." | — |

### Fase 4 – Constitution cognitive

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| IMPL-4.1 | Modificar | SddIA/constitution/constitution.cognitive.md | Propósito, Estado Actual | Propósito: "Dotar al proyecto de...". Estado Actual: "En GesFer.Admin.Back: persistencia en base de datos (Infrastructure); memoria/contexto según convención del dominio. No aplica Core/Memory (repo backend)." | — |
| IMPL-4.2 | Modificar | SddIA/constitution/constitution.cognitive.json | implementation.storagePath | storagePath: "N/A (GesFer.Admin.Back); persistence via Infrastructure/DbContext." | — |

### Fase 5 – Constitution duality

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| IMPL-5.1 | Modificar | SddIA/constitution/constitution.duality.md | Estado Actual | "En GesFer.Admin.Back no aplica IPC Electron (backend .NET). Modos: N/A en este repo; principio conservado para referencia." | — |
| IMPL-5.2 | Modificar | SddIA/constitution/constitution.duality.json | implementation | ipcBridge: "N/A (GesFer.Admin.Back; backend only)." modesPath: "N/A (GesFer.Admin.Back)." | — |

### Fase 6 – Acción clarify

| Id | Acción | Ruta | Ubicación | Propuesta | Dependencias |
|----|--------|------|-----------|-----------|--------------|
| IMPL-6.1 | Modificar | SddIA/actions/clarify/spec.md | Flujo paso 3 (Determinación de Contexto) | Sustituir: "Si la especificación pertenece a una Feature (`Kalma2/Docs/Feature/`), se asegura que exista una carpeta dedicada para la feature (e.g., `Kalma2/Docs/Feature/{SpecName}/`)." por: "Si la especificación pertenece a una Feature, se asegura que exista una carpeta dedicada en paths.featurePath (Cúmulo), e.g. paths.featurePath/<nombre_feature>/ (p. ej. docs/features/<nombre_feature>/ en este repo)." Mantener: "Si no existe, se crea y se mueve el archivo original allí (migración automática)." Referenciar "carpeta en paths.featurePath/<nombre_feature>/". | — |

---

## Orden de ejecución

1. IMPL-1.1 (CONSTITUTION.md).  
2. IMPL-2.1, IMPL-2.2 (architect).  
3. IMPL-3.1, IMPL-3.2 (audity).  
4. IMPL-4.1, IMPL-4.2 (cognitive).  
5. IMPL-5.1, IMPL-5.2 (duality).  
6. IMPL-6.1 (clarify/spec.md).

No hay dependencias entre ítems; el orden es por fase para facilitar revisión.

---

## Verificación post-ejecución

- Búsqueda en los 10 archivos: no debe quedar "Kalma2", "src/Kalma2" ni "Kalma2/".
- No modificar SddIA/skills/frontend-test. No modificar referencias a Karma2Token/karma2-token.
- `dotnet build` en raíz del repo (opcional; solo documentación tocada).

---

*Documento de implementación para la fase execution. Referencias: spec.md §3, plan.md, clarify.md.*
