---
fix_id: audit-log-login-empty
title: "Fix: AuditLogs vacía tras login Admin desde frontend"
process_ref: SddIA/process/bug-fix
paths_ref: Cúmulo paths.fixPath
spec_version: 1.0.0
---

# Fix: AuditLogs vacía tras login Admin

## Descripción

Tras realizar login desde el frontend Admin, la tabla `AuditLogs` no muestra registros. Se espera que cada login exitoso (o fallido) genere un registro con `Action=LoginSuccess` o `Action=LoginFailed`.

## Causa raíz (análisis)

El flujo CQRS está correctamente implementado:
- `AdminAuthController.Login` → `AdminLoginCommand` → `AdminLoginHandler`
- `AdminLoginHandler` llama a `IAuditLogService.LogActionAsync` en éxito y fallo
- `AuditLogService` persiste en `AdminDbContext.AuditLogs`

**Posibles causas ambientales:**
1. Migraciones no aplicadas → tabla `AuditLogs` inexistente → `SaveChangesAsync` lanza; el catch en `AuditLogService` traga la excepción (RNF3).
2. Connection string apunta a BD distinta a la inspeccionada.
3. Frontend llama a otro API (ej. Product) en lugar de `/api/admin/auth/login`.

## Cambios realizados

1. **Test de integración** (`AdminLogin_WithDefaultCredentials_CreatesAuditLogRecord`): Verifica que tras login exitoso existe un registro en `AuditLogs` con `Action=LoginSuccess`.
2. **Logging mejorado** en `AuditLogService`: Incluye `Path` y `ex.Message` en el log de error para facilitar diagnóstico si `SaveChangesAsync` falla.

## Condición de salida

- [x] Test de integración que valida registro en AuditLogs tras login
- [x] Logging mejorado para diagnóstico
- [ ] Validación manual: tras login desde front, verificar en BD `SELECT * FROM AuditLogs WHERE Action='LoginSuccess'` o ejecutar el test de integración

## Validación manual

```sql
-- Tras login exitoso desde frontend
SELECT Id, CursorId, Username, Action, Path, ActionTimestamp 
FROM AuditLogs 
WHERE Action IN ('LoginSuccess', 'LoginFailed') 
ORDER BY ActionTimestamp DESC;
```

Si no hay registros, revisar logs de la API (buscar "Error al registrar log de auditoría") para ver si `SaveChangesAsync` falla.

## Archivos tocados

- `src/GesFer.Admin.Back.IntegrationTests/AdminAuthIntegrationTests.cs` — test `AdminLogin_WithDefaultCredentials_CreatesAuditLogRecord`
- `src/GesFer.Admin.Back.Infrastructure/Services/AuditLogService.cs` — logging mejorado
