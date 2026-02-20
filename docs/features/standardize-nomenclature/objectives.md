# Objetivo: Estandarizar Nomenclatura a GesFer.Admin.Back

## Contexto
El proyecto tiene un origen monolítico y presenta inconsistencias en los namespaces y nombres de proyectos. La nomenclatura actual mezcla `GesFer.Admin`, `GesFer.Product`, y `GesFer.Admin.Back` de forma parcial. Se requiere estandarizar todo bajo la raíz `GesFer.Admin.Back`.

## Alcance
- **Directorios:** Renombrar carpetas en `src/` para reflejar la estructura `GesFer.Admin.Back.*`.
- **Proyectos (.csproj):** Renombrar archivos de proyecto.
- **Namespaces:** Actualizar todos los namespaces en el código fuente (`.cs`).
- **Referencias:** Actualizar dependencias entre proyectos y en la solución `.sln`.
- **Infraestructura:** Actualizar `Dockerfile` y `docker-compose.yml` para reflejar los nuevos nombres.

## Ley Aplicada
- **Estandarización:** Unificación bajo el namespace raíz `GesFer.Admin.Back`.
- **Clean Architecture:** Mantener la separación de capas (Api, Application, Domain, Infrastructure).

## Plan de Ejecución
Ver `plan.md` para el detalle paso a paso.

## Cierre y PR
El resultado final será un Pull Request que contenga:
1.  Esta documentación (`objectives.md`, `plan.md`).
2.  La reestructuración completa del código fuente.

## Referencias
- `SddIA/Process/feature.md`
