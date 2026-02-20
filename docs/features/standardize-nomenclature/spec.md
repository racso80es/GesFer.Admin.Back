# SPEC-standardize-nomenclature.md

**Fecha:** 2026-05-22
**Estado:** Especificado
**Autor:** Jules (IA)
**Contexto:** `docs/features/standardize-nomenclature/`

## Contexto
El proyecto actual tiene inconsistencias en los nombres de directorios, proyectos y namespaces, mezclando `GesFer.Admin`, `GesFer.Product`, y `GesFer.Admin.Back`. Se requiere estandarizar todo bajo la raíz `GesFer.Admin.Back` para alinear el código con la arquitectura Clean Architecture y la nueva nomenclatura oficial.

## Arquitectura
- **Namespace Raíz:** `GesFer.Admin.Back`
- **Capas:**
    - `Api`: Controlador de entrada (`GesFer.Admin.Back.Api`).
    - `Application`: Lógica de negocio y casos de uso (`GesFer.Admin.Back.Application`).
    - `Domain`: Entidades y reglas de negocio (`GesFer.Admin.Back.Domain`).
    - `Infrastructure`: Persistencia y servicios externos (`GesFer.Admin.Back.Infrastructure`).
- **Pruebas:**
    - `UnitTests`: Pruebas unitarias (`GesFer.Admin.Back.UnitTests`).
    - `IntegrationTests`: Pruebas de integración (`GesFer.Admin.Back.IntegrationTests`).
- **Despliegue:**
    - `Dockerfile`: Actualizado para apuntar al nuevo proyecto `GesFer.Admin.Back.Api`.
    - `docker-compose.yml`: Actualizado para usar la nueva imagen y nombres de servicio.

## Seguridad
- **Riesgos:** Ninguno directo. El cambio es puramente de refactorización de nombres.
- **Validación:** Se requiere verificar que las referencias cruzadas entre proyectos sigan funcionando correctamente y que no haya fugas de información por configuración incorrecta tras el renombrado.

## Criterios de Aceptación
1.  Todos los proyectos en `src/` deben tener el prefijo `GesFer.Admin.Back.`.
2.  Todos los namespaces en `.cs` deben comenzar con `GesFer.Admin.Back.`.
3.  La solución `GesFer.Admin.Back.sln` debe compilar sin errores (`dotnet build`).
4.  Todas las pruebas existentes deben pasar (`dotnet test`).
5.  El contenedor Docker debe construirse y ejecutarse correctamente (`docker-compose up`).
6.  No deben quedar referencias a `GesFer.Admin` (sin `.Back`) excepto donde sea explícitamente requerido por librerías externas (ninguna conocida).

## Datos de Entrada/Salida
- **Entrada:** Código fuente actual con nomenclatura mixta.
- **Salida:** Código fuente refactorizado con nomenclatura estandarizada.

## Dependencias
- `SddIA/Process/feature.md`
- `AGENTS.md` (Instrucciones de sistema)
