---
type: spec
feature_name: correccion-2026-05-22
base:
  - docs/audits/AUDITORIA_2026_05_22.md
scope:
  in_scope:
    - Add AsNoTracking to AdminAuthService read query
  out_scope:
    - None
---

# Especificaciones
El sistema carece de AsNoTracking() en la consulta de lectura de AdminAuthService.
