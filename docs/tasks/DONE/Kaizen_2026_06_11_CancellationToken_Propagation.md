---
id: T-26-001
created: 2026-06-11
type: Kaizen
priority: high
status: pending
---

# Propagación de CancellationToken en EF Core

## Descripción
Se ha identificado que existen múltiples llamadas a base de datos usando Entity Framework Core (`FirstOrDefaultAsync`, `ToListAsync`, etc.) que no propagan el `CancellationToken`. Esto puede llevar a bloqueos del thread pool en caso de cancelaciones de peticiones HTTP o timeouts.

## Objetivos
1. Identificar llamadas asíncronas a EF Core en los repositorios/servicios/handlers (p. ej. en `src/`).
2. Añadir el parámetro `CancellationToken` y propagarlo a los métodos de LINQ como `FirstOrDefaultAsync`, `ToListAsync`, etc.
3. Mantener la compatibilidad y verificar que no hay regresiones.

## Alcance
- Handlers en `src/GesFer.Admin.Back.Application`
- Servicios y Contextos en `src/GesFer.Admin.Back.Infrastructure`

## Criterios de Aceptación (DoD)
- [ ] No existen llamadas asíncronas de EF Core sin propagar `CancellationToken` (donde corresponda).
- [ ] El proyecto compila sin errores.
- [ ] Las pruebas unitarias y de integración pasan correctamente.
