#!/bin/bash
cat << 'AUDIT_CONTENT' > docs/audits/AUDITORIA_$(date -u +"%Y_%m_%d").md
# S+ Audit Report

1. Métricas de Salud (0-100%)
Arquitectura: 90% | Nomenclatura: 95% | Estabilidad Async: 85%

2. Pain Points (🔴 Críticos / 🟡 Medios)
Hallazgo: [🔴 Crítico] Falta propagación de CancellationToken en llamadas a base de datos en `AdminJsonDataSeeder.cs`, `AuditLogService.cs` y `AdminAuthService.cs`.
Ubicación: src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs (múltiples líneas), src/GesFer.Admin.Back.Infrastructure/Services/AuditLogService.cs:50, src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs:37

Hallazgo: [🔴 Crítico] Uso de operaciones asíncronas Entity Framework sin propagar `CancellationToken` (por ejemplo, `SaveChangesAsync()`, `FirstOrDefaultAsync()`).
Ubicación: src/GesFer.Admin.Back.Infrastructure/ y src/GesFer.Admin.Back.Application/

Hallazgo: [🔴 Crítico] Llamadas a la base de datos para entidades de solo lectura sin utilizar `AsNoTracking()` (por ejemplo, `ToListAsync()`).
Ubicación: src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs

3. Acciones Kaizen (Hoja de Ruta para el Executor)
**Tarea 1: Propagar CancellationToken en la capa de Infraestructura y Aplicación**
- Identificar todas las llamadas asíncronas de base de datos (e.g. `ToListAsync`, `FirstOrDefaultAsync`, `SaveChangesAsync`) en `AdminJsonDataSeeder.cs`, `AuditLogService.cs`, `AdminAuthService.cs` y manejadores en `GesFer.Admin.Back.Application`.
- Añadir el parámetro opcional `CancellationToken` en las firmas de los métodos.
- Propagar el `CancellationToken` a todas las llamadas asíncronas para evitar bloqueos del thread pool.
- **DoD:** Todas las llamadas a métodos `Async` de EF Core reciben explícitamente un `CancellationToken`. El código compila sin errores.

**Tarea 2: Uso de AsNoTracking para consultas de solo lectura**
- Modificar las llamadas de solo lectura en `AdminJsonDataSeeder.cs` u otras partes de la capa de infraestructura/aplicación para agregar `.AsNoTracking()`.
- **DoD:** Consultas a entidades como Languages, Countries, States, Cities, PostalCodes, Companies, Users, y AdminUsers en métodos Seed incluyen `.AsNoTracking()` donde sea seguro y aplicable para prevenir fugas de memoria. El código compila y pasa todas las pruebas.
AUDIT_CONTENT
