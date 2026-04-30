---
process_id: audit-tool
tool_id: invoke-mysql-seeds
date: 2026-04-28
status: draft
---

## Objetivos de auditoría

- Verificar que `invoke-mysql-seeds` es **ejecutable** como `.exe` en la **raíz** de su cápsula.
- Verificar **contrato tools v2** (envelope `capsule-json-io`): campos obligatorios, coherencia `success`/`exitCode`, payload en `result`.
- Verificar **trazabilidad**: fases observadas en `feedback[].phase` cubren las fases declaradas en `SddIA/tools/invoke-mysql-seeds/spec.md` (según caso de prueba).
- Verificar objetivo funcional declarado:
  - Comprueba disponibilidad de MySQL.
  - Aplica migraciones EF Core (estructura).
  - Ejecuta seeds (RUN_SEEDS_ONLY=1).
  - Elimina datos previos con **estrategia B** (Drop/Create DB) cuando se habilita `DropCreateDb`.

