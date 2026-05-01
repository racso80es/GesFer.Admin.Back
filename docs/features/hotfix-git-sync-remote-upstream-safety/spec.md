---
feature_name: hotfix-git-sync-remote-upstream-safety
created: "2026-05-01"
functional_requirements:
  - id: FR-01
    text: git_sync_remote comprueba upstream con rev-parse @{u}.
  - id: FR-02
    text: Sin upstream, push usa -u origin HEAD (force-with-lease si force).
  - id: FR-03
    text: Fallos devuelven success false y mensaje explícito con resumen de salida.
validation_criteria:
  - Compilación release de git_sync_remote y copia a cápsula.
  - Norma git-operations.md publicada; git-via-skills alineada.
---

# SPEC

Ver implementación en `scripts/skills-rs/src/bin/git_sync_remote.rs` y definición en `SddIA/skills/git-sync-remote/spec.md`.
