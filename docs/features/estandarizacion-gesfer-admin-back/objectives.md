# Objetivos: Estandarización GesFer.Admin.Back

**Proceso:** Feature (SddIA/process/feature.md)  
**Rama:** feat/estandarizacion-gesfer-admin-back  
**Ruta (Cúmulo):** paths.featurePath/estandarizacion-gesfer-admin-back

## Objetivo

Adecuar el proyecto GesFer.Admin.Back (backend de administración del monolito GesFer) para que contenga **únicamente** la gestión de administración: CRUD de empresas, get/update de empresa concreta, logs y auditorías. Unificar y eliminar dependencias del directorio Shared, unificar nomenclatura y adecuación de proyectos y tests.

## Alcance

- **In scope:** Código en `src/` (Api, Application, Domain, Infrastructure, tests, IntegrationTests). Eliminación de Shared como carpeta y dependencia. Namespaces y nombres de proyecto alineados a GesFer.Admin.Back.
- **Out of scope:** Cambios en otros repos (Product, Front); solo este repositorio.

## Ley aplicada

- **Ley GIT:** Trabajo en rama feat/; no commit en master.
- **Ley SOBERANÍA:** Documentación canónica en docs/features/estandarizacion-gesfer-admin-back/.
- **Ley COMPILACIÓN:** El código debe compilar y los tests relevantes pasar tras los cambios.

## Acciones (resumen)

1. **Detectar y eliminar código innecesario**  
   En administración solo debe existir: CRUD empresas, get/update empresa concreta, logs, auditorías. Eliminar: Dashboard (y ProductApiClient, DashboardSummaryDto), tests/código de Product (GesFer.Product.UnitTests, referencias a GesFer.Api/Product).

2. **Unificar directorio Shared**  
   Reubicar en Admin lo que Admin use de Shared (BaseEntity, Company, entidades de dirección, ValueObjects TaxId/Email, servicios SensitiveDataSanitizer, ISequentialGuidGenerator/MySqlSequentialGuidGenerator, SequentialGuidValueGenerator, DbContextExtensions). Eliminar referencias a Shared en .csproj y eliminar el directorio Shared.

3. **Adecuar nomenclatura**  
   Unificar namespaces a convención GesFer.Admin.Back (o GesFer.Admin coherente). Unificar nombre de proyecto Infra vs Infrastructure (AssemblyName vs carpetas).

4. **Adecuar proyectos y tests**  
   Solution solo con proyectos Admin; tests unitarios e integración solo para Admin; reubicar/adaptar tests que estaban en Shared (p. ej. SensitiveDataSanitizer, SequentialGuid) en GesFer.Admin.UnitTests; Architecture.Tests solo para Admin.

## Fases del proceso (0–8)

| Fase | Estado |
|------|--------|
| 0 Preparar entorno | Hecho (rama feat creada) |
| 1 Documentación objetivos | Hecho (este documento) |
| 2 Especificación | spec.md / spec.json |
| 3–8 Clarify, Plan, Implementación, Ejecución, Validar, Finalizar | Según proceso |

## Referencias

- AGENTS.md (protocolo maestro)
- SddIA/process/feature.md
- SddIA/agents/cumulo.json (paths.featurePath)
