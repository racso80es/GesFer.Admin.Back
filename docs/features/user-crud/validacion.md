---
feature_name: user-crud
branch: feat/user-crud
base_branch: main
created: "2026-04-30"
global: pass
blocking: false
checks:
  - name: dotnet_build_release
    result: pass
    message: "dotnet build .\\src\\GesFer.Admin.Back.sln -c Release (vía invoke-command)"
  - name: dotnet_test_release
    result: pass
    message: "dotnet test .\\src\\GesFer.Admin.Back.sln -c Release --no-build (vía invoke-command)"
  - name: ef_migrations_squash_initial_b2
    result: pass
    message: "B2-RESET: migraciones squashed a una única Initial en Data/Migrations."
  - name: ef_database_drop
    result: pass
    message: "dotnet ef database drop --force (DB: GesFer_Admin)"
  - name: ef_database_update
    result: pass
    message: "dotnet ef database update (aplicada Initial nueva)"
  - name: seeds_run_seeds_only
    result: pass
    message: "RUN_SEEDS_ONLY=1 + dotnet run (migraciones+seeds)"
git_changes:
  files_added: 38
  files_modified: 7
  files_deleted: 2
---

## Evidencia

- Evidencia de ejecución: `docs/diagnostics/feat/user-crud/execution_history.json`
- B2-RESET aplicado sobre BD local `GesFer_Admin` (drop + update) y seeds en modo `RUN_SEEDS_ONLY=1`.

