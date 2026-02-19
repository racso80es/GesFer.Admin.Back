# Especificación técnica: Estandarización GesFer.Admin.Back

**Feature:** estandarizacion-gesfer-admin-back  
**{persist}:** docs/features/estandarizacion-gesfer-admin-back

## 1. Código innecesario (eliminar)

### 1.1 Funcionalidad fuera de alcance Admin

- **DashboardController** y ruta `api/admin/dashboard`: eliminar. No es CRUD empresas ni get/update empresa ni logs ni auditorías.
- **IProductApiClient / ProductApiClient**: eliminar (usado solo por Dashboard).
- **DashboardSummaryDto** (Application e Infrastructure): eliminar.
- **Tests de Dashboard**: DashboardControllerTests, ProductApiClientTests (si existen) — eliminar o adaptar solo lo que corresponda a Admin.

### 1.2 Proyectos y carpetas de Product

- **GesFer.Product.UnitTests** (src/tests/GesFer.Product.UnitTests): eliminar del repositorio (o de la solution si se usa). No pertenece al backend de Admin.
- Cualquier **GesFer.Product.IntegrationTests** en este repo: eliminar.
- **Referencias a GesFer.Api (Product)** en tests (p. ej. Architecture.Tests que referencian GesFer.Api.csproj): eliminar o reemplazar por Admin.Api.

## 2. Unificación de Shared (reubicar y eliminar)

### 2.1 Contenido a reubicar en Admin

| Origen (Shared) | Destino (Admin) |
|-----------------|-----------------|
| BaseEntity | GesFer.Admin.Domain (Common/BaseEntity.cs) namespace GesFer.Admin.Domain.Common |
| Company, City, State, Country, Language, PostalCode | GesFer.Admin.Domain (Entities) namespace GesFer.Admin.Domain.Entities |
| TaxId, Email (ValueObjects) | GesFer.Admin.Domain (ValueObjects) namespace GesFer.Admin.Domain.ValueObjects |
| ISensitiveDataSanitizer, SensitiveDataSanitizer | GesFer.Admin.Domain (Services) o Infrastructure según dependencias |
| ISequentialGuidGenerator, MySqlSequentialGuidGenerator | GesFer.Admin.Infrastructure (Persistence/Services) |
| SequentialGuidValueGenerator, DbContextExtensions | GesFer.Admin.Infrastructure (Persistence) |

### 2.2 Cambios en proyectos

- **GesFer.Admin.Domain.csproj**: eliminar ProjectReference a GesFer.Shared.Back.Domain. Tras reubicar, Domain no dependerá de Shared.
- **GesFer.Admin.Infra.csproj** (o Infrastructure): eliminar referencias a Shared; usar solo Domain e implementaciones locales de persistence/services.
- **Api, Application**: eliminar cualquier referencia a Shared; usar solo Domain/Infrastructure.

### 2.3 Migraciones EF

- Las migraciones existentes referencian entidades por nombre completo (p. ej. "GesFer.Shared.Back.Domain.Entities.Company"). Tras mover entidades a GesFer.Admin.Domain.Entities, puede ser necesario:
  - Ajustar el snapshot y migraciones para usar el nuevo nombre de entidad (GesFer.Admin.Domain.Entities.Company), o
  - Generar una migración de “renombrado” si EF lo requiere. Evaluar según guía de EF Core (renaming entity types).

### 2.4 Eliminación de Shared

- Eliminar carpeta src/Shared (y subcarpetas Back, tests, etc.).
- Eliminar GesFer.Shared.Back.Domain.sln si existe en Shared.
- No quedar ninguna referencia a GesFer.Shared en el código ni en .csproj.

## 3. Nomenclatura

### 3.1 Namespaces

- **Convención objetivo:** Todo bajo `GesFer.Admin.Back.*` o `GesFer.Admin.*` de forma coherente.
- **Decisión:** Usar **GesFer.Admin.Back** como prefijo de namespaces para alinear con el nombre del repositorio:
  - Api: GesFer.Admin.Back.Api (y .Controllers, .Attributes, etc.)
  - Application: GesFer.Admin.Back.Application
  - Domain: GesFer.Admin.Back.Domain (y .Common, .Entities, .ValueObjects, .Services si aplica)
  - Infrastructure: GesFer.Admin.Back.Infrastructure (o mantener GesFer.Admin.Infrastructure si el proyecto se sigue llamando Admin.Infra; ver 3.2)

### 3.2 Nombres de proyecto y assembly

- Unificar: el .csproj actual es **GesFer.Admin.Infra** (AssemblyName) pero en código se usa **GesFer.Admin.Infrastructure**. Opciones:
  - **A:** Renombrar proyecto/carpeta a GesFer.Admin.Infrastructure y usar namespace GesFer.Admin.Back.Infrastructure.
  - **B:** Mantener GesFer.Admin.Infra como nombre de proyecto y namespace GesFer.Admin.Back.Infra.
- Se adopta **A** para consistencia con el resto del ecosistema (Application, Domain suelen usar nombre completo). Proyecto: **GesFer.Admin.Back.Infrastructure** (carpeta Infrastructure), namespace **GesFer.Admin.Back.Infrastructure**.

### 3.3 Rutas de solución (.sln)

- La solución actual (src/GesFer.Admin.Back.sln) referencia rutas como GesFer.Admin.Back.Api, GesFer.Admin.Back.Application, etc., que no coinciden con las carpetas actuales (Api, application, domain, Infrastructure, IntegrationTests, tests). Adecuar la .sln para que apunte a los .csproj reales y, si se desea, renombrar carpetas/proyectos a GesFer.Admin.Back.* para alinear nombres de proyecto con namespaces.

## 4. Proyectos y tests

### 4.1 Proyectos en la solution

- Incluir solo: Api, Application, Domain, Infrastructure, IntegrationTests, UnitTests (Admin), y opcionalmente scripts (InitDatabase, GenerateHash) si se usan.
- Excluir: GesFer.Product.UnitTests, GesFer.Shared.Back.*, GesFer.Architecture.Tests (Shared); reemplazar por un único proyecto de Architecture.Tests bajo UnitTests que solo referencie Admin.

### 4.2 Tests unitarios

- **GesFer.Admin.UnitTests**: mantener y asegurar que solo referencien GesFer.Admin.* / GesFer.Admin.Back.*. Tras reubicar Shared, actualizar usings a GesFer.Admin.Back.Domain.*.
- Reubicar tests útiles de **GesFer.Shared.Back.UnitTests** (SensitiveDataSanitizerTests, TaxIdTests, SequentialGuidGeneratorTests) en GesFer.Admin.UnitTests con namespaces GesFer.Admin.Back.UnitTests.*.

### 4.3 Tests de integración

- **GesFer.Admin.IntegrationTests**: mantener; eliminar pruebas que dependan de Dashboard o ProductApiClient; actualizar usings a namespaces nuevos.

### 4.4 Tests de arquitectura

- Un único proyecto de arquitectura (p. ej. GesFer.Admin.Back.Architecture.Tests) que verifique dependencias solo del backend Admin (sin referencias a GesFer.Api/Product). Referenciar GesFer.Admin.Back.Api (o el proyecto API real) y validar que no existan dependencias no permitidas.

## 5. Criterios de aceptación

- [ ] No existe DashboardController ni ProductApiClient ni DashboardSummaryDto.
- [ ] No existe carpeta Shared ni referencias a GesFer.Shared en el código ni en .csproj.
- [ ] Entidades y value objects usados por Admin viven en GesFer.Admin.Back.Domain (o GesFer.Admin.Domain si se mantiene ese prefijo de forma excepcional y documentada).
- [ ] Namespaces unificados (GesFer.Admin.Back.* o documento de decisión en clarify).
- [ ] Solution compila y todos los proyectos referenciados existen y tienen rutas correctas.
- [ ] Tests unitarios e integración de Admin pasan; no quedan proyectos Product ni Shared en la solution.
- [ ] Architecture.Tests solo valida el backend Admin.
