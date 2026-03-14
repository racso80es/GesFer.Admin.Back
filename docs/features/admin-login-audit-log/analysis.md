---
feature_name: admin-login-audit-log
created: 2026-03-14
purpose: Análisis de estructura del login Admin para definir objetivos de registro en AuditLogs
---

# Análisis: Estructura del Login Admin y registro en AuditLogs

## 1. Situación actual del login

### 1.1 Flujo actual (AdminAuthController)

```
[HTTP POST /api/admin/auth/login]
        ↓
AdminAuthController.Login(AdminLoginRequest)
        ↓
IAdminAuthService.AuthenticateAsync(Usuario, Contraseña)
        ↓
IAdminJwtService.GenerateAdminToken(...)
        ↓
return Ok(AdminLoginResponse)
```

**Características:**
- **No usa MediatR/CQRS.** El controlador orquesta directamente los servicios.
- **No usa Commands ni Handlers.** A diferencia de CompanyController, LogController, CountriesController, etc.
- **Responsabilidad concentrada en el controlador:** validación de entrada, autenticación, generación de token, construcción de respuesta.
- **No hay registro en AuditLogs** en ningún punto del flujo.

### 1.2 Comparación con otros controladores del proyecto

| Controlador | Patrón | MediatR | Responsabilidad del controlador |
|-------------|--------|---------|---------------------------------|
| **CompanyController** | CQRS | Sí (`_mediator.Send(command)`) | Solo envía command y devuelve resultado |
| **LogController** | CQRS | Sí (`_sender.Send(command)`) | Solo envía command y devuelve resultado |
| **CountriesController** | CQRS | Sí | Solo envía command y devuelve resultado |
| **AdminAuthController** | Servicios directos | **No** | Orquesta Auth + JWT + respuesta |

**Conclusión:** AdminAuthController es una excepción arquitectónica: no sigue el patrón CQRS usado en el resto del proyecto.

---

## 2. Requisito de entrada

> **El controlador NO será el encargado de esta responsabilidad.**

Por tanto, el registro en AuditLogs no debe añadirse como una llamada directa en `AdminAuthController.Login`. Las opciones viables son:

1. **Command/Handler (caso de uso)** — Mover la lógica de login a un `AdminLoginCommand` y su handler; el handler registra en AuditLogs.
2. **Pipeline behavior** — Un `IPipelineBehavior` de MediatR que intercepte el login y registre; requiere que el login pase por MediatR.
3. **Servicio decorado** — Un decorador de `IAdminAuthService` que registre tras autenticación exitosa; el controlador no cambia.
4. **Evento de dominio** — El handler dispara un evento tras login exitoso; un suscriptor registra en AuditLogs.
5. **Middleware/Filtro** — Interceptar peticiones a `/api/admin/auth/login` con éxito; menos acoplado al dominio pero más genérico.

---

## 3. Opciones de diseño

### Opción A: AdminLoginCommand + Handler (CQRS)

**Estructura:**
```
AdminAuthController.Login(request)
    → _sender.Send(new AdminLoginCommand(request))
        → AdminLoginHandler:
            1. AuthenticateAsync
            2. Si éxito: LogActionAsync (AuditLog)
            3. GenerateAdminToken
            4. return AdminLoginResponse
```

**Pros:**
- Alinea AdminAuth con el resto del proyecto (Company, Log, Countries).
- El controlador queda en una sola línea: `await _sender.Send(new AdminLoginCommand(request))`.
- El handler es el responsable de la lógica completa (auth + audit + token).
- Fácil de testear (unitarios del handler).

**Contras:**
- Refactorización: hay que crear Command, Handler, y mover lógica del controlador.

---

### Opción B: Pipeline Behavior de MediatR

**Estructura:**
- Crear `AdminLoginCommand` y su handler (sin audit en el handler).
- Crear `AdminLoginAuditBehavior` que intercepta `AdminLoginCommand` y, tras éxito, envía `CreateAuditLogCommand`.

**Pros:**
- Separación de concerns: el handler no conoce el audit.
- Comportamiento transversal reutilizable.

**Contras:**
- Requiere que el login use MediatR (igual que Opción A).
- Dos behaviors: uno para el command, otro para el audit.
- Más complejidad de configuración.

---

### Opción C: Decorador de IAdminAuthService

**Estructura:**
- Crear `AdminAuthAuditDecorator : IAdminAuthService` que envuelve la implementación real.
- Tras `AuthenticateAsync` exitoso, llama a `IAuditLogService.LogActionAsync` y devuelve el usuario.

**Pros:**
- Cambio mínimo en el controlador: sigue usando `IAdminAuthService`.
- No requiere MediatR para el login.
- El controlador no se entera del audit.

**Contras:**
- El decorador no tiene acceso directo al path HTTP ni al método HTTP (habría que pasarlos o inyectar IHttpContextAccessor).
- El servicio de auth no debería conocer "qué acción se está haciendo" (login vs logout); el decorador sí puede inferirlo.

---

### Opción D: Evento de dominio

**Estructura:**
- Crear `AdminLoginSucceededEvent` y publicarlo con MediatR `INotification`.
- Handler `AdminLoginSucceededEventHandler` que llama a `IAuditLogService.LogActionAsync`.

**Pros:**
- Desacoplamiento total: el handler de login no conoce el audit.
- Escalable: otros handlers pueden suscribirse (métricas, notificaciones).

**Contras:**
- Requiere que el login use MediatR para publicar el evento.
- Más complejidad (eventos, handlers, registro).

---

### Opción E: Filtro de acción (Action Filter)

**Estructura:**
- Crear `AdminLoginAuditFilter` que se ejecuta OnActionExecuted cuando el resultado es 200 OK.
- El filtro puede leer el `AdminLoginResponse` del resultado y registrar en AuditLogs.

**Pros:**
- El controlador no cambia.
- No requiere MediatR.
- Fácil de aplicar solo al endpoint de login.

**Contras:**
- El filtro necesita acceso a `IAuditLogService` y a los datos del usuario (del response).
- El filtro es una capa de infraestructura que conoce el contrato de la API; puede acoplarse al detalle del response.

---

## 4. Resumen de datos para AuditLog

| Campo | Valor en login exitoso |
|-------|------------------------|
| **CursorId** | adminUser.Id.ToString() |
| **Username** | adminUser.Username |
| **Action** | "Login" |
| **HttpMethod** | "POST" |
| **Path** | "/api/admin/auth/login" |
| **AdditionalData** | Opcional: `"{\"ip\":\"...\",\"userAgent\":\"...\"}"` |
| **ActionTimestamp** | DateTime.UtcNow |

---

## 5. Recomendación

**Opción A (AdminLoginCommand + Handler)** es la más coherente con el proyecto:

1. **Consistencia:** Company, Log, Countries usan MediatR; AdminAuth debería unificarse.
2. **Requisito cumplido:** El controlador solo envía el command; el handler es responsable del audit.
3. **Testabilidad:** El handler se prueba unitariamente con mocks de IAdminAuthService, IAdminJwtService, IAuditLogService.
4. **CQRS:** El proyecto ya usa MediatR; no introduce patrones nuevos.

**Flujo propuesto:**

```
AdminAuthController.Login(request)
    → _sender.Send(new AdminLoginCommand(request))
        → AdminLoginHandler:
            1. Validar request (o delegar a FluentValidation)
            2. adminUser = await _authService.AuthenticateAsync(...)
            3. if (adminUser == null) return Result<AdminLoginResponse>.Failure(401)
            4. token = _jwtService.GenerateAdminToken(...)
            5. await _auditService.LogActionAsync(cursorId, username, "Login", "POST", "/api/admin/auth/login", ...)
            6. return Result<AdminLoginResponse>.Success(response)
```

**Alternativa:** Si se quiere evitar un refactor completo, **Opción C (Decorador)** o **Opción E (Filtro)** permiten añadir el audit sin tocar la lógica actual del controlador, pero con menor alineación arquitectónica.

---

## 6. Archivos a crear/modificar (según Opción A)

| Acción | Archivo |
|--------|---------|
| Crear | `Application/Commands/Auth/AdminLoginCommand.cs` |
| Crear | `Application/Handlers/Auth/AdminLoginHandler.cs` |
| Modificar | `Api/Controllers/AdminAuthController.cs` — usar `ISender.Send(AdminLoginCommand)` |
| Opcional | `Application/Validators/AdminLoginCommandValidator.cs` (FluentValidation) |

---

## 7. Preguntas para definir objetivos

1. **¿Refactorizar a CQRS o solución mínima?** ¿Opción A (Command/Handler) o Opción C/E (decorador/filtro)?
2. **¿Incluir datos adicionales en AuditLog?** (IP, User-Agent, etc.)
3. **¿Registrar solo login exitoso o también intentos fallidos?** (401 podría ser auditado por seguridad)
4. **¿Validación con FluentValidation?** El proyecto ya usa validators en otros commands.
