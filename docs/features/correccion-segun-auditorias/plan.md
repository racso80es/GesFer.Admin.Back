# PLAN: Corrección según Auditorías (2026-02)

**Proceso:** correccion-auditorias  
**Ruta (Cúmulo):** docs/features/correccion-segun-auditorias/  
**Especificación:** spec.md | **Clarificación:** clarify.md  
**Fecha:** 2026-02-21  

---

## 1. Objetivos del plan

- Desbloquear la compilación creando los DTOs de Logs faltantes (C1).
- Eliminar credenciales en texto plano en Program.cs (C2).
- Verificar que C3, C4 y C5 se cumplen (LogController con IApplicationDbContext; Application sin ref a Infrastructure; interfaces en Application).
- Limpiar estructura de tests si existe src/tests o proyectos legacy (M2).

---

## 2. Fases y tareas técnicas

### Fase 1 — C1: DTOs de Logs (bloqueante)

| Id  | Tarea | Criterio |
|-----|-------|----------|
| 1.1 | Crear carpeta `src/GesFer.Admin.Back.Application/DTOs/Logs/` | Si no existe. |
| 1.2 | Crear `CreateLogDto.cs` | Namespace `GesFer.Admin.Back.Application.DTOs.Logs`. Propiedades: Level, Message, Exception, TimeStamp, Properties (Dictionary&lt;string, object&gt;?). Definición exacta en docs/audits/AUDITORIA_2026_02_21.md. |
| 1.3 | Crear `CreateAuditLogDto.cs` | CursorId, Username, Action, HttpMethod, Path, AdditionalData, ActionTimestamp. |
| 1.4 | Crear `LogDto.cs` | Id, Level, Message, Exception, TimeStamp, Source, CompanyId, UserId. |
| 1.5 | Crear `LogsPagedResponseDto.cs` | Logs (List&lt;LogDto&gt;), TotalCount, PageNumber, PageSize, TotalPages. |
| 1.6 | Crear `PurgeLogsResponseDto.cs` | DeletedCount, DateLimit. |
| 1.7 | Verificación | `dotnet build` desde raíz de la solución (src/) debe completar sin errores de tipos faltantes en LogController. |

### Fase 2 — C2: Credenciales en Program.cs

| Id  | Tarea | Criterio |
|-----|-------|----------|
| 2.1 | Modificar `src/GesFer.Admin.Back.Api/Program.cs` | Sustituir el fallback de connection string (líneas ~29-30) por: `var connectionString = builder.Configuration.GetConnectionString("DefaultConnection") ?? throw new InvalidOperationException("Connection string 'DefaultConnection' not found.");` |
| 2.2 | Verificación | No debe existir ninguna cadena con "Password=" ni credenciales en texto plano en Program.cs. |

### Fase 3 — Verificación C3, C4, C5

| Id  | Tarea | Criterio |
|-----|-------|----------|
| 3.1 | Verificar LogController | Confirmar que inyecta solo `IApplicationDbContext` (y ILogger). No debe referenciar AdminDbContext ni Infrastructure. |
| 3.2 | Verificar Application.csproj | Confirmar que no tiene ProjectReference a GesFer.Admin.Back.Infrastructure. |
| 3.3 | Verificar interfaces | IApplicationDbContext, IAdminAuthService, IAdminJwtService, IAuditLogService en Application/Common/Interfaces; implementaciones en Infrastructure. |
| 3.4 | Acción si hay regresión | Si se detectara referencia Application→Infrastructure o interfaces en Infrastructure, eliminar referencia y mover interfaces a Application. Según clarify, el estado actual ya cumple; esta fase es solo verificación. |

### Fase 4 — M2: Limpieza de src/tests (opcional esta ronda)

| Id  | Tarea | Criterio |
|-----|-------|----------|
| 4.1 | Comprobar existencia de `src/tests/` | Si existe carpeta con proyectos que no sean Admin.Back o con nombre legacy (p. ej. GesFer.Admin.UnitTests sin "Back"), documentar. |
| 4.2 | Actualizar solución | Si UnitTests está bajo src/tests/, actualizar .sln para apuntar a la ruta estándar (p. ej. src/GesFer.Admin.Back.UnitTests) y mover el proyecto si procede. |
| 4.3 | Eliminar código legacy | Eliminar carpeta src/tests/ si solo contiene proyectos que no pertenecen a Admin.Back o duplicados. No eliminar GesFer.Admin.Back.UnitTests ni GesFer.Admin.Back.IntegrationTests. |
| 4.4 | Si no existe src/tests/ | Marcar M2 como N/A en validación. |

---

## 3. Verificación final

- **Build:** `dotnet build src/GesFer.Admin.Back.sln` (o desde src: `dotnet build GesFer.Admin.Back.sln`) exitoso.
- **Seguridad:** Búsqueda de "Password=" o "User=" en cadenas literales en Program.cs debe dar 0 resultados.
- **Arquitectura:** Application sin referencia a Infrastructure; LogController sin referencia a AdminDbContext.

---

## 4. Seguridad y trazabilidad

- No introducir nuevas credenciales ni secretos en código.
- Mantener [Authorize] y validación de inputs en LogController.
- Rutas y artefactos según Cúmulo (paths.featurePath, paths.auditsPath).

---

## 5. Orden de ejecución (resumen)

1. Fase 1 (C1) — DTOs Logs.  
2. Fase 2 (C2) — Program.cs.  
3. Fase 3 — Verificación C3/C4/C5 (sin cambios si ya se cumple).  
4. Fase 4 — M2 si aplica (src/tests).

---

*Plan generado a partir de spec.md y clarify.md. Acción planning.*
