# gesfer-skills (Rust)

Implementación en Rust de skills ejecutables (contrato `SddIA/skills/skills-contract.md` v2). Envelope JSON: `SddIA/norms/capsule-json-io.md`.

## Binarios

| Binario | Skill | Cápsula (raíz) |
|---------|--------|----------------|
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
