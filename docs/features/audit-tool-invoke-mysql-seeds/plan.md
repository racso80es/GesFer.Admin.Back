---
process_id: audit-tool
tool_id: invoke-mysql-seeds
date: 2026-04-28
status: draft
cases:
  - id: T1
    name: Help CLI
    purpose: Verificar ejecutable y flags expuestos.
  - id: T2
    name: DropCreateDb only
    purpose: Verificar estrategia B sin migraciones ni seeds.
  - id: T3
    name: Full run
    purpose: Verificar objetivo completo (migraciones + seeds).
---

## Plan de pruebas

Los comandos y evidencias se guardarán en `docs/audits/tools/invoke-mysql-seeds/` según el proceso `audit-tool`.

