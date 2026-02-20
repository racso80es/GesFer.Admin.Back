# Especificación: invoke-mysql-seeds

**toolId:** `invoke-mysql-seeds`  
**Definición (SddIA):** Este directorio.  
**Implementación (scripts):** Ruta canónica en Cúmulo → **implementation_path_ref:** `paths.toolCapsules.invoke-mysql-seeds` (consultar `SddIA/agents/cumulo.json`). La raíz del path de implementación la indica Cúmulo.

## Objetivo

Herramienta que comprueba la disponibilidad de MySQL, aplica migraciones EF Core y ejecuta los seeds de Admin (companies, admin-users) mediante la variable de entorno RUN_SEEDS_ONLY=1 en la API.

## Entradas

| Parámetro | Tipo | Descripción |
|----------|------|-------------|
| SkipMigrations | switch | No ejecutar `dotnet ef database update`; solo seeds. |
| SkipSeeds | switch | Solo ejecutar migraciones; no ejecutar seeds. |
| ConfigPath | string | Ruta al JSON de configuración (por defecto en la implementación). |
| OutputPath | string | Fichero donde escribir el resultado JSON (contrato). |
| OutputJson | switch | Emitir el resultado JSON por stdout. |

## Salida

Cumple `SddIA/tools/tools-contract.json`: objeto JSON con toolId, exitCode, success, timestamp, message, feedback[], data (mysql, migrations, seeds), duration_ms.

## Fases (feedback)

init → mysql → migrations → seeds → done (o error).

## Implementación

La implementación técnica (scripts PowerShell, .bat, config, manifest, opcional binario Rust) reside en la carpeta cuya ruta se obtiene de Cúmulo mediante **implementation_path_ref**. No se duplica la ruta aquí; la raíz del path la indica Cúmulo.
