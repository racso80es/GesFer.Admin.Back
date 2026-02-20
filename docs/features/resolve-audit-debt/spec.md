# SPEC-RESOLVE-AUDIT-DEBT

**Título:** Resolución de Deuda Técnica y Hallazgos de Auditoría (2026-02-20)
**Estado:** Draft
**Fecha:** 2026-02-21

## 1. Contexto
El proyecto `GesFer.Admin.Back` presenta errores de compilación críticos y violaciones de arquitectura limpia detectados en auditorías recientes.
El objetivo es corregir estos problemas para estabilizar el build y asegurar la mantenibilidad del código, siguiendo los principios de Clean Architecture.

## 2. Alcance (Scope)
*   **In-Scope:**
    *   Creación de DTOs en `GesFer.Admin.Back.Application`.
    *   Refactorización de `LogController` para eliminar dependencia directa de `AdminDbContext`.
    *   Inversión de dependencias entre `Application` e `Infrastructure`.
    *   Movimiento de interfaces de `Infrastructure` a `Application`.
    *   Eliminación de código legacy (`src/tests`).
*   **Out-of-Scope:**
    *   Nuevas funcionalidades de negocio.
    *   Refactorización completa de todos los controladores (solo `LogController` es prioritario por el error).

## 3. Arquitectura y Diseño Técnico

### 3.1. Data Transfer Objects (DTOs)
Se crearán las siguientes clases en `GesFer.Admin.Back.Application.DTOs.Logs`:
*   `CreateLogDto`: Para recepción de logs.
*   `CreateAuditLogDto`: Para recepción de logs de auditoría.
*   `LogDto`: Para respuesta de logs.
*   `LogsPagedResponseDto`: Para respuestas paginadas.
*   `PurgeLogsResponseDto`: Para respuesta de purgado.

### 3.2. Inversión de Dependencias
*   **Interfaces:** Mover `IAdminAuthService`, `IAdminJwtService`, `IAuditLogService` de `Infrastructure/Services` a `Application/Common/Interfaces`.
*   **Referencias:**
    *   Eliminar referencia `Application -> Infrastructure`.
    *   Asegurar `Infrastructure -> Application` (ya debería existir o añadirse).
*   **Persistencia:**
    *   Definir `IApplicationDbContext` en `Application/Common/Interfaces` exponiendo `DbSet<Log>`, `DbSet<AuditLog>`, `SaveChangesAsync`.
    *   Implementar `IApplicationDbContext` en `AdminDbContext` (Infrastructure).

### 3.3. Desacoplamiento de LogController
*   Refactorizar `LogController` para inyectar `IApplicationDbContext` (o `IMediator` si se decide usar CQRS completo, pero `IApplicationDbContext` es el primer paso para romper la dependencia circular si `LogController` está en `Api` y `Api` depende de `Application`).
*   *Nota:* Si `LogController` está en `Api`, y `Api` depende de `Infrastructure` para inyectar el DbContext, es una violación. `Api` debe depender de `Application` e `Infrastructure` (solo para inyección de dependencias en Program.cs), pero los controladores deben usar interfaces de `Application`.

### 3.4. Limpieza
*   Eliminar `src/tests/` y su contenido.

## 4. Seguridad
*   Mantener atributos `[Authorize]`.
*   Validar inputs en DTOs.

## 5. Criterios de Aceptación
1.  La solución `GesFer.Admin.Back.sln` compila sin errores (`dotnet build` exitoso).
2.  El proyecto `GesFer.Admin.Back.Application` NO tiene referencia a `GesFer.Admin.Back.Infrastructure`.
3.  `LogController` usa DTOs definidos en `Application`.
4.  No existen carpetas `src/tests`.
