# Skill verify-pr-protocol — Cápsula

**skillId:** verify-pr-protocol  
**Ruta canónica:** Cúmulo `paths.skillCapsules["verify-pr-protocol"]` (`scripts/skills/verify-pr-protocol/`)

## Comportamiento

1. Validación de nomenclatura de rama (`scripts/validate-nomenclatura.ps1` vía PowerShell).
2. `dotnet build src/GesFer.Admin.Back.sln`
3. `dotnet test src/GesFer.Admin.Back.sln`

## Uso humano

```powershell
.\scripts\skills\verify-pr-protocol\Verify-PR-Protocol.bat
```

## Uso agente

Envelope JSON v2 en **stdin** (`meta` + `request`). Sin stdin: logs por consola (modo TTY).

**Salida JSON:** `success` true y `exitCode` 0 solo si los tres pasos pasan. Fallos: nomenclatura → 1, build → 2, tests → 3.

Definición: `SddIA/skills/verify-pr-protocol/spec.md`. Norma PR: `SddIA/norms/pr-acceptance-protocol.md`.
