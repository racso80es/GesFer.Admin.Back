---
feature_name: admin-login-audit-log
created: 2026-03-14
base: objectives.md, analysis.md, clarify.md
---

# Especificación: Registro en AuditLogs del evento de login Admin

**Feature:** admin-login-audit-log  
**Fecha:** 2026-03-14  
**Base:** [objectives.md](./objectives.md), [analysis.md](./analysis.md), [clarify.md](./clarify.md)

---

## 1. Objetivo

Refactorizar el login Admin a CQRS (AdminLoginCommand + Handler) y registrar en la tabla `AuditLogs` tanto los logins exitosos como los intentos fallidos, incluyendo IP y User-Agent en AdditionalData. El controlador no será responsable de la lógica ni del audit.

---

## 2. Alcance

| Incluido | No incluido |
|----------|-------------|
| AdminLoginCommand y AdminLoginHandler | FluentValidation |
| Registro en AuditLogs (éxito y fallo) | Cambios en tabla AuditLogs (esquema existente) |
| IP y User-Agent en AdditionalData | Logout u otras acciones de auth |
| Validación en Handler (Usuario/Contraseña no vacíos) | Middleware o filtros genéricos |

---

## 3. Requerimientos funcionales

### RF1 – AdminLoginCommand

- **Descripción:** Crear un command MediatR que encapsule el caso de uso de login Admin.
- **Estructura:**
  - `Usuario` (string)
  - `Contraseña` (string)
  - `ClientIp` (string?, opcional — para AdditionalData)
  - `UserAgent` (string?, opcional — para AdditionalData)
- **Ubicación:** `Application/Commands/Auth/AdminLoginCommand.cs`
- **Tipo de retorno:** `AdminLoginResult` (véase RF4) para distinguir éxito, validación, auth fallido y error.
- **Criterio de aceptación:** El command se puede enviar desde el controlador con `ISender.Send(AdminLoginCommand)`.

### RF2 – AdminLoginHandler

- **Descripción:** Handler que ejecuta el flujo completo: validación, autenticación, generación de token, registro en AuditLogs.
- **Comportamiento:**
  1. **Validación:** Si `Usuario` o `Contraseña` están vacíos o solo espacios → devolver `AdminLoginResult.ValidationError("Usuario y contraseña son requeridos")`.
  2. **Autenticación:** Llamar a `IAdminAuthService.AuthenticateAsync(Usuario, Contraseña)`.
  3. **Si adminUser == null (credenciales inválidas):**
     - Llamar a `IAuditLogService.LogActionAsync` con Action="LoginFailed", CursorId="", Username=Usuario (del request), HttpMethod="POST", Path="/api/admin/auth/login", AdditionalData con IP y UserAgent.
     - Devolver `AdminLoginResult.AuthFailure("Credenciales administrativas inválidas")`.
  4. **Si adminUser != null (éxito):**
     - Generar token con `IAdminJwtService.GenerateAdminToken`.
     - Llamar a `IAuditLogService.LogActionAsync` con Action="LoginSuccess", CursorId=adminUser.Id.ToString(), Username=adminUser.Username, HttpMethod="POST", Path="/api/admin/auth/login", AdditionalData con IP y UserAgent.
     - Devolver `AdminLoginResult.Success(AdminLoginResponse)`.
  5. **Excepciones:** Capturar y devolver `AdminLoginResult.Error(message)`; no registrar audit en errores 500 (opcional: registrar con Action="LoginError" si se desea trazabilidad de fallos técnicos).
- **Dependencias:** IAdminAuthService, IAdminJwtService, IAuditLogService.
- **Ubicación:** `Application/Handlers/Auth/AdminLoginHandler.cs`
- **Criterio de aceptación:** El handler ejecuta el flujo completo sin que el controlador orqueste servicios.

### RF3 – AdminLoginResult

- **Descripción:** Tipo de retorno que permite al controlador mapear a códigos HTTP correctos.
- **Variantes:**
  - `Success(AdminLoginResponse response)` → 200 OK
  - `ValidationError(string message)` → 400 BadRequest
  - `AuthFailure(string message)` → 401 Unauthorized
  - `Error(string message)` → 500 Internal Server Error
- **Implementación:** Record o sealed class con patron matching. Ejemplo: `AdminLoginResult` con `TryGetSuccess(out AdminLoginResponse? response)` y propiedades `HttpStatusCode`, `ErrorMessage`.
- **Ubicación:** `Application/DTOs/Auth/AdminLoginResult.cs` o en el mismo archivo del Command.
- **Criterio de aceptación:** El controlador puede hacer pattern matching o chequeo de tipo para devolver la respuesta HTTP adecuada.

### RF4 – AdditionalData (IP y User-Agent)

- **Descripción:** El campo `AdditionalData` del AuditLog debe contener un JSON con `ClientIp` y `UserAgent` cuando estén disponibles.
- **Formato:** `{"clientIp":"...","userAgent":"..."}` (valores null/empty omitidos o como string vacío según convención).
- **Origen:** El controlador obtiene `ClientIp` y `UserAgent` de `HttpContext` (Connection.RemoteIpAddress, Request.Headers["User-Agent"]) y los pasa al command.
- **Criterio de aceptación:** Tras un login (éxito o fallo), el registro en AuditLogs tiene AdditionalData con IP y UserAgent cuando el request los proporciona.

### RF5 – Refactor del AdminAuthController

- **Descripción:** El controlador debe delegar todo al command; no orquesta servicios ni registra audit.
- **Comportamiento:**
  1. Obtener `ClientIp` y `UserAgent` de HttpContext.
  2. Crear `AdminLoginCommand` con request.Usuario, request.Contraseña, ClientIp, UserAgent.
  3. `var result = await _sender.Send(command)`.
  4. Mapear result a IActionResult:
     - Success → Ok(response)
     - ValidationError → BadRequest(new { message })
     - AuthFailure → Unauthorized(new { message })
     - Error → StatusCode(500, new { message, error })
  5. En catch de excepción no controlada: LogError y StatusCode(500).
- **Dependencias:** Solo `ISender` (MediatR) e `ILogger`.
- **Criterio de aceptación:** El controlador no inyecta IAdminAuthService, IAdminJwtService ni IAuditLogService; solo ISender.

---

## 4. Requerimientos no funcionales

- **RNF1:** No introducir nuevas dependencias NuGet (FluentValidation, OneOf, etc.) salvo las ya existentes.
- **RNF2:** Mantener compatibilidad con el contrato actual de la API (AdminLoginRequest, AdminLoginResponse).
- **RNF3:** El registro en AuditLogs no debe bloquear la respuesta al cliente; si LogActionAsync falla, el handler puede capturar y loguear sin fallar el login (comportamiento actual de AuditLogService: catch interno, no rethrow).
- **RNF4:** Tests unitarios del handler y tests de integración del endpoint deben actualizarse o crearse para cubrir los nuevos flujos.

---

## 5. Arquitectura

```
[AdminAuthController]
    → ISender.Send(AdminLoginCommand)
        → AdminLoginHandler
            → IAdminAuthService.AuthenticateAsync
            → IAdminJwtService.GenerateAdminToken (si éxito)
            → IAuditLogService.LogActionAsync (siempre: éxito o fallo)
            → return AdminLoginResult
    ← AdminLoginResult
    → Map to IActionResult (Ok/BadRequest/Unauthorized/500)
```

---

## 6. Touchpoints

| Archivo | Acción |
|---------|--------|
| `Application/Commands/Auth/AdminLoginCommand.cs` | Crear |
| `Application/Handlers/Auth/AdminLoginHandler.cs` | Crear |
| `Application/DTOs/Auth/AdminLoginResult.cs` | Crear (o incluir en Command) |
| `Api/Controllers/AdminAuthController.cs` | Modificar |
| `UnitTests/.../AdminAuthControllerTests.cs` | Modificar (mock ISender) |
| `UnitTests/.../AdminLoginHandlerTests.cs` | Crear |
| `IntegrationTests/AdminAuthIntegrationTests.cs` | Verificar (debe seguir pasando) |

---

## 7. Criterios de validación (resumen)

1. Login exitoso (admin/admin123) → 200 OK, token válido, registro en AuditLogs con Action="LoginSuccess", AdditionalData con IP y UserAgent.
2. Login fallido (credenciales inválidas) → 401 Unauthorized, registro en AuditLogs con Action="LoginFailed", Username del request, AdditionalData con IP y UserAgent.
3. Request sin Usuario o Contraseña → 400 BadRequest, sin registro en AuditLogs (validación previa al intento de auth).
4. El controlador no inyecta IAdminAuthService, IAdminJwtService ni IAuditLogService.
5. Tests unitarios e integración pasan.

---

## 8. Referencias

- [objectives.md](./objectives.md)
- [analysis.md](./analysis.md)
- [clarify.md](./clarify.md)
- `src/GesFer.Admin.Back.Api/Controllers/AdminAuthController.cs`
- `src/GesFer.Admin.Back.Infrastructure/Services/AuditLogService.cs`
- `src/GesFer.Admin.Back.Application/Commands/Logs/CreateAuditLogCommand.cs`
