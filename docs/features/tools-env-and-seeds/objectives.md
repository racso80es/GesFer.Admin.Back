# Objetivos: Herramientas entorno y seeds (tools-env-and-seeds)

**Rama:** feat/tools-env-and-seeds  
**Persist:** `docs/features/tools-env-and-seeds/`  
**Ley aplicada:** Entorno Windows + PowerShell; contrato tools (SddIA/tools/).

## Objetivo

Disponer de herramientas ejecutables en `scripts/tools/` que preparen el entorno (Docker, API/clientes) y que ejecuten MySQL con migraciones y seeds adecuados, con salida JSON y feedback según el contrato de tools.

## Entregables

1. **Prepare-FullEnv:** Docker (DB, cache, Adminer), opcionalmente API y clientes. Contrato: salida JSON y feedback por fases.
2. **Contrato tools:** `SddIA/tools/tools-contract.json` y `.md` — salida JSON obligatoria y feedback adecuado para todas las herramientas.
3. **Invoke-MySqlSeeds:** Comprueba MySQL, aplica `dotnet ef database update` y ejecuta seeds de Admin (RUN_SEEDS_ONLY en la API). Config en `mysql-seeds-config.json`, documentación en `mysql-seeds.md`.

## Referencias

- Prepare-FullEnv: `docs/features/prepare-full-env/`, `scripts/tools/Prepare-FullEnv.ps1`, `prepare-env.md`.
- Contrato: `SddIA/tools/tools-contract.json`.
- MySQL/seeds: `scripts/tools/Invoke-MySqlSeeds.ps1`, `mysql-seeds.md`, `src/Infrastructure/Data/Seeds/`.
