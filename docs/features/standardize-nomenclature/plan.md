# Plan de Ejecución: Estandarización de Nomenclatura

## 1. Initialize Feature Documentation (Completado)
- [x] Crear directorio `docs/features/standardize-nomenclature/`.
- [x] Crear `objectives.md` describiendo el alcance y objetivo.
- [x] Crear `plan.md` con los pasos detallados.

## 2. Rename Folders and Projects
- [ ] Renombrar directorio `src/Api` a `src/GesFer.Admin.Back.Api` y renombrar `GesFer.Admin.Api.csproj` a `GesFer.Admin.Back.Api.csproj`.
- [ ] Renombrar directorio `src/application` a `src/GesFer.Admin.Back.Application` y renombrar `GesFer.Admin.Application.csproj` a `GesFer.Admin.Back.Application.csproj`.
- [ ] Renombrar directorio `src/domain` a `src/GesFer.Admin.Back.Domain` y renombrar `GesFer.Admin.Domain.csproj` a `GesFer.Admin.Back.Domain.csproj`.
- [ ] Renombrar directorio `src/Infrastructure` a `src/GesFer.Admin.Back.Infrastructure` y renombrar `GesFer.Admin.Infra.csproj` a `GesFer.Admin.Back.Infrastructure.csproj`.
- [ ] Renombrar directorio `src/tests/GesFer.Admin.UnitTests` a `src/GesFer.Admin.Back.UnitTests` y renombrar `GesFer.Admin.UnitTests.csproj` a `GesFer.Admin.Back.UnitTests.csproj`.
- [ ] Renombrar directorio `src/IntegrationTests` a `src/GesFer.Admin.Back.IntegrationTests` y renombrar `GesFer.Admin.IntegrationTests.csproj` a `GesFer.Admin.Back.IntegrationTests.csproj`.

## 3. Update Code Namespaces and Usings
- [ ] Actualizar todos los archivos `.cs` en los proyectos renombrados para usar namespaces que comiencen con `GesFer.Admin.Back.` en lugar de `GesFer.Admin.`.
- [ ] Asegurar que `GesFer.Admin.Infra` sea actualizado a `GesFer.Admin.Back.Infrastructure`.
- [ ] Actualizar directivas `using` en todos los archivos para reflejar los nuevos namespaces.

## 4. Update Project References and Solution
- [ ] Actualizar archivos `.csproj` para referenciar los proyectos renombrados con sus nuevas rutas y nombres.
- [ ] Actualizar `src/GesFer.Admin.Back.sln` para incluir los proyectos renombrados en sus nuevas ubicaciones y eliminar las referencias antiguas.

## 5. Update Infrastructure Configuration
- [ ] Actualizar `src/Dockerfile` para referenciar la ruta y nombre correctos del proyecto (`src/GesFer.Admin.Back.Api/GesFer.Admin.Back.Api.csproj`).
- [ ] Actualizar `docker-compose.yml` para reflejar los cambios en nombres de servicio y contexto de construcción si es necesario (principalmente `gesfer-admin-api`).

## 6. Verification
- [ ] Ejecutar `dotnet build src/GesFer.Admin.Back.sln` para asegurar que la solución compile sin errores.
- [ ] Ejecutar `dotnet test src/GesFer.Admin.Back.sln` para asegurar que todas las pruebas pasen.

## 7. Submit Change
- [ ] Confirmar cambios y enviar Pull Request con la refactorización completa.
