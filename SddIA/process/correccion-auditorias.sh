#!/bin/bash
export GITHUB_HEAD_REF="feat/correccion-2026-05-09"
git checkout -b feat/correccion-2026-05-09

mkdir -p docs/features/correccion-2026-05-09

cat << 'DOCEOF' > docs/features/correccion-2026-05-09/objectives.md
---
type: objectives
---
# Objetivos de la corrección
- Resolver memory leak termodinámico en consultas de solo lectura.
- Aplicar `.AsNoTracking()` en `GetAuditLogsHandler` y `GetLogsHandler`.
DOCEOF

cat << 'DOCEOF' > docs/features/correccion-2026-05-09/spec.md
---
type: spec
base:
  - src/GesFer.Admin.Back.Application/Queries/Logs/GetAuditLogsQuery.cs
  - src/GesFer.Admin.Back.Application/Queries/Logs/GetLogsQuery.cs
scope:
  in_scope:
    - Add `.AsNoTracking()` to read-only queries in Application project.
  out_scope:
    - Modifications outside of the specified files.
---
# Especificaciones
Añadir `.AsNoTracking()` a las consultas de solo lectura en:
- `src/GesFer.Admin.Back.Application/Queries/Logs/GetAuditLogsQuery.cs`
- `src/GesFer.Admin.Back.Application/Queries/Logs/GetLogsQuery.cs`
DOCEOF
