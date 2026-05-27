---
title: "Agregar AsNoTracking a consultas EF Core de sólo lectura"
feature_id: "kaizen-asnotracking"
description: "Identificar y modificar consultas read-only en EF Core para que utilicen .AsNoTracking() mejorando el rendimiento y evitando memory leaks en el tracking de entidades."
---

# Objetivos: kaizen-asnotracking

## Propósito
Mejorar el rendimiento de consultas a base de datos y cumplir con el estándar SddIA de uso de memoria, añadiendo `.AsNoTracking()` a las consultas de EF Core que solo leen entidades (sin modificarlas ni guardarlas).

## Alcance
- Modificar `AdminAuthService` (`src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs`).
- Revisar y ajustar test methods en `AuditLogServiceTests` si es necesario (`src/GesFer.Admin.Back.UnitTests/Services/AuditLogServiceTests.cs`).
- Validar mediante ejecución de pruebas (`dotnet test`).

## Leyes Aplicadas
- Testability, Audit & Judge: El código debe funcionar correctamente y las pruebas deben pasar luego de los cambios de performance.
