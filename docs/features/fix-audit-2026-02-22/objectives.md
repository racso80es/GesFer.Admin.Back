# Objetivos: Correcciones Auditoría 2026-02-22

## Contexto
La auditoría realizada el 2026-02-22 identificó tres acciones Kaizen críticas en `GesFer.Admin.Back.Api` para mejorar la seguridad, el rendimiento y la estabilidad.

## Objetivos Principales
1.  **Eliminar Credenciales Hardcodeadas**: Retirar la cadena de conexión de fallback con contraseña en texto plano en `DependencyInjection.cs`.
2.  **Optimizar PurgeLogs**: Reemplazar la eliminación en memoria (`ToListAsync` + `RemoveRange`) por `ExecuteDeleteAsync` para evitar `OutOfMemoryException`.
3.  **Corregir Warnings de Nulabilidad**: Resolver los warnings CS8601 en `LogController.cs` para mejorar la estabilidad del código.

## Alcance
- **En Scope**: Modificaciones en `DependencyInjection.cs`, `LogController.cs`.
- **Out of Scope**: Refactorización de la arquitectura (referencias directas de Api a Infrastructure), aunque se reconoce como un problema.

## Entregables
- Código fuente modificado.
- Build limpio sin warnings en `LogController`.
- Commits atómicos por cada acción Kaizen.
