# Prepare-FullEnv — Preparar entorno completo

Herramienta para dejar listo el entorno de desarrollo: Docker (MySQL, Memcached, Adminer) y opcionalmente clientes indicados.

## Requisitos

- **Windows 11** con **PowerShell 7+**.
- **Docker Desktop** instalado y en ejecución.
- Opcional: **Node/npm** si se configuran clientes en `prepare-env.json`.

## Uso

### Ejecutable (recomendado)

Desde la **raíz del repositorio** o mediante el launcher en **paths.toolsPath** (Cúmulo):

```powershell
.\scripts\tools\prepare-full-env\Prepare-FullEnv.bat
```

o

```powershell
.\scripts\tools\Prepare-FullEnv.bat
```

El `.bat` usa `prepare_full_env.exe` dentro de la cápsula (build Rust + `scripts/tools-rs/install.ps1`). **El ejecutable debe existir en la raíz de la cápsula** (no se admite `bin\`).

### PowerShell directo

La interfaz soportada por automatizaciones/MCP es el **ejecutable** `.exe` (ver spec SddIA). El wrapper `.bat` es para humanos.

Parámetros opcionales:

| Parámetro        | Descripción                                      |
|------------------|--------------------------------------------------|
| `-DockerOnly`    | Solo levanta Docker (DB, cache, Adminer).        |
| `-NoDocker`      | No levanta Docker (solo fases no-Docker configuradas, p. ej. clientes). |
| `-ConfigPath`    | Ruta al JSON de configuración.                   |
| `-OutputPath`    | Fichero donde escribir el resultado JSON (contrato tools). |
| `-OutputJson`    | Emitir el resultado JSON por stdout al finalizar. |

Ejemplos:

```powershell
.\scripts\tools\prepare-full-env\Prepare-FullEnv.bat -DockerOnly
```

## Configuración: `prepare-env.json`

Ubicación: en esta cápsula, `prepare-env.json`. Ruta canónica (Cúmulo): **paths.toolCapsules['prepare-full-env']** (`SddIA/agents/cumulo.json`).

| Campo                  | Descripción |
|------------------------|-------------|
| `dockerComposePath`    | Ruta al `docker-compose.yml` respecto a la raíz del repo. |
| `mysqlContainerName`   | Nombre del contenedor MySQL para el healthcheck. |
| `dockerServices`       | Lista de servicios a levantar con `docker compose up -d` (o `docker-compose` si aplica). |
| `startClients`         | Array de `{ "name", "workingDir", "command" }` para frontends u otros clientes. |
| `healthCheck.*`        | Reintentos y tiempos de espera para MySQL. |

Si el fichero no existe, se usan valores por defecto (solo Docker: db, cache, adminer).

## Estructura esperada

- **Raíz del repo:** contiene `docker-compose.yml` y la carpeta `src/`.
- **Logs:** si se usa `run-service-with-log.ps1`, los logs se escriben en `logs/services/<ServiceName>.log`.

## Troubleshooting

- **Docker no está corriendo:** iniciar Docker Desktop y volver a ejecutar el script.
- **Puerto 3306 en uso:** detener el proceso que lo use o cambiar el mapeo en `docker-compose.yml`.
- **MySQL tarda en estar listo:** el script espera hasta `mysqlMaxAttempts * mysqlRetrySeconds` segundos; si no basta, revisar `docker-compose logs gesfer-db`.

## Salida JSON (contrato tools)

La herramienta cumple `SddIA/tools/tools-contract.json`. Al finalizar produce un JSON con:

- `toolId`, `exitCode`, `success`, `timestamp`, `message`, `feedback[]`, `result`, `duration_ms`.
- `feedback`: array de eventos por fase (`init`, `docker`, `mysql`, `clients`, `done`) con `phase`, `level` (info|warning|error), `message`, `timestamp`.
- `result`: servicios Docker levantados y clientes iniciados.

Ejemplo de uso con salida a fichero y por stdout:

```powershell
.\scripts\tools\prepare-full-env\Prepare-FullEnv.bat -OutputPath "logs\prepare-env-result.json" -OutputJson
```

## Referencia

- Contrato de herramientas: `SddIA/tools/tools-contract.json`, `SddIA/tools/tools-contract.md`.
- Manifest de la cápsula: `manifest.json` en esta carpeta.
