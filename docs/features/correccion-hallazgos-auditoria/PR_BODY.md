## Objetivo
Corrección de los hallazgos indicados en la auditoría `docs/audits/validacion-main-20260217.json`.

## Cambios
- **Build (bloqueante):** Referencias a Shared corregidas para usar `src/Shared/Back/` dentro del repositorio (Domain e Infrastructure .csproj y `GesFer.Product.sln`). Build y tests pasan.
- Documentación de la feature en `docs/features/correccion-hallazgos-auditoria/`.

## Referencia canónica (SSOT)
`docs/features/correccion-hallazgos-auditoria/`
- `objectives.md` — Objetivo y plan de corrección.
- `audit-hallazgos.json` — Inventario de hallazgos y estado.
