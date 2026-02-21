# Clarificación: Corrección según Auditorías (2026-02)

**Proceso:** correccion-auditorias  
**Ruta (Cúmulo):** docs/features/correccion-segun-auditorias/  
**Fecha:** 2026-02-21  
**Especificación:** spec.md  

---

## 1. Gaps y ambigüedades resueltas

### 1.1 Namespace y definición exacta de DTOs de Logs (C1)

**Gap:** ¿Namespace exacto y propiedades de los DTOs? El spec indica "según uso en LogController y auditoría 2026-02-21".

**Resolución:** El namespace canónico es `GesFer.Admin.Back.Application.DTOs.Logs`. La definición exacta de las cinco clases (CreateLogDto, CreateAuditLogDto, LogDto, LogsPagedResponseDto, PurgeLogsResponseDto) se toma como **fuente de verdad** de `docs/audits/AUDITORIA_2026_02_21.md` (Acción 1, bloques de código). No hay discrepancia con el uso actual en LogController: las propiedades coinciden. Crear la carpeta `src/GesFer.Admin.Back.Application/DTOs/Logs/` y los cinco archivos según la auditoría.

### 1.2 LogController: ¿Mediator o IApplicationDbContext? (C3)

**Gap:** El spec indica "Mediator y/o IApplicationDbContext". El código actual de LogController ya inyecta `IApplicationDbContext` y no usa `AdminDbContext` directamente.

**Resolución:** El estado actual **cumple el objetivo de desacoplamiento** (no hay dependencia directa de AdminDbContext). Para esta ronda de corrección **no es obligatorio** introducir Commands/Queries/Handlers con MediatR para Logs. Mantener LogController usando IApplicationDbContext es aceptable. Si en el futuro se desea homogeneizar con el patrón CQRS de Company/Geo, será una tarea aparte. **C3 considerado satisfecho** tras verificación en implementación; no bloquear planning por migración a Mediator.

### 1.3 C4 y C5: estado actual del código

**Gap:** ¿Application referencia hoy a Infrastructure? ¿Dónde están las interfaces?

**Resolución:** Tras revisión del código: (1) `GesFer.Admin.Back.Application.csproj` **no** referencia a Infrastructure (solo Domain). (2) Las interfaces `IApplicationDbContext`, `IAdminAuthService`, `IAdminJwtService`, `IAuditLogService` están en `Application/Common/Interfaces`; las implementaciones están en Infrastructure. Por tanto **C4 y C5 están ya resueltos** en el estado actual. En la fase de implementation/execution solo se **verificará** que no exista referencia Application→Infrastructure y que no queden interfaces de dominio en Infrastructure; no se exige cambio de código para C4/C5 salvo si una comprobación detectara regresión.

### 1.4 Credenciales en Program.cs (C2)

**Gap:** Ninguno. Comportamiento requerido está definido.

**Resolución:** Sustituir el fallback de connection string (líneas 29-30) por `throw new InvalidOperationException("Connection string 'DefaultConnection' not found.");`. Sin variantes.

### 1.5 M2 (limpieza src/tests) en esta ronda

**Gap:** El spec marca M2 como "opcional esta ronda". ¿Incluir o diferir?

**Resolución:** **Incluir en esta ronda** si el tiempo lo permite, con prioridad menor que C1 y C2. Acciones: (1) Mover `GesFer.Admin.Back.UnitTests` a ubicación estándar si está bajo `src/tests/`; (2) Eliminar carpeta `src/tests/` y cualquier proyecto que no pertenezca a Admin.Back (p. ej. GesFer.Product.IntegrationTests); (3) Actualizar la solución (.sln) para reflejar las rutas correctas. Si la estructura actual ya no tiene `src/tests/`, documentar en plan y marcar M2 como verificado o N/A.

---

## 2. Orden de implementación recomendado

1. **C1** — Crear DTOs de Logs (desbloquea compilación).  
2. **C2** — Eliminar credenciales hardcoded en Program.cs.  
3. **Verificación C3/C4/C5** — Confirmar que LogController usa IApplicationDbContext y que Application no referencia Infrastructure; sin cambios si ya se cumple.  
4. **M2** — Limpieza de src/tests y solución (si aplica).

---

## 3. Sin gaps pendientes

No quedan ambigüedades que bloqueen la fase de planning o implementation.

---

## 4. Referencias a skills/herramientas

| Fase        | Skill / estándar        |
|------------|--------------------------|
| Implementación código | dotnet-development |
| Validación (build, tests) | invoke-command, dotnet-development |
| Documentación         | documentation           |

---

*Clarificación generada en el marco de la acción clarify. Cumple interfaz de proceso (clarify.md + clarify.json).*
