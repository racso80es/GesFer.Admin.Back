# REPORTE DE AUDITORÍA S+ (2026-02-20)

## 1. Métricas de Salud (0-100%)
*   **Arquitectura: 40%**
    *   🔴 **Compilación Rota**: El proyecto no compila debido a la falta de DTOs (`CreateLogDto`, etc.) en la capa `Application`.
    *   🔴 **Violación de Capas (Crítico)**: La capa `Api` (Controladores) accede directamente a `AdminDbContext` (Infraestructura), saltándose la lógica de Aplicación.
    *   🔴 **Dependencia Inversa**: La capa `Application` tiene una referencia directa a `Infrastructure`, violando el principio de inversión de dependencias.
*   **Nomenclatura: 80%**
    *   🟡 **Inconsistencia de Casing**: Carpetas `application` y `domain` están en minúsculas, mientras que `Api` e `Infrastructure` están en PascalCase.
    *   🟡 **Código Legacy**: Existencia de `src/tests/GesFer.Product.IntegrationTests/` que no pertenece al dominio `GesFer.Admin`.
*   **Estabilidad Async: 95%**
    *   ✅ No se detectaron llamadas bloqueantes (`.Result`, `.Wait()`) en el código inspeccionado.

## 2. Pain Points (🔴 Críticos / 🟡 Medios)

*   **Hallazgo 1: Compilación Rota (🔴 Crítico)**
    *   **Descripción**: Faltan las clases DTO requeridas por `LogController` en el namespace `GesFer.Admin.Application.Dtos.Logs`.
    *   **Ubicación**: `src/Api/Controllers/LogController.cs` (referencias rotas), `src/application/DTOs/` (falta carpeta Logs).
    *   **Impacto**: Imposible compilar y ejecutar tests.

*   **Hallazgo 2: Acoplamiento Api -> Infraestructura (🔴 Crítico)**
    *   **Descripción**: `LogController` inyecta y usa `AdminDbContext` directamente para operaciones de base de datos.
    *   **Ubicación**: `src/Api/Controllers/LogController.cs`.
    *   **Impacto**: Alta dependencia de la implementación de persistencia, lógica de negocio dispersa en controladores.

*   **Hallazgo 3: Dependencia Application -> Infrastructure (🔴 Crítico)**
    *   **Descripción**: El proyecto `GesFer.Admin.Application` referencia al proyecto `GesFer.Admin.Infra`.
    *   **Ubicación**: `src/application/GesFer.Admin.Application.csproj`.
    *   **Impacto**: Violación de Clean Architecture. Dificulta el testing unitario de la capa de aplicación.

*   **Hallazgo 4: Estructura de Tests Confusa y Legacy (🟡 Medio)**
    *   **Descripción**: Existe carpeta `src/tests/` con `GesFer.Product.IntegrationTests` (código muerto) y `GesFer.Admin.UnitTests` (ubicación no estándar según memoria). `IntegrationTests` está en la raíz `src/IntegrationTests/`.
    *   **Ubicación**: `src/tests/`.

*   **Hallazgo 5: Inconsistencia de Nombres de Carpetas (🟡 Medio)**
    *   **Descripción**: Carpetas `application` y `domain` en minúsculas.
    *   **Ubicación**: `src/application`, `src/domain`.

## 3. Acciones Kaizen (Hoja de Ruta para el Executor)

### Acción 1: Reparar Compilación [PRIORIDAD MÁXIMA]
*   **Instrucción**: Crear los DTOs faltantes en `src/application/DTOs/Logs/`.
*   **Fragmento de Código (Ejemplo CreateLogDto.cs)**:
    ```csharp
    namespace GesFer.Admin.Application.Dtos.Logs;
    public class CreateLogDto {
        public string Level { get; set; }
        public string Message { get; set; }
        public string? Exception { get; set; }
        public DateTime TimeStamp { get; set; }
        public Dictionary<string, object>? Properties { get; set; }
    }
    ```
    (Repetir para `CreateAuditLogDto`, `LogDto`, `LogsPagedResponseDto`, `PurgeLogsResponseDto` basándose en el uso en `LogController`).
*   **DoD**: `dotnet build` exitoso (o al menos avanza más allá de estos errores).

### Acción 2: Limpieza de Código Legacy [PRIORIDAD ALTA]
*   **Instrucción**:
    1.  Mover `src/tests/GesFer.Admin.UnitTests/` a `src/UnitTests/`.
    2.  Eliminar carpeta `src/tests/`.
    3.  Actualizar referencia en `GesFer.Admin.Back.sln`.
*   **DoD**: Estructura de carpetas limpia y sin código de `GesFer.Product`.

### Acción 3: Desacoplar Controlador (Mediator) [PRIORIDAD MEDIA]
*   **Instrucción**:
    1.  Crear Commands/Queries en `Application` para `CreateLog`, `GetLogs`, `PurgeLogs`.
    2.  Implementar Handlers usando `IApplicationDbContext` (interfaz a definir).
    3.  Refactorizar `LogController` para enviar comandos vía `IMediator`.
*   **DoD**: `LogController` no depende de `AdminDbContext`.

### Acción 4: Inversión de Dependencias (App -> Infra) [PRIORIDAD MEDIA]
*   **Instrucción**:
    1.  Definir `IApplicationDbContext` y otras interfaces en `Application`.
    2.  Hacer que `AdminDbContext` implemente `IApplicationDbContext` en `Infrastructure`.
    3.  Eliminar referencia a `GesFer.Admin.Infra` en `GesFer.Admin.Application.csproj`.
*   **DoD**: `Application` compila sin referencia a `Infrastructure`.

### Acción 5: Renombrar Carpetas [PRIORIDAD BAJA]
*   **Instrucción**: Renombrar `src/application` a `src/Application` y `src/domain` a `src/Domain`.
*   **DoD**: Estructura de carpetas consistente PascalCase.
