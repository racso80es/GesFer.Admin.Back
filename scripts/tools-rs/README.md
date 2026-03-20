# GesFer Tools (Rust)

Implementación en Rust del contrato `SddIA/tools/tools-contract.md` (v2). Envelope JSON: `SddIA/norms/capsule-json-io.md`, tipos compartidos en el crate **`gesfer-capsule`** (`scripts/gesfer-capsule`).

## Requisitos

- [Rust](https://www.rust-lang.org/) (rustup, cargo).
- **Windows (msvc):** Visual Studio Build Tools con C++ (para `link.exe`).

## Compilar e instalar

```powershell
cd scripts/tools-rs
.\install.ps1
```

Compila `cargo build --release` y copia los `.exe` a la **raíz** de cada cápsula (`paths.toolCapsules`):

- `prepare-full-env/prepare_full_env.exe`
- `invoke-mysql-seeds/invoke_mysql_seeds.exe`
- `start-api/start_api.exe`

## Uso

- **Agente:** JSON por stdin cuando stdin no es TTY; respuesta envelope v2 por stdout.
- **Humano:** flags `clap` habituales; `--output-json` o `TOOLS_OUTPUT_JSON=1` para imprimir JSON.
- `GESFER_REPO_ROOT` — raíz del repo cuando aplica.

## Estructura

- `src/lib.rs` — Reexporta `gesfer_capsule`; `ToolResult` legacy; `to_contract_json` → `CapsuleResponse`.
- `src/bin/prepare_full_env.rs`, `invoke_mysql_seeds.rs`, `start_api.rs`

Referencia: `SddIA/tools/tools-contract.md`.
