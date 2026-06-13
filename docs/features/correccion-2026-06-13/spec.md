---
base: ["docs/audits/AUDITORIA_2026_06_13.md"]
scope:
  in_scope:
    - Aplicar propagación de CancellationToken en AdminAuthService.
    - Aplicar propagación de CancellationToken en AuditLogService.
    - Aplicar propagación de CancellationToken en AdminJsonDataSeeder.
    - Actualizar interfaces IAdminAuthService e IAuditLogService.
  out_scope:
    - Modificar la lógica de negocio ajena a la propagación del token.
---
# Spec - Correcciones de Auditoría 2026-06-13

Implementar las mejoras de propagación de `CancellationToken` identificadas en la auditoría del 2026-06-13.
