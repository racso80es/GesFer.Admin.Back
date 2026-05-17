---
feature_name: kaizen-asnotracking
branch: feat/kaizen-asnotracking
global:
  - Handlers
checks:
  - "Se agregó AsNoTracking a las validaciones de unicidad previas a Create/Update en CreateCompanyHandler, UpdateCompanyHandler, CreateUserHandler y UpdateUserHandler."
git_changes:
  files_added: 4
  files_modified: 4
  files_deleted: 0
---
# Validación
Se verificó que los handlers que requieren tracking para updates lo mantienen, pero las consultas de lectura y validación previa incluyen `.AsNoTracking()` según normativa.
