# IMPL: Corrección según Auditorías (2026-02)

**Plan:** plan.md | **SPEC:** spec.md | **Validación:** principles-validation.md  
**Ruta (Cúmulo):** docs/features/correccion-segun-auditorias/  
**Rama:** feat/correccion-segun-auditorias

---

## Ajustes incorporados desde la validación #principios

- **invoke-command (execution_contract):** Todo comando de sistema (git, dotnet build, etc.) debe ejecutarse vía skill **invoke-command** (paths.skillCapsules.invoke-command). No ejecutar comandos directamente en shell.
- **Cláusulas de guarda (clausulas-de-guarda-early-return):** Reforzar validación de inputs al inicio de los endpoints en LogController; early return en caso de precondiciones no cumplidas.
- **Commits atómicos:** Un commit por fase lógica: commit 1 tras Fase 1 (C1), commit 2 tras Fase 2 (C2), commit 3 opcional tras Fase 4 (M2) si hubo cambios.

---

## Ítems de implementación

### Fase 1 — C1: DTOs de Logs

#### 1.1 – Crear: carpeta DTOs/Logs

- **Id:** 1.1
- **Acción:** Crear
- **Ruta:** `src/GesFer.Admin.Back.Application/DTOs/Logs/`
- **Propuesta:** Crear la carpeta si no existe. Namespace base para todos los DTOs: `GesFer.Admin.Back.Application.DTOs.Logs`. Fuente de verdad: docs/audits/AUDITORIA_2026_02_21.md (Acción 1).

#### 1.2 – Crear: CreateLogDto.cs

- **Id:** 1.2
- **Acción:** Crear
- **Ruta:** `src/GesFer.Admin.Back.Application/DTOs/Logs/CreateLogDto.cs`
- **Propuesta:** Clase con propiedades: Level (string), Message (string), Exception (string?), TimeStamp (DateTime), Properties (Dictionary<string, object>?). Inicializaciones por defecto según auditoría. Namespace `GesFer.Admin.Back.Application.DTOs.Logs`.

#### 1.3 – Crear: CreateAuditLogDto.cs

- **Id:** 1.3
- **Acción:** Crear
- **Ruta:** `src/GesFer.Admin.Back.Application/DTOs/Logs/CreateAuditLogDto.cs`
- **Propuesta:** Clase con CursorId, Username, Action, HttpMethod, Path, AdditionalData (todos string?), ActionTimestamp (DateTime). Namespace `GesFer.Admin.Back.Application.DTOs.Logs`.

#### 1.4 – Crear: LogDto.cs

- **Id:** 1.4
- **Acción:** Crear
- **Ruta:** `src/GesFer.Admin.Back.Application/DTOs/Logs/LogDto.cs`
- **Propuesta:** Clase con Id (int), Level, Message, Exception, TimeStamp, Source (string?), CompanyId (Guid?), UserId (Guid?). Namespace `GesFer.Admin.Back.Application.DTOs.Logs`.

#### 1.5 – Crear: LogsPagedResponseDto.cs

- **Id:** 1.5
- **Acción:** Crear
- **Ruta:** `src/GesFer.Admin.Back.Application/DTOs/Logs/LogsPagedResponseDto.cs`
- **Propuesta:** Clase con Logs (List<LogDto>), TotalCount, PageNumber, PageSize, TotalPages (int). Inicializar Logs = new(). Namespace `GesFer.Admin.Back.Application.DTOs.Logs`.

#### 1.6 – Crear: PurgeLogsResponseDto.cs

- **Id:** 1.6
- **Acción:** Crear
- **Ruta:** `src/GesFer.Admin.Back.Application/DTOs/Logs/PurgeLogsResponseDto.cs`
- **Propuesta:** Clase con DeletedCount (int), DateLimit (DateTime). Namespace `GesFer.Admin.Back.Application.DTOs.Logs`.

#### 1.7 – Verificación: build (vía invoke-command)

- **Id:** 1.7
- **Acción:** Verificar
- **Propuesta:** Ejecutar build **mediante skill invoke-command** (paths.skillCapsules.invoke-command), por ejemplo: `Invoke-Command.ps1 -Command "dotnet build src/GesFer.Admin.Back.sln"` (o la ruta relativa desde repo según la cápsula). Confirmar que no hay errores de tipos faltantes en LogController. **Commit atómico:** feat(deps): add Log DTOs in Application (C1).

---

### Fase 2 — C2: Credenciales en Program.cs

#### 2.1 – Modificar: Program.cs (connection string)

- **Id:** 2.1
- **Acción:** Modificar
- **Ruta:** `src/GesFer.Admin.Back.Api/Program.cs`
- **Ubicación:** Líneas ~29-30 (asignación de connectionString con fallback).
- **Propuesta:** Sustituir el fallback que contiene credenciales por:  
  `var connectionString = builder.Configuration.GetConnectionString("DefaultConnection") ?? throw new InvalidOperationException("Connection string 'DefaultConnection' not found.");`  
  Eliminar cualquier cadena literal con Password= o User= en ese archivo.

#### 2.2 – Verificación: ausencia de credenciales

- **Id:** 2.2
- **Acción:** Verificar
- **Propuesta:** Comprobar que en Program.cs no quede ninguna cadena con "Password=" ni credenciales en texto plano. **Commit atómico:** fix(security): remove hardcoded connection string fallback (C2).

---

### Fase 2b — Ajuste validación: cláusulas de guarda en LogController

#### 2.3 – Modificar: LogController (early return en ReceiveAuditLog)

- **Id:** 2.3
- **Acción:** Modificar
- **Ruta:** `src/GesFer.Admin.Back.Api/Controllers/LogController.cs`
- **Ubicación:** Inicio del método `ReceiveAuditLog([FromBody] CreateAuditLogDto dto)`.
- **Propuesta:** Añadir cláusula de guarda al inicio (principio clausulas-de-guarda-early-return): si `dto == null`, `return BadRequest(...)` de forma inmediata antes del try. Mantener las guardas existentes en ReceiveLog (dto null, Level, Message) y en GetLogs/PurgeLogs (validación de parámetros al inicio). Objetivo: validación de precondiciones al principio del método; camino feliz sin anidación innecesaria.

---

### Fase 3 — Verificación C3, C4, C5

#### 3.1 – Verificar: LogController (IApplicationDbContext)

- **Id:** 3.1
- **Acción:** Verificar
- **Propuesta:** Confirmar que LogController inyecta solo IApplicationDbContext (y ILogger). No debe referenciar AdminDbContext ni tipos de Infrastructure. Si se detectara regresión, corregir inyección y usings.

#### 3.2 – Verificar: Application.csproj

- **Id:** 3.2
- **Acción:** Verificar
- **Propuesta:** Abrir `src/GesFer.Admin.Back.Application/GesFer.Admin.Back.Application.csproj` y confirmar que no existe ProjectReference a GesFer.Admin.Back.Infrastructure. Solo Domain (y paquetes). Si existiera referencia, eliminarla (acción 3.4).

#### 3.3 – Verificar: interfaces en Application

- **Id:** 3.3
- **Acción:** Verificar
- **Propuesta:** Confirmar que IApplicationDbContext, IAdminAuthService, IAdminJwtService, IAuditLogService están en Application/Common/Interfaces; implementaciones en Infrastructure. Sin cambio si ya se cumple.

#### 3.4 – Acción condicional: regresión C4/C5

- **Id:** 3.4
- **Acción:** Modificar (solo si 3.2 o 3.3 fallan)
- **Propuesta:** Si Application tuviera referencia a Infrastructure: eliminar la referencia del .csproj. Si alguna interfaz estuviera definida en Infrastructure: moverla a Application/Common/Interfaces y actualizar namespaces e implementaciones.

---

### Fase 4 — M2: Limpieza src/tests

#### 4.1 – Comprobar: existencia de src/tests

- **Id:** 4.1
- **Acción:** Comprobar
- **Propuesta:** Verificar si existe la carpeta `src/tests/` y qué proyectos contiene. Si existe GesFer.Admin.UnitTests (sin "Back") o proyectos que no sean Admin.Back, documentar para 4.2/4.3.

#### 4.2 – Modificar: solución .sln (si aplica)

- **Id:** 4.2
- **Acción:** Modificar (condicional)
- **Ruta:** `src/GesFer.Admin.Back.sln`
- **Propuesta:** Si UnitTests estaba bajo src/tests/, actualizar la ruta del proyecto en la solución para apuntar a la ubicación estándar (p. ej. src/GesFer.Admin.Back.UnitTests) y mover físicamente el proyecto si procede. No eliminar GesFer.Admin.Back.UnitTests ni IntegrationTests.

#### 4.3 – Eliminar: código legacy en src/tests (si aplica)

- **Id:** 4.3
- **Acción:** Eliminar (condicional)
- **Propuesta:** Si src/tests/ contiene solo proyectos que no pertenecen a Admin.Back o duplicados, eliminar la carpeta y actualizar la solución. No borrar proyectos Admin.Back. **Commit atómico (si hubo cambios):** chore(tests): clean src/tests legacy (M2).

#### 4.4 – Documentar: M2 N/A

- **Id:** 4.4
- **Acción:** Documentar
- **Propuesta:** Si no existe src/tests/ o no hay cambios que hacer, registrar en execution.json o validacion.json que M2 se considera N/A o verificado.

---

## Verificación final (execution)

- **Build:** Ejecutar `dotnet build` sobre la solución **solo mediante invoke-command** (paths.skillCapsules.invoke-command). Criterio: build exitoso.
- **Seguridad:** Ninguna cadena con "Password=" o "User=" en texto plano en Program.cs.
- **Arquitectura:** Application sin referencia a Infrastructure; LogController sin referencia a AdminDbContext.

---

## Orden de ejecución y commits

| Orden | Ítems        | Commit sugerido (atómico) |
|-------|--------------|---------------------------|
| 1     | 1.1–1.7      | feat(deps): add Log DTOs in Application (C1) |
| 2     | 2.1–2.3      | fix(security): remove hardcoded connection string; guard clause ReceiveAuditLog (C2 + principios) |
| 3     | 3.1–3.4      | Sin commit si solo verificación; si hubo corrección, commit específico. |
| 4     | 4.1–4.4      | chore(tests): clean src/tests legacy (M2) — solo si hubo cambios. |

---

*Implementación alineada con plan.md, principles-validation.md y execution_contract (invoke-command). paths.skillCapsules.invoke-command (Cúmulo).*
