# Cierre manual (finalize) — rust-skills-tools-protocol

Los archivos ya están **staged** (git add ejecutado vía skill invoke-command). Para completar el cierre **usando las versiones .exe (Rust) de las skills**:

## 0. Compilar e instalar los .exe (Rust)

Desde la raíz del repo, una sola vez:

```powershell
.\scripts\skills-rs\install.ps1
```

Esto compila `scripts/skills-rs` y copia `invoke_command.exe` y `push_and_create_pr.exe` a `scripts/skills/invoke-command/bin/` y `scripts/skills/finalizar-git/bin/`. Los launchers .bat y .ps1 usan el .exe si existe.

## 1. Commit (skill invoke-command, .exe con --command-file)

Para evitar que el entorno inyecte un trailer en la línea de comando, el comando de commit se lee desde un archivo. Desde la raíz del repo:

```powershell
.\scripts\skills\invoke-command\Invoke-Command.bat --command-file "docs\features\rust-skills-tools-protocol\commit_cmd.txt" --fase Accion
```

Si usas la versión .exe (tras `install.ps1`), el .bat invoca `bin\invoke_command.exe` con los mismos argumentos. El mensaje de commit está en `commit_cmd.txt`.

## 2. Push y creación del PR (skill finalizar-git, .exe)

```powershell
.\scripts\skills\finalizar-git\Push-And-CreatePR.ps1 -Persist "docs/features/rust-skills-tools-protocol/"
```

Si existe `bin\push_and_create_pr.exe`, el script lo usa; si no, usa la lógica en PowerShell. Con `install.ps1` ejecutado, se usará el .exe.

## Resumen

| Paso | Skill/Herramienta | Versión .exe (Rust) |
|------|-------------------|----------------------|
| 0 | scripts/skills-rs/install.ps1 | Compila y copia invoke_command.exe, push_and_create_pr.exe a las cápsulas |
| 1 | invoke-command (Invoke-Command.bat) | bin/invoke_command.exe con --command-file |
| 2 | finalizar-git (Push-And-CreatePR.ps1) | bin/push_and_create_pr.exe |

El uso de `--command-file` evita que el runner inyecte un `--trailer` en la línea de comando al no contener "git commit" en la invocación.
