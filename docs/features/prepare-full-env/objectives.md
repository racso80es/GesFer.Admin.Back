# Objetivos: Preparar entorno completo (Prepare-FullEnv)

**Feature:** prepare-full-env  
**Persist:** `docs/features/prepare-full-env/`  
**Ley aplicada:** Entorno Windows 11 + PowerShell 7+; no bash. Soberanía documental en Cúmulo.

## Objetivo

Disponer de un único punto de entrada ejecutable (`.bat`/PowerShell) en **paths.toolsPath** (Cúmulo) que prepare todo el entorno de desarrollo: Docker (DB, cache, Adminer), y opcionalmente la API Admin y los clientes indicados en configuración, con documentación y metadatos en formato `.md` y `.json` en la cápsula.

## Alcance

- **Script ejecutable (wrapper):** **paths.toolsPath** + `Prepare-FullEnv.bat` (delega a la cápsula). Fuente: Cúmulo.
- **Cápsula:** **paths.toolCapsules['prepare-full-env']** (Cúmulo). Script principal: `Prepare-FullEnv.ps1` (comprobar Docker, levantar servicios vía `docker-compose`, esperar salud de DB/cache, y opcionalmente levantar API y clientes). Binario Rust opcional en `bin/prepare_full_env.exe`.
- **Configuración:** en la cápsula, `prepare-env.json`: qué servicios levantar (Docker y/o API/clientes), puertos y rutas.
- **Documentación:** en la cápsula, `prepare-env.md` y `manifest.json`.
- **Documentación de la feature:** `docs/features/prepare-full-env/` con objectives.md, spec.md, spec.json.

## Resumen del proceso (fases 0–8)

| Fase | Acción |
|------|--------|
| 0 | Rama feat/prepare-full-env (skill Iniciar-Rama). |
| 1 | Documentación con objetivos (este documento). |
| 2 | Spec técnica en spec.md / spec.json. |
| 3–5 | Clarificación, plan, implementación. |
| 6 | Ejecución: crear scripts y docs. |
| 7 | Validación pre-PR. |
| 8 | Cierre y PR. |

## Referencias

- Cúmulo: `SddIA/agents/cumulo.json` → `paths.featurePath`, `paths.toolsPath`, `paths.toolCapsules`.
- Proceso feature: `SddIA/process/feature.md`.
- Docker actual: `docker-compose.yml` (servicios gesfer-db, cache, adminer; Admin API según contexto repo).
