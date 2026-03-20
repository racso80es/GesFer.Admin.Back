---
feature_name: refactorization-skills-exe-root-json-io
created: 2026-03-20
process: feature
phases:
  - id: P1
    name: Normas y contratos v2
    status: hecho
  - id: P2
    name: Crate gesfer-capsule + refactor Rust skills/tools
    status: hecho
  - id: P3
    name: Cápsulas (install, bat, manifest, índice)
    status: hecho
  - id: P4
    name: Documentación residual y tools sin binario Rust
    status: pendiente_light
---

# Plan: Adecuación a contrato v2 (exe raíz + JSON)

## Fases ejecutadas

1. **Contratos / normas** — `capsule-json-io.md`, `skills-contract` y `tools-contract` 2.0.0 (trabajo previo en la feature).
2. **`scripts/gesfer-capsule`** — Tipos `CapsuleRequest` / `CapsuleResponse`, `FeedbackEntry`, `try_read_capsule_request`, `write_capsule_response`.
3. **gesfer-tools** — Salida envelope v2; stdin JSON opcional en `prepare_full_env`, `invoke_mysql_seeds`, `start_api`; eliminación de bins huérfanos en `Cargo.toml` (`gesfer_manager`, `dual_to_frontmatter`).
4. **gesfer-skills** — Todos los bins soportan TTY (legacy) o JSON stdin; `install.ps1` copia a raíz de cápsula.
5. **Launchers** — `.bat` solo llaman a `.exe` en la misma carpeta; sin fallback `.ps1` en tools preparados / start-api / mysql / run-tests-local / postman (este último hasta que exista binario).

## Pendiente ligero

- Alinear textos en `SddIA/skills/*/spec.md` y `.md` de cápsulas que aún mencionen `bin/` o `.ps1`.
- **postman-mcp-validation** y **run-tests-local**: implementar o restaurar binarios Rust si se requiere flujo humano sin .ps1.
- **verify_pr_protocol.exe**: no se copia por `install.ps1`; si debe distribuirse, añadir entrada en install y cápsula dedicada.

---

*Siguiente acción de proceso: `validacion.md` / cierre según feature.*
