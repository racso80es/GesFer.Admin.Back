# IMPL-standardize-nomenclature.md

**Fecha:** 2026-05-22
**Estado:** Pendiente de ejecución
**Plan Base:** `docs/features/standardize-nomenclature/plan.md`
**Spec Base:** `docs/features/standardize-nomenclature/spec.md`
**Autor:** Jules (IA)

## Resumen de Cambios
Este documento detalla los pasos exactos para estandarizar la nomenclatura del proyecto a `GesFer.Admin.Back`. Se renombrarán carpetas, proyectos y namespaces, y se actualizarán las referencias en la solución y la configuración de Docker.

---

## 2. Rename Folders and Projects

### 2.1 Renombrar Proyecto Api
- **Id:** 2.1
- **Acción:** Renombrar Directorio y Archivo
- **Ruta Original:** `src/Api/`
- **Ruta Nueva:** `src/GesFer.Admin.Back.Api/`
- **Archivo Original:** `src/Api/GesFer.Admin.Api.csproj`
- **Archivo Nuevo:** `src/GesFer.Admin.Back.Api/GesFer.Admin.Back.Api.csproj`

### 2.2 Renombrar Proyecto Application
- **Id:** 2.2
- **Acción:** Renombrar Directorio y Archivo
- **Ruta Original:** `src/application/`
- **Ruta Nueva:** `src/GesFer.Admin.Back.Application/`
- **Archivo Original:** `src/application/GesFer.Admin.Application.csproj`
- **Archivo Nuevo:** `src/GesFer.Admin.Back.Application/GesFer.Admin.Back.Application.csproj`

### 2.3 Renombrar Proyecto Domain
- **Id:** 2.3
- **Acción:** Renombrar Directorio y Archivo
- **Ruta Original:** `src/domain/`
- **Ruta Nueva:** `src/GesFer.Admin.Back.Domain/`
- **Archivo Original:** `src/domain/GesFer.Admin.Domain.csproj`
- **Archivo Nuevo:** `src/GesFer.Admin.Back.Domain/GesFer.Admin.Back.Domain.csproj`

### 2.4 Renombrar Proyecto Infrastructure
- **Id:** 2.4
- **Acción:** Renombrar Directorio y Archivo
- **Ruta Original:** `src/Infrastructure/`
- **Ruta Nueva:** `src/GesFer.Admin.Back.Infrastructure/`
- **Archivo Original:** `src/Infrastructure/GesFer.Admin.Infra.csproj`
- **Archivo Nuevo:** `src/GesFer.Admin.Back.Infrastructure/GesFer.Admin.Back.Infrastructure.csproj`

### 2.5 Renombrar Proyecto UnitTests
- **Id:** 2.5
- **Acción:** Renombrar Directorio y Archivo
- **Ruta Original:** `src/tests/GesFer.Admin.UnitTests/`
- **Ruta Nueva:** `src/GesFer.Admin.Back.UnitTests/`
- **Archivo Original:** `src/tests/GesFer.Admin.UnitTests/GesFer.Admin.UnitTests.csproj`
- **Archivo Nuevo:** `src/GesFer.Admin.Back.UnitTests/GesFer.Admin.Back.UnitTests.csproj`

### 2.6 Renombrar Proyecto IntegrationTests
- **Id:** 2.6
- **Acción:** Renombrar Directorio y Archivo
- **Ruta Original:** `src/IntegrationTests/`
- **Ruta Nueva:** `src/GesFer.Admin.Back.IntegrationTests/`
- **Archivo Original:** `src/IntegrationTests/GesFer.Admin.IntegrationTests.csproj`
- **Archivo Nuevo:** `src/GesFer.Admin.Back.IntegrationTests/GesFer.Admin.Back.IntegrationTests.csproj`

---

## 3. Update Code Namespaces and Usings

### 3.1 Actualizar Namespaces en todos los archivos .cs
- **Id:** 3.1
- **Acción:** Buscar y Reemplazar Global
- **Ruta:** `src/**/*.cs`
- **Propuesta:**
    - Reemplazar `namespace GesFer.Admin.Api` por `namespace GesFer.Admin.Back.Api`
    - Reemplazar `namespace GesFer.Admin.Application` por `namespace GesFer.Admin.Back.Application`
    - Reemplazar `namespace GesFer.Admin.Domain` por `namespace GesFer.Admin.Back.Domain`
    - Reemplazar `namespace GesFer.Admin.Infra` por `namespace GesFer.Admin.Back.Infrastructure` (Nota: cambio de `Infra` a `Infrastructure`)
    - Reemplazar `namespace GesFer.Admin.UnitTests` por `namespace GesFer.Admin.Back.UnitTests`
    - Reemplazar `namespace GesFer.Admin.IntegrationTests` por `namespace GesFer.Admin.Back.IntegrationTests`

### 3.2 Actualizar Usings en todos los archivos .cs
- **Id:** 3.2
- **Acción:** Buscar y Reemplazar Global
- **Ruta:** `src/**/*.cs`
- **Propuesta:**
    - Reemplazar `using GesFer.Admin.Api` por `using GesFer.Admin.Back.Api`
    - Reemplazar `using GesFer.Admin.Application` por `using GesFer.Admin.Back.Application`
    - Reemplazar `using GesFer.Admin.Domain` por `using GesFer.Admin.Back.Domain`
    - Reemplazar `using GesFer.Admin.Infra` por `using GesFer.Admin.Back.Infrastructure`

---

## 4. Update Project References and Solution

### 4.1 Actualizar Referencias en .csproj
- **Id:** 4.1
- **Acción:** Modificar contenido
- **Ruta:** Todos los archivos `.csproj` renombrados.
- **Propuesta:**
    - Buscar referencias `<ProjectReference Include="..\RutaAntigua\ProyectoAntiguo.csproj" />` y actualizarlas a las nuevas rutas y nombres.
    - Ejemplo: En `Api.csproj`, cambiar `..\application\GesFer.Admin.Application.csproj` por `..\GesFer.Admin.Back.Application\GesFer.Admin.Back.Application.csproj`.

### 4.2 Actualizar Solución (.sln)
- **Id:** 4.2
- **Acción:** Modificar contenido (o regenerar)
- **Ruta:** `src/GesFer.Admin.Back.sln`
- **Propuesta:**
    - Actualizar las rutas de los proyectos (`Project(...) = "Nombre", "Ruta\Nombre.csproj", ...`).
    - Eliminar referencias a los proyectos antiguos.
    - Asegurar que todos los proyectos renombrados estén incluidos correctamente.

---

## 5. Update Infrastructure Configuration

### 5.1 Actualizar Dockerfile
- **Id:** 5.1
- **Acción:** Modificar contenido
- **Ruta:** `src/Dockerfile`
- **Propuesta:**
    - Cambiar `RUN dotnet restore src/GesFer.Admin.Back.Api/GesFer.Admin.Back.Api.csproj` (asegurar ruta correcta).
    - Cambiar `COPY . .` (ya correcto).
    - Cambiar `RUN dotnet publish src/GesFer.Admin.Back.Api/GesFer.Admin.Back.Api.csproj ...`.
    - Cambiar `ENTRYPOINT ["dotnet", "GesFer.Admin.Back.Api.dll"]`.

### 5.2 Actualizar docker-compose.yml
- **Id:** 5.2
- **Acción:** Modificar contenido
- **Ruta:** `docker-compose.yml` (en raíz)
- **Propuesta:**
    - En servicio `gesfer-admin-api`:
        - `build: dockerfile:` -> `src/Dockerfile` (asegurar que apunte al archivo correcto).
        - Si el contexto cambia, ajustarlo.

---

## 6. Verification (Manual)

### 6.1 Compilación
- **Id:** 6.1
- **Acción:** Ejecutar comando
- **Comando:** `dotnet build src/GesFer.Admin.Back.sln`

### 6.2 Pruebas
- **Id:** 6.2
- **Acción:** Ejecutar comando
- **Comando:** `dotnet test src/GesFer.Admin.Back.sln`
