# Reporte de Auditoría S+
**Fecha:** 2026-03-02

## 1. Métricas de Salud (0-100%)
Arquitectura: 95% | Nomenclatura: 100% | Estabilidad Async: 100%

## 2. Pain Points (🔴 Críticos / 🟡 Medios)

🟡 **Medio**
Hallazgo: [Violación de Arquitectura Limpia] El controlador API `AdminAuthController` está importando y dependiendo directamente de interfaces de `GesFer.Admin.Back.Infrastructure` (`IAdminAuthService`, `IAdminJwtService`) en lugar de depender de abstracciones de `Application`. Además, hay DTOs (`AdminLoginRequest`, `AdminLoginResponse`) definidos temporalmente en el mismo archivo del controlador, los cuales deberían estar en `Application/DTOs`.

Ubicación: `src/GesFer.Admin.Back.Api/Controllers/AdminAuthController.cs`, líneas 3, 16, 17, 72-88.

## 3. Acciones Kaizen (Hoja de Ruta para el Executor)

**Instrucción 1: Mover DTOs a la capa Application**
- Mover las clases `AdminLoginRequest` y `AdminLoginResponse` al proyecto `GesFer.Admin.Back.Application`, en la carpeta `DTOs/Auth/` (crear si no existe).

**Instrucción 2: Mover Interfaces a la capa Application**
- Mover las interfaces `IAdminAuthService` y `IAdminJwtService` desde `GesFer.Admin.Back.Infrastructure.Services` a `GesFer.Admin.Back.Application.Common.Interfaces`. Asegurarse de actualizar el `namespace` de dichas interfaces a `GesFer.Admin.Back.Application.Common.Interfaces`.
- Actualizar las implementaciones concretas en `GesFer.Admin.Back.Infrastructure` para que implementen las interfaces desde su nueva ubicación en `Application`.

**Instrucción 3: Limpiar el Controlador**
- Eliminar el `using GesFer.Admin.Back.Infrastructure.Services;` del archivo `src/GesFer.Admin.Back.Api/Controllers/AdminAuthController.cs`.
- Actualizar los `using` en el controlador para referenciar los DTOs e interfaces desde `GesFer.Admin.Back.Application`.

**Definition of Done (DoD):**
1.  El archivo `AdminAuthController.cs` no contiene `using GesFer.Admin.Back.Infrastructure.Services`.
2.  Las clases `AdminLoginRequest` y `AdminLoginResponse` están en la capa `Application`.
3.  Las interfaces `IAdminAuthService` y `IAdminJwtService` están en la capa `Application`.
4.  El proyecto compila sin errores (`dotnet build`).
5.  Las pruebas unitarias y de integración pasan correctamente (`dotnet test`).
