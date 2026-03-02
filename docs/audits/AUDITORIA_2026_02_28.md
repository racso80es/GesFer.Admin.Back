# Auditoría S+

**Fecha:** 2026-02-28
**Auditor:** Guardián de la Infraestructura backend
**Objetivo:** Analizar el estado del proyecto para garantizar la escalabilidad, eficiencia y mantenibilidad.

---

## 1. Métricas de Salud (0-100%)

- **Arquitectura:** 90% (Se ha avanzado en Inversión de Dependencias y Desacoplamiento de API, pero falta completar la separación estricta en ciertos puntos).
- **Nomenclatura:** 100% (Las ramas y archivos siguen el estándar `GesFer.Admin.Back.*` exigido).
- **Estabilidad Async:** 100% (Cero `async void` en el código de producción. Cero bloqueos síncronos como `.Wait()` o `.Result` en flujos asíncronos críticos, excepto el uso intencional en filtros de autorización asíncronos `IAsyncAuthorizationFilter`).

---

## 2. Pain Points (🔴 Críticos / 🟡 Medios)

### 🟡 Medio: Atributo `AuthorizeSystemOrAdminAttribute` usa asignación directa en lugar de contexto asíncrono puro

- **Hallazgo:** El filtro de autorización `AuthorizeSystemOrAdminAttribute` asigna `context.Result = new UnauthorizedResult();` en caso de fallo, lo cual es correcto para el flujo, pero el diseño de la validación del secreto y rol podría ser más robusto y centralizado en un servicio.
- **Ubicación:** `src/GesFer.Admin.Back.Api/Attributes/AuthorizeSystemOrAdminAttribute.cs` (línea 43).

### 🟡 Medio: Dependencias NuGet acopladas en API

- **Hallazgo:** El proyecto API (`GesFer.Admin.Back.Api.csproj`) tiene referencias directas a `Microsoft.EntityFrameworkCore.Design` y otros paquetes que podrían abstraerse. Aunque no rompe Clean Architecture, se debe vigilar para no añadir paquetes de implementación de infraestructura directamente en la capa de presentación.
- **Ubicación:** `src/GesFer.Admin.Back.Api/GesFer.Admin.Back.Api.csproj`.

---

## 3. Acciones Kaizen (Hoja de Ruta para el Executor)

### Acción 1: Refactorizar `AuthorizeSystemOrAdminAttribute` para usar un servicio de validación

**Instrucciones para el Executor:**
Extraer la lógica de validación de `X-Internal-Secret` a un servicio dedicado (e.g., `ISystemAuthValidationService`) inyectado vía DI, en lugar de resolverlo directamente del `HttpContext` y acceder a la configuración dentro del atributo.

**Fragmento de código sugerido:**
```csharp
// En GesFer.Admin.Back.Application.Common.Interfaces
public interface ISystemAuthValidationService
{
    bool ValidateSecret(string? secret);
}

// En el atributo
var authValidationService = httpContext.RequestServices.GetRequiredService<ISystemAuthValidationService>();
if (authValidationService.ValidateSecret(secret))
{
    return;
}
```

**Definition of Done (DoD):**
1. Se ha creado la interfaz `ISystemAuthValidationService` en Application.
2. Se ha implementado la interfaz en Infrastructure.
3. El atributo `AuthorizeSystemOrAdminAttribute` utiliza este servicio.
4. Las pruebas unitarias/integración de autenticación pasan exitosamente.

### Acción 2: Revisión de Dependencias en API

**Instrucciones para el Executor:**
Asegurar que `GesFer.Admin.Back.Api` solo referencie proyectos de abstracción (`Application`) y el de composición (`Infrastructure` solo para DI). Evitar añadir paquetes de Entity Framework Core o Pomelo directamente al API en el futuro.

**Definition of Done (DoD):**
1. Revisión manual del `.csproj` del API.
2. Documentación actualizada en `docs/features/` si se realiza alguna limpieza de NuGet.
