# SPEC: Corrección según auditoría 2026-02-23

**ID de especificación:** SPEC-CORR-AUD-20260223  
**Rama:** feat/correccion-auditorias-20260223  
**Estado:** Draft  
**Proceso:** correccion-auditorias  
**Origen:** paths.auditsPath — AUDITORIA_2026_02_23.md  
**Contexto (Cúmulo):** paths.featurePath/correccion-auditorias-20260223/

---

## 1. Propósito y contexto

### 1.1 Objetivo
Corregir los hallazgos críticos y medios de la auditoría del 2026-02-23: modularizar la inyección de dependencias (Clean Architecture), migrar la lógica de LogController a CQRS (MediatR) y corregir el test de integración PurgeLogs. Alcance acotado a lo reportado en la auditoría.

### 1.2 Alcance (scope)

**Incluido:**
- **C1 — Modularidad DI:** Crear `Application/DependencyInjection.cs` (AddApplicationServices: MediatR, validators, behaviors) y `Infrastructure/DependencyInjection.cs` (AddInfrastructureServices: DB, Auth, Serilog Sinks). Limpiar Api/DependencyInjection.cs: solo invocar AddApplicationServices() y AddInfrastructureServices(config). Api sin referencias a Infrastructure.
- **C2 — CQRS en Logs:** Crear en Application: CreateLogCommand, CreateAuditLogCommand, GetLogsQuery, PurgeLogsCommand y sus Handlers. LogController inyecta ISender (MediatR); métodos como dispatchers (Send(command)/Send(query)). Migrar lógica de ExecuteDeleteAsync a PurgeLogsCommandHandler y filtros a GetLogsQueryHandler.
- **C3 — Test de integración:** Corregir PurgeLogs_ShouldDeleteOldLogs (500 → 200). Investigar ExecuteDeleteAsync en entorno test (InMemory vs relacional / Testcontainers).

**Fuera de alcance:**
- Cambios no indicados en AUDITORIA_2026_02_23.md.

---

## 2. Arquitectura y diseño técnico

### 2.1 Paso 1: Modularizar inyección de dependencias
- **Application/DependencyInjection.cs:** `AddApplicationServices(this IServiceCollection services)` — registrar MediatR desde Assembly ejecutante, validators, behaviors. Sin referencias a Infrastructure.
- **Infrastructure/DependencyInjection.cs:** `AddInfrastructureServices(this IServiceCollection services, IConfiguration configuration)` — registrar AdminDbContext, AdminAuthService, MySqlSequentialGuidGenerator, Serilog Sinks y demás servicios de infraestructura. Implementaciones de interfaces definidas en Application.
- **Api/DependencyInjection.cs (limpieza):** Eliminar registros manuales de Infrastructure. Reemplazar por `builder.Services.AddApplicationServices()` y `builder.Services.AddInfrastructureServices(config)`.
- **Api .csproj:** Eliminar referencia a proyecto Infrastructure si existe; Api no debe depender de Infrastructure.

### 2.2 Paso 2: CQRS en Logs
- **Application:** Commands: CreateLogCommand, CreateAuditLogCommand, PurgeLogsCommand. Query: GetLogsQuery. Handlers que inyecten IApplicationDbContext (o interfaces de Application) y ejecuten la lógica actual del LogController.
- **LogController:** Inyectar solo IMediator o ISender. Endpoints: dispatch a Send(command) o Send(query); sin acceso directo a DbContext ni lógica de negocio.

### 2.3 Paso 3: Test de integración
- **PurgeLogs_ShouldDeleteOldLogs:** Identificar causa del 500 (excepción en ExecuteDeleteAsync o configuración DbContext en tests). Opciones: base efímera relacional, Testcontainers, o mock adecuado. Objetivo: test en verde con 200 OK.

### 2.4 Componentes afectados
- `GesFer.Admin.Back.Application`: DependencyInjection.cs, Commands/Queries/Handlers de Logs.
- `GesFer.Admin.Back.Infrastructure`: DependencyInjection.cs, implementaciones ya existentes.
- `GesFer.Admin.Back.Api`: DependencyInjection.cs, Controllers/LogController.cs, .csproj.
- `GesFer.Admin.Back.IntegrationTests`: LogControllerTests (PurgeLogs).

---

## 3. Requisitos de seguridad
- No introducir nuevas dependencias directas Api → Infrastructure.
- Mantener [Authorize] y validación de inputs en endpoints de logs.

---

## 4. Criterios de aceptación (DoD)
- [ ] GesFer.Admin.Back.Api no tiene `using GesFer.Admin.Back.Infrastructure.*`.
- [ ] Servicios registrados mediante AddApplicationServices y AddInfrastructureServices.
- [ ] LogController no inyecta IApplicationDbContext; usa ISender (MediatR).
- [ ] dotnet build exitoso.
- [ ] dotnet test pasa al 100% (unitarios + integración PurgeLogs).
- [ ] validacion.json generado en paths.featurePath/correccion-auditorias-20260223/.

---

## 5. Referencias
- paths.auditsPath — AUDITORIA_2026_02_23.md
- paths.processPath/correccion-auditorias/
- objectives.md (esta carpeta)
