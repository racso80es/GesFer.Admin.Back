# Objetivos: Corrección según Auditorías (2026-02)

**Proceso:** correccion-auditorias  
**Ruta (Cúmulo):** paths.featurePath/correccion-segun-auditorias/  
**Rama sugerida:** feat/correccion-segun-auditorias  
**Ley aplicada:** Ley GIT (trabajo en feat/), Ley COMPILACIÓN (código que compila), Soberanía documental (docs/audits/).

---

## 1. Objetivo

Corregir los hallazgos críticos y medios reportados en las últimas auditorías del proyecto (paths.auditsPath), priorizando la compilación, la seguridad y el respeto a la arquitectura de capas (Clean Architecture / Dependency Inversion).

---

## 2. Fuentes analizadas

| Auditoría | Fecha | Ubicación |
| :--- | :--- | :--- |
| Auditoría S+ Backend | 2026-02-21 | docs/audits/AUDITORIA_2026_02_21.md |
| Reporte Auditoría S+ | 2026-02-20 | docs/audits/AUDITORIA_2026_02_20.md |
| Reporte Auditoría S+ | 2026-02-19 | docs/audits/AUDITORIA_2026_02_19.md |
| Auditoría carpeta scripts | 2026-02-19 | docs/audits/AUDITORIA_CARPETA_SCRIPTS_20260219.md |
| Validación main (hallazgos previos) | 2026-02-17 | docs/features/correccion-hallazgos-auditoria/audit-hallazgos.json (aplicados) |

---

## 3. Hallazgos consolidados por prioridad

### 3.1 Críticos (bloqueantes)

| Id | Hallazgo | Ubicación | Auditoría |
| :--- | :--- | :--- | :--- |
| C1 | **Compilación rota:** Faltan DTOs de Logs en Application (`CreateLogDto`, `CreateAuditLogDto`, `LogDto`, `LogsPagedResponseDto`, `PurgeLogsResponseDto`). LogController referencia namespace/clases inexistentes. | src/GesFer.Admin.Back.Application/, LogController.cs | 2026-02-21, 2026-02-20 |
| C2 | **Credenciales hardcoded:** Cadena de conexión con contraseña en texto plano como fallback en Program.cs. | src/GesFer.Admin.Back.Api/Program.cs (aprox. L29) | 2026-02-21 |
| C3 | **Acoplamiento Api → Infraestructura:** LogController inyecta y usa AdminDbContext directamente. | src/GesFer.Admin.Back.Api/Controllers/LogController.cs | 2026-02-20 |
| C4 | **Application → Infrastructure:** La capa Application tiene referencia directa a Infrastructure; violación Dependency Inversion. | GesFer.Admin.Back.Application.csproj | 2026-02-20, 2026-02-19 |
| C5 | **Interfaces en Infraestructura:** Interfaces (IAuthService, IJwtService, etc.) definidas en Infrastructure en lugar de Application. | Infrastructure/Services/ | 2026-02-19 |

### 3.2 Medios

| Id | Hallazgo | Ubicación | Auditoría |
| :--- | :--- | :--- | :--- |
| M1 | **Versiones de dependencias:** Coherencia EF Core vs MediatR; evitar conflictos transitivos. | GesFer.Admin.Back.Infrastructure.csproj | 2026-02-21 |
| M2 | **Estructura de tests / código legacy:** Carpeta src/tests con GesFer.Product.IntegrationTests (no pertenece a Admin). Ubicación estándar de UnitTests. | src/tests/ | 2026-02-20 |
| M3 | **Nomenclatura de carpetas:** application y domain en minúsculas; Api e Infrastructure en PascalCase. Normalizar a PascalCase. | src/ | 2026-02-20, 2026-02-19 |
| M4 | **Unificar-Rama.ps1:** Aceptar documentación en docs/features/<slug>/objectives.md además de docs/branches/.../OBJETIVO.md. | scripts/skills/Unificar-Rama.ps1 | AUDITORIA_CARPETA_SCRIPTS |
| M5 | **Revisar commit-skill.sh:** Confirmar uso en hooks/CI; si no, archivar o eliminar. | scripts/skills/ | AUDITORIA_CARPETA_SCRIPTS |

### 3.3 Bajos

| Id | Hallazgo | Nota |
| :--- | :--- | :--- |
| B1 | **Propuesta/ en scripts:** Decidir si mover a docs/proposals/ o mantener como referencia. | scripts/Propuesta/ |

---

## 4. Objetivos medibles (Key Results)

- **KR1:** El proyecto compila con `dotnet build` sin errores (C1 resuelto).
- **KR2:** No existen credenciales en texto plano en Program.cs (C2 resuelto).
- **KR3:** LogController no depende de AdminDbContext; uso de Mediator y/o IApplicationDbContext (C3).
- **KR4:** Application no referencia a Infrastructure; interfaces en Application, implementaciones en Infrastructure (C4, C5).
- **KR5:** Estructura de tests limpia: sin GesFer.Product en Admin.Back; UnitTests en ubicación estándar (M2).
- **KR6:** (Opcional) Nomenclatura de carpetas consistente PascalCase (M3).

---

## 5. Alcance de esta ronda

- **Incluir:** Corrección de C1, C2 y, en la medida de lo posible, C3–C5 dentro del mismo ciclo o desglosados en sub-tareas/ramas.
- **Opcional en esta ronda:** M1–M5, B1; pueden documentarse como siguientes iteraciones.
- **No incluir:** Refactor no indicado por auditoría; ampliación de funcionalidad.

---

## 6. Referencias

- paths.auditsPath (Cúmulo): docs/audits/
- Proceso: SddIA/process/correccion-auditorias/
- Leyes: AGENTS.md (Ley COMPILACIÓN, Ley GIT, Soberanía documental)
