---
skill_id: sddia-evolution-register
created: 2026-03-27
capsule_ref: paths.skillCapsules.sddia-evolution-register
implementation: scripts/skills-rs/src/bin/sddia_evolution_register.rs
---

# Skill — sddia-evolution-register

Registra un cambio del protocolo SddIA bajo `./SddIA/`: genera UUID v4, escribe `SddIA/evolution/{uuid}.md`, actualiza `Evolution_log.md`, calcula `replicacion.hash_integrity` (SHA-256 del YAML canónico).

## Invocación

- Ejecutable: `sddia_evolution_register.exe` (cápsula `scripts/skills/sddia-evolution/`, vía `install.ps1`).
- Argumentos: `--input <fichero.json>` o JSON por stdin (`--input -`).
- Opcional: salida JSON según `SddIA/skills/skills-contract.md`.

## Request JSON (camelCase)

| Campo | Obligatorio | Descripción |
| :--- | :---: | :--- |
| `autor` | Sí | Responsable del registro. |
| `descripcionBreve` | Sí | Una línea (índice). |
| `tipoOperacion` | Sí | `alta` \| `baja` \| `modificacion`. |
| `contexto` | No | Motivación (opcional en binario actual; usar valores por defecto si falta). |
| `proyectoOrigenCambio` | No | Repositorio o producto. |
| `cambiosRealizados` | No | Lista `{ anterior, nuevo }`. |
| `impacto` | No | `Bajo` \| `Medio` \| `Alto`. |

**Nota:** El binario actual (`sddia_evolution_register.rs`) usa serde `rename_all = "camelCase"` sobre campos en snake_case Rust; el payload debe usar claves camelCase (`descripcionBreve`, etc.).

## Binarios hermanos

En la misma cápsula: `sddia_evolution_validate`, `sddia_evolution_watch` (ver `manifest.json`).
