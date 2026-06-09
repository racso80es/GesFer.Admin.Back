---
type: spec
feature_name: correccion-2026-06-09
status: active
base: []
scope:
  in_scope: true
  out_scope: false
---

# Especificación: correccion-2026-06-09

## Descripción
Se deben aplicar las correcciones de Kaizen descritas en la auditoría del 2026-06-09. Esto implica añadir y propagar adecuadamente el `CancellationToken` en las llamadas a la base de datos de los servicios indicados.

## Requisitos
- **AdminAuthService:**
  Modificar la interfaz y su implementación para recibir un `CancellationToken` con valor por defecto `default`. Propagarlo a la llamada `.FirstOrDefaultAsync(cancellationToken)`.

- **AdminJsonDataSeeder:**
  Modificar las firmas de los métodos del seeder para aceptar `CancellationToken cancellationToken = default`. Propagarlo a todos los usos de `.ToListAsync()`, `.SaveChangesAsync()` y otras operaciones asíncronas de la base de datos.
