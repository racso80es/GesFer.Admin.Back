# SPEC: Corrección según Auditorías (2026-02)

**ID de Especificación:** SPEC-CORR-AUD-2026-01  
**Rama:** feat/correccion-segun-auditorias  
**Estado:** Draft  
**Proceso:** correccion-auditorias  
**Contexto (Cúmulo):** paths.featurePath/correccion-segun-auditorias/

---

## 1. Propósito y Contexto

### 1.1 Objetivo
Corregir los hallazgos críticos y medios reportados en las últimas auditorías (paths.auditsPath), priorizando compilación, seguridad y respeto a la arquitectura de capas (Clean Architecture / Dependency Inversion). La especificación formaliza el alcance técnico y los criterios de aceptación para esta ronda de corrección.

### 1.2 Alcance (Scope)

**Incluido:**
- **C1:** Crear DTOs de Logs en `GesFer.Admin.Back.Application` (CreateLogDto, CreateAuditLogDto, LogDto, LogsPagedResponseDto, PurgeLogsResponseDto).
- **C2:** Eliminar credenciales hardcoded en `Program.cs`; usar excepción si no hay ConnectionString.
- **C3:** Desacoplar LogController de AdminDbContext (Mediator y/o IApplicationDbContext).
- **C4:** Inversión de dependencias: Application sin referencia a Infrastructure; IApplicationDbContext en Application, implementación en Infrastructure.
- **C5:** Mover interfaces (IAuthService, IJwtService, etc.) de Infrastructure a Application (Common/Interfaces).
- **M2 (opcional esta ronda):** Limpieza de src/tests y código legacy GesFer.Product.

**Fuera de alcance:**
- Refactor no indicado por auditoría; ampliación de funcionalidad.
- M1, M3, M4, M5, B1 se documentan para iteraciones posteriores.

---

## 2. Arquitectura y Diseño Técnico

### 2.1 DTOs de Logs (C1)
- **Ubicación:** `src/GesFer.Admin.Back.Application/DTOs/Logs/`.
- **Clases:** CreateLogDto, CreateAuditLogDto, LogDto, LogsPagedResponseDto, PurgeLogsResponseDto (namespaces y propiedades según uso en LogController y auditoría 2026-02-21).

### 2.2 Seguridad (C2)
- **Program.cs:** Sustituir fallback de connection string por `throw new InvalidOperationException("Connection string 'DefaultConnection' not found.")`.

### 2.3 Inversión de dependencias (C4, C5)
- **Application:** Definir `IApplicationDbContext` (DbSet&lt;Log&gt;, DbSet&lt;AuditLog&gt;, SaveChangesAsync) y mover interfaces de servicios (IAuthService, IJwtService, IAuditLogService, etc.) a `Application/Common/Interfaces`.
- **Infrastructure:** AdminDbContext implementa IApplicationDbContext; servicios implementan interfaces de Application. Eliminar referencia Application → Infrastructure.

### 2.4 Desacoplamiento LogController (C3)
- LogController deja de inyectar AdminDbContext. Inyectar IMediator y/o IApplicationDbContext (según diseño); Commands/Queries en Application con Handlers que usen IApplicationDbContext.

### 2.5 Componentes afectados
- `GesFer.Admin.Back.Application`: DTOs/Logs, Common/Interfaces, Commands/Queries/Handlers para Logs.
- `GesFer.Admin.Back.Infrastructure`: AdminDbContext, implementaciones de interfaces.
- `GesFer.Admin.Back.Api`: Program.cs, Controllers/LogController.cs.

---

## 3. Requisitos de Seguridad
- No exponer credenciales en código (C2).
- Mantener [Authorize] y validación de inputs en endpoints de logs.

---

## 4. Criterios de Aceptación
- [ ] `dotnet build` exitoso (KR1).
- [ ] No existen contraseñas en texto plano en Program.cs (KR2).
- [ ] LogController no depende de AdminDbContext (KR3).
- [ ] Application no referencia a Infrastructure (KR4).
- [ ] Interfaces de servicios en Application; implementaciones en Infrastructure (KR4/KR5).
- [ ] Validación registrada (validacion.json en carpeta de tarea o paths.auditsPath).

---

## 5. Trazabilidad
- **Fuentes:** docs/audits/AUDITORIA_2026_02_21.md, AUDITORIA_2026_02_20.md, AUDITORIA_2026_02_19.md.
- **Objetivos:** docs/features/correccion-segun-auditorias/objectives.md.
- **Hallazgos consolidados:** docs/features/correccion-segun-auditorias/audit-hallazgos-consolidado.json.
