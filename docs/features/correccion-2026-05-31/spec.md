---
type: specification
feature_name: correccion-2026-05-31
branch: feat/correccion-2026-05-31
base: ["docs/audits/AUDITORIA_2026_05_31.md"]
scope:
  in_scope:
    - "Añadir CancellationToken a las firmas de los métodos y pasarlo a las llamadas asíncronas de base de datos."
    - "Añadir AsNoTracking() a las consultas de solo lectura en AdminJsonDataSeeder."
  out_scope:
    - "Refactorizaciones arquitectónicas mayores no mencionadas en la auditoría."
---

# Especificación

Basado en el informe de auditoría `AUDITORIA_2026_05_31.md`, se aplicarán las siguientes correcciones de deuda técnica:
- Modificar `AdminJsonDataSeeder.cs`, `AuditLogService.cs`, `AdminAuthService.cs` y los Handlers/Commands de CQRS en la capa de Aplicación para incluir `CancellationToken` y `AsNoTracking()`.
