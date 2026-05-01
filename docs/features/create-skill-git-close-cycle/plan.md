---
feature_name: create-skill-git-close-cycle
artifact: plan
process: feature
---

## Plan

| Fase | Entrega |
| --- | --- |
| 0 | `git-workspace-recon` → `git-branch-manager` rama `feat/create-skill-git-close-cycle`. |
| 1–5 | Documentación en esta carpeta (objectives, spec, plan, implementation). |
| 6 | `spec.md` SddIA skill; `cumulo.paths.json`; `git_close_cycle.rs`; `Cargo.toml`; cápsula (`manifest.json`, `.bat`); `index.json`; `install.ps1`; `finalize-process/spec.md`; difusión normas/reglas. |
| 7 | Compilar/copiar con `install.ps1`; probar cápsula; `validacion.md`. |
| 8 | `sddia_evolution_register` → `git-save-snapshot` → `git-save-snapshot` (evolution) → `git-sync-remote` → `git-create-pr`. |

## Riesgos

- Repos sin `main` ni `master` local: la skill falla con mensaje explícito.
- Ejecutar **git-close-cycle** antes de fusión remota elimina la rama de trabajo con `-D`: el contrato de **finalize-process** debe acotar el paso a **post-fusión**.
