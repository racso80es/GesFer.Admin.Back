# Objetivos: Herramientas entorno y seeds (tools-env-and-seeds)

**Rama:** feat/tools-env-and-seeds  
**Persist:** `docs/features/tools-env-and-seeds/`  
**Ley aplicada:** Entorno Windows + PowerShell; contrato tools (SddIA/tools/).

## Objetivo

Disponer de herramientas ejecutables en **paths.toolsPath** y en las cápsulas **paths.toolCapsules** (Cúmulo) que preparen el entorno (Docker, API/clientes) y que ejecuten MySQL con migraciones y seeds adecuados, con salida JSON y feedback según el contrato de tools.

## Entregables

1. **Prepare-FullEnv:** Docker (DB, cache, Adminer), opcionalmente API y clientes. Contrato: salida JSON y feedback por fases.
2. **Contrato tools:** `SddIA/tools/tools-contract.json` y `.md` — salida JSON obligatoria y feedback adecuado para todas las herramientas.
3. **Invoke-MySqlSeeds:** Comprueba MySQL, aplica `dotnet ef database update` y ejecuta seeds de Admin (RUN_SEEDS_ONLY en la API). Cápsula **paths.toolCapsules['invoke-mysql-seeds']** (Cúmulo): config `mysql-seeds-config.json`, documentación `mysql-seeds.md`.

## Referencias

- Prepare-FullEnv: **paths.featurePath** prepare-full-env; cápsula **paths.toolCapsules['prepare-full-env']** (Prepare-FullEnv.ps1, prepare-env.md, manifest.json). Launcher wrapper: **paths.toolsPath** + Prepare-FullEnv.bat. Fuente: Cúmulo.
- Contrato: `SddIA/tools/tools-contract.json`.
- MySQL/seeds: cápsula **paths.toolCapsules['invoke-mysql-seeds']** (Invoke-MySqlSeeds.ps1, mysql-seeds.md, mysql-seeds-config.json, manifest.json). Launcher wrapper: **paths.toolsPath** + Invoke-MySqlSeeds.bat. Seeds: `src/Infrastructure/Data/Seeds/`. Fuente: Cúmulo.
