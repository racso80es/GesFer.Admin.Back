# gesfer-skills (Rust)

Implementación en Rust de skills ejecutables (contrato `SddIA/skills/skills-contract.md` v2). Envelope JSON: `SddIA/norms/capsule-json-io.md`.

## Binarios

| Binario | Skill | Cápsula (raíz) |
|---------|--------|----------------|
| iniciar_rama.exe | iniciar-rama | `scripts/skills/iniciar-rama/` |
| merge_to_master_cleanup.exe | finalizar-git (post_pr) | `scripts/skills/finalizar-git/` |
| push_and_create_pr.exe | finalizar-git (pre_pr) | `scripts/skills/finalizar-git/` |
| invoke_command.exe | invoke-command | `scripts/skills/invoke-command/` |
| invoke_commit.exe | invoke-commit | `scripts/skills/invoke-commit/` |
| verify_pr_protocol.exe | (utilidad; no copiada por install) | — |

## Build e instalación

```powershell
.\scripts\skills-rs\install.ps1
```

- `cargo build --release`
- Copia cada `.exe` a **la raíz** de `scripts/skills/<skill-id>/` (sin carpeta `bin/`).

## Uso

- **Agente:** pipe JSON por stdin; respuesta JSON por stdout.
- **Humano:** mismos argumentos CLI que antes en TTY (o `.bat` en la cápsula).

Rutas: Cúmulo `paths.skillCapsules` (`SddIA/agents/cumulo.json`).
