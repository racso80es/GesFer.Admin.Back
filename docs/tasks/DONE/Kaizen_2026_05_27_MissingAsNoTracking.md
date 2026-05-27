---
title: "Agregar AsNoTracking a consultas EF Core de sólo lectura"
type: "kaizen"
created: "2026-05-27"
priority: "high"
status: "pending"
---

# Kaizen: Mejorar rendimiento con AsNoTracking en EF Core

## Contexto
Durante una auditoría de la base de código, se observó que algunas consultas de Entity Framework Core de sólo lectura no utilizan `.AsNoTracking()`. Esto es una deuda técnica que puede llevar a pérdida de rendimiento y memory leaks al guardar entidades innecesariamente en el contexto (tracking), cuando en realidad son sólo de consulta y no serán modificadas.

## Objetivo
Revisar las consultas de lectura a lo largo del código (especialmente en Infrastructure y Application) y añadir `.AsNoTracking()` en donde corresponda, para mejorar el rendimiento y cumplir con las normas de memoria de SddIA.

## Alcance
- `src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs`
- `src/GesFer.Admin.Back.UnitTests/Services/AuditLogServiceTests.cs` (evaluar si es test u operacion de lectura)
- Cualquier otra consulta read-only que involucre `FirstOrDefaultAsync()` o `ToListAsync()` donde las entidades no se modifiquen.

## Criterios de Aceptación
- Todas las consultas read-only usan `.AsNoTracking()`.
- Los tests siguen pasando correctamente (`dotnet test`).
- Se genera la documentación de la feature correspondiente en `docs/features/`.
