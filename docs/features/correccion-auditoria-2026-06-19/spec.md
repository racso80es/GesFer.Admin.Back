---
type: spec
status: active
base:
  - "docs/audits/AUDITORIA_2026_06_19.md"
scope:
  in_scope:
    - "Propagar CancellationToken en llamadas a EF Core dentro de la capa Infrastructure."
    - "Actualizar IAdminAuthService e IAuditLogService."
    - "Actualizar AdminLoginHandler para pasar el CancellationToken."
    - "Actualizar pruebas unitarias que dependen de estas firmas."
  out_scope:
    - "Modificar la lógica de negocio de los servicios."
---

# Corrección de Auditoría: Estabilidad Async

Se detectaron llamadas asíncronas (`FirstOrDefaultAsync`, `SaveChangesAsync`) en los servicios de infraestructura `AdminAuthService` y `AuditLogService` que no propagan el `CancellationToken`, lo que podría generar bloqueos si se interrumpe la petición.

Esta feature implementa las acciones de la auditoría: propagar los tokens en los métodos y adaptar las interfaces, el handler de MediatR y las pruebas.
