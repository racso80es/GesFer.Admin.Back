# SPEC: Consolidación — Eliminar referencias a Kalma2

**Proceso:** refactorization  
**Ruta (Cúmulo):** paths.featurePath/refactorization-consolidacion-kalma2/  
**Referencias:** objectives.md, analysis.md  

---

## 1. Requerimientos

1. Eliminar o sustituir todas las referencias al **nombre de proyecto "Kalma2"** y a **rutas/estructuras** `src/Kalma2`, `Kalma2/Docs/`, `Kalma2/Interfaces/` en SddIA y constitución.
2. Consolidar la identidad del repositorio como **GesFer.Admin.Back** (o "el proyecto" en redacción neutra) en la Constitución y principios constitucionales.
3. En artefactos que describen "Estado Actual" o rutas de implementación inexistentes en este repo (p. ej. Electron, Desktop, Core/Memory), sustituir por rutas canónicas del **Cúmulo** (paths.*) o por indicación explícita de **no aplica a GesFer.Admin.Back**.
4. En **acciones** y **skills**, reemplazar referencias a `Kalma2/Docs/Feature/` y `Kalma2/Interfaces/Desktop` por paths.featurePath (Cúmulo) o por estructura/scripts aplicables a este repositorio.
5. **No modificar** ninguna referencia a **Karma2Token** ni a **karma2-token** (contratos, normas, agentes).

---

## 2. Reglas de sustitución

| Contexto | Sustituir | Por |
|----------|-----------|-----|
| Título / nombre del proyecto | "Kalma2", "Constitución del Proyecto: Kalma2" | "Constitución del Proyecto" o "Constitución del Proyecto (GesFer.Admin.Back)" |
| Párrafos descriptivos | "Kalma2" como sujeto (Core, MCP, Consciencia, etc.) | "el proyecto" o "este repositorio (GesFer.Admin.Back)" |
| Rutas de código inexistentes | `src/Kalma2`, `src/Kalma2/Interface/Desktop`, `src/Kalma2/Core/*` | Según caso: (a) `src/` + proyectos .NET actuales (Api, Application, Domain, Infrastructure) cuando el principio aplique; (b) "No aplica a GesFer.Admin.Back" o referencia a Cúmulo cuando no exista equivalente |
| Docs / Features | `Kalma2/Docs/Feature/`, `Kalma2/Docs/Feature/{SpecName}/` | `paths.featurePath` (Cúmulo); en texto: "carpeta de la feature en paths.featurePath (Cúmulo)" o "paths.featurePath/<nombre_feature>/" |
| Skills frontend (Desktop) | `Kalma2/Interfaces/Desktop`, "Vitest para Kalma2 Desktop" | En este repo no existe interfaz Desktop; usar "N/A para GesFer.Admin.Back" o `cwd` según estructura real si hubiera frontend; en descripción: "Frontend (cuando aplique; en Admin.Back no existe Desktop)" |

---

## 3. Touchpoints (plan de implementación)

### 3.1 SddIA/CONSTITUTION.md

| Ubicación | Cambio |
|-----------|--------|
| Título (línea 1) | `# Constitución del Proyecto: Kalma2` → `# Constitución del Proyecto (GesFer.Admin.Back)` |
| §1 (línea 4) | "El proyecto Kalma2 se rige..." → "El proyecto se rige..." (o "Este repositorio (GesFer.Admin.Back) se rige...") |
| §1.1 (líneas 6–10) | "Kalma2 Core" → "Core (el Núcleo)"; mantener descripción genérica sin nombre Kalma2. |
| §1.2 (líneas 14–22) | "Calma2 Desktop" → "Desktop" o "Interfaz Desktop (cuando exista)"; "Kalma2 Mobile" → "Otras interfaces (p. ej. Mobile)"; eliminar nombre Kalma2. |
| §3 (líneas 33–35) | "Kalma2 adopta..." → "El proyecto adopta..."; "Kalma2 carga" → "El sistema carga"; "Kalma2/Projects/..." → "Configuración de proyectos gestionados según Cúmulo (paths)." |
| §5 (líneas 44–77) | Todas las apariciones de "Kalma2" → "el proyecto" o "este repositorio". Mantener contenido de Consciencia, Juez, Auditor, Dualidad, Kaizen; solo cambiar el sujeto nominal. |

### 3.2 SddIA/constitution/constitution.architect.md

| Ubicación | Cambio |
|-----------|--------|
| Propósito (línea 4) | "Kalma2" → "el proyecto". |
| Directrices (línea 10) | "El código debe residir en `src/Kalma2`..." → "El código debe residir en `src/`, con proyectos claramente separados (Api, Application, Domain, Infrastructure)." |
| Estado Actual (líneas 14–17) | "Ubicación: `src/Kalma2/Interface/Desktop`" → "En GesFer.Admin.Back: ubicación en `src/` (proyectos .NET). No aplica Interface/Desktop (repo backend)."; "Contratos: `src/Kalma2/Core/Contracts`" → "Contratos: interfaces en Domain/Application; convención del proyecto." |

### 3.3 SddIA/constitution/constitution.architect.json

| Campo | Cambio |
|-------|--------|
| `guidelines` (ítem con src/Kalma2) | "Code hierarchy under src/Kalma2 (Interface vs Core)." → "Code hierarchy under src/ (Api, Application, Domain, Infrastructure)." |
| `implementation.location` | "src/Kalma2/Interface/Desktop" → "src/ (GesFer.Admin.Back.*; no aplica Desktop en este repo)". |
| `implementation.contractsPath` | "src/Kalma2/Core/Contracts" → "Convention: Domain/Application interfaces (paths from Cúmulo when applicable)." |

### 3.4 SddIA/constitution/constitution.audity.md

| Ubicación | Cambio |
|-----------|--------|
| Estado Actual (líneas 15–16) | "Logger: `src/Kalma2/Core/Audit/Logger.ts`" → "En GesFer.Admin.Back: auditoría vía SddIA (paths.auditsPath, agente auditor). No aplica Logger.ts/Electron."; "Reglas: `src/Kalma2/Core/Audit/Rules`" → "Reglas: SddIA/norms y agentes (auditor, process-interaction)." |

### 3.5 SddIA/constitution/constitution.audity.json

| Campo | Cambio |
|-------|--------|
| `implementation.loggerPath` | "src/Kalma2/Core/Audit/Logger.ts" → "N/A (GesFer.Admin.Back); audit via paths.auditsPath, SddIA/agents/auditor." |
| `implementation.rulesPath` | "src/Kalma2/Core/Audit/Rules" → "SddIA/norms, SddIA/agents/auditor." |

### 3.6 SddIA/constitution/constitution.cognitive.md

| Ubicación | Cambio |
|-----------|--------|
| Propósito (línea 4) | "Dotar a Kalma2 de..." → "Dotar al proyecto de...". |
| Estado Actual (líneas 15–16) | "Almacén: `src/Kalma2/Core/Memory`" → "En GesFer.Admin.Back: persistencia en base de datos (Infrastructure); memoria/contexto según convención del dominio. No aplica Core/Memory (repo backend)." |

### 3.7 SddIA/constitution/constitution.cognitive.json

| Campo | Cambio |
|-------|--------|
| `implementation.storagePath` | "src/Kalma2/Core/Memory" → "N/A (GesFer.Admin.Back); persistence via Infrastructure/DbContext." |

### 3.8 SddIA/constitution/constitution.duality.md

| Ubicación | Cambio |
|-----------|--------|
| Estado Actual (líneas 15–16) | "IPC: Implementado en `src/Kalma2/Desktop/electron/ipc`" → "En GesFer.Admin.Back no aplica IPC Electron (backend .NET)."; "Modos: `src/Kalma2/Core/Modes`" → "Modos: N/A en este repo; principio conservado para referencia." |

### 3.9 SddIA/constitution/constitution.duality.json

| Campo | Cambio |
|-------|--------|
| `implementation.ipcBridge` | "src/Kalma2/Desktop/electron/ipc" → "N/A (GesFer.Admin.Back; backend only)." |
| `implementation.modesPath` | "src/Kalma2/Core/Modes" → "N/A (GesFer.Admin.Back)." |

### 3.10 SddIA/actions/clarify/spec.md

| Ubicación | Cambio |
|-----------|--------|
| Flujo paso 3 (línea 18) | "Si la especificación pertenece a una Feature (`Kalma2/Docs/Feature/`), se asegura que exista una carpeta dedicada para la feature (e.g., `Kalma2/Docs/Feature/{SpecName}/`)." → "Si la especificación pertenece a una Feature, se asegura que exista una carpeta dedicada en paths.featurePath (Cúmulo), e.g. paths.featurePath/<nombre_feature>/ (p. ej. docs/features/<nombre_feature>/ en este repo)." |
| Mismo paso, continuación | "Si no existe, se crea y se mueve el archivo original allí (migración automática)." → Mantener lógica; referenciar "carpeta en paths.featurePath/<nombre_feature>/". |

**Fuera de alcance:** SddIA/skills/frontend-test (spec.md, spec.json) — no se modifican en esta refactorización (decisión clarify 1.2).

---

## 4. Criterios de aceptación

- [ ] No queda ninguna referencia activa a "Kalma2" como nombre de proyecto en SddIA/CONSTITUTION.md ni en SddIA/constitution/*.
- [ ] No quedan rutas literales `src/Kalma2` ni `Kalma2/` en los 10 archivos tocados (constitución, clarify; frontend-test excluido).
- [ ] Las referencias a carpetas de features usan paths.featurePath (Cúmulo) o redacción equivalente en clarify/spec.md.
- [ ] Ninguna modificación afecta a Karma2Token ni a karma2-token.
- [ ] Build del proyecto y documentación siguen coherentes; no se introduce rotura.

---

## 5. Restricciones

- **NO** modificar referencias a **Karma2Token**, **karma2-token**, ni contratos/reglas que los mencionen (skills-contract, tools-contract, actions-contract, process-contract, etc.).
- **NO** eliminar principios de la Constitución (Consciencia, Juez, Auditor, Dualidad, Kaizen); solo sustituir el nombre "Kalma2" y las rutas obsoletas.
- Entorno: Windows 11 + PowerShell 7+; rutas según Cúmulo (paths.*).

---

*Especificación técnica de refactorización. Cumple interfaz de proceso (spec.md + spec.json).*
