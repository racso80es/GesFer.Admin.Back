---
base: ["main"]
scope:
  in_scope:
    - Agregar CancellationToken a operaciones asíncronas en Infrastructure
    - Agregar AsNoTracking() a validaciones AnyAsync() de lectura en Application
  out_scope:
    - Reestructuración o rediseño mayor de la arquitectura
---
# Corrección según Auditoría 2026-06-03

## Contexto
El reporte de auditoría `docs/audits/AUDITORIA_2026_06_03.md` identificó problemas críticos con la estabilidad asíncrona (falta de CancellationToken en llamadas a Base de Datos) y la eficiencia de memoria (falta de `AsNoTracking()` en queries que no necesitan ser modificadas).

## Plan de Acción

### 1. Estabilidad Async
- Propagar `CancellationToken` en todas las llamadas EF asíncronas (`SaveChangesAsync`, `FirstOrDefaultAsync`, `ToListAsync`) especialmente dentro de `AdminJsonDataSeeder` y `AuditLogService`.
- Validar `IAdminAuthService.cs` para el paso de CancellationToken.

### 2. Memoria/Rendimiento (AsNoTracking)
- Agregar `.AsNoTracking()` en llamadas de validación `AnyAsync()` y otras llamadas `FirstOrDefaultAsync` que actúen solo como comprobación (ej: `UpdateUserHandler`).
