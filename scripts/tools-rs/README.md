# GesFer Tools (Rust)

Implementación por defecto en Rust de las herramientas del contrato `SddIA/tools/tools-contract.json`.

## Requisitos

- [Rust](https://www.rust-lang.org/) (rustup, cargo) instalado.
- **Windows (target msvc):** [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) con la carga de trabajo **Desktop development with C++** (para `link.exe`). Si falta, la compilación falla con `linker 'link.exe' not found`.

## Compilar

Desde esta carpeta o desde la raíz del repo:

```powershell
cd scripts/tools-rs
cargo build --release
```

El script `install.ps1` compila y **copia los ejecutables a scripts/tools/**:

- `scripts/tools/prepare_full_env.exe` — Prepare-FullEnv (Docker, MySQL)
- `scripts/tools/invoke_mysql_seeds.exe` — Invoke-MySqlSeeds (migraciones EF, seeds)

## Uso

Los launchers `.bat` en `scripts/tools/` invocan el `.exe` de la misma carpeta (`scripts/tools/`) si existe; si no, usan el script PowerShell.

Variables de entorno opcionales:

- `TOOLS_OUTPUT_JSON=1` — Emite el resultado JSON por stdout.
- `GESFER_REPO_ROOT` — Raíz del repositorio (para invoke_mysql_seeds; por defecto el directorio actual).

Argumentos:

- `--output-path <ruta>` — Escribe el JSON de resultado en el fichero indicado.

## Estructura

- `src/lib.rs` — Tipos del contrato (`ToolResult`, `FeedbackEntry`, `to_contract_json`).
- `src/bin/prepare_full_env.rs` — Herramienta prepare-full-env.
- `src/bin/invoke_mysql_seeds.rs` — Herramienta mysql-seeds.

Referencia: `SddIA/tools/tools-contract.json`, `SddIA/agents/security-engineer.json`.
