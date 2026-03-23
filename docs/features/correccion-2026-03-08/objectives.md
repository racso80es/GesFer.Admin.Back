# Objetivos: Corrección de Auditoría 2026-03-08

## 1. Contexto
Se ha realizado la auditoría del proyecto el día 2026-03-08. Se han encontrado áreas de mejora relacionadas con la arquitectura, el acoplamiento y el uso de tipos de colección en los DTOs.

## 2. Hallazgos Consolidados

| ID | Nivel | Hallazgo | Prioridad |
|---|---|---|---|
| 1 | 🟡 | Acoplamiento a Entity Framework Core en capa API (using innecesario). | Alta |
| 2 | 🟡 | Uso de `List<T>` en vez de interfaces de solo lectura (`IEnumerable<T>`) en respuestas y DTOs. | Media |
| 3 | 🟡 | Lógica de configuración de Auth (`AddAuthentication`, `AddJwtBearer`) en `Program.cs`. | Baja (ya se configuró previamente como `AddInfrastructureServices`, pero se requiere limpieza de Program.cs) |

*Nota sobre la capa Application:* Según la arquitectura de CQRS + EF Core, muchas veces se permite que la capa Application dependa de `Microsoft.EntityFrameworkCore` para el uso de `ToListAsync` y otras utilidades que simplifican el acceso directo a los DbSets a través de `IApplicationDbContext`. No se intentará quitar EF Core completamente de Application a menos que se reescriba todo con repositorios. Nos enfocaremos en remover el `using` innecesario en API y cambiar a `IEnumerable<T>`.

## 3. Alcance

- [x] Remover `using Microsoft.EntityFrameworkCore;` de `src/GesFer.Admin.Back.Api/Program.cs`.
- [x] Refactorizar los queries, commands, handlers y controladores para devolver `IEnumerable<T>` en lugar de `List<T>`.
- [x] Mover configuración JWT si aplica y no rompe la dependencia de Secrets en `builder.Configuration`.

## 4. Criterios de Aceptación (DoD)
- El proyecto compila correctamente.
- Los tests unitarios, de integración y E2E pasan al 100%.
- No existen dependencias ni `using` de `Microsoft.EntityFrameworkCore` en `Program.cs`.
- Todos los listados retornados por DTOs usan `IEnumerable<T>`.
