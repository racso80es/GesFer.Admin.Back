# Especificación técnica: Prepare-FullEnv

**Feature:** prepare-full-env  
**Ubicación scripts:** `scripts/tools/`

## 1. Entrada

- Configuración en `scripts/tools/prepare-env.json` (servicios a levantar, rutas, puertos).
- Parámetros opcionales por línea de comandos (por ejemplo `-DockerOnly`, `-StartApi`, `-StartClients`).

## 2. Componentes

### 2.1 Ejecutable de entrada

- **`Prepare-FullEnv.bat`**: script batch que invoca PowerShell 7+ con el script `.ps1` desde la raíz del repositorio, con política de ejecución adecuada. Debe poder ejecutarse con doble clic o desde terminal.

### 2.2 Script principal PowerShell

- **`Prepare-FullEnv.ps1`**:
  1. Comprobar que Docker está en ejecución (`docker info`).
  2. Resolver ruta raíz del repo (por encima de `scripts/tools/`).
  3. Cargar opciones desde `prepare-env.json` (con valores por defecto si no existe).
  4. Levantar servicios Docker indicados (`docker-compose up -d` para los servicios configurados, p. ej. `gesfer-db`, `cache`, `adminer`).
  5. Esperar a que MySQL esté listo (healthcheck o `mysqladmin ping` contra el contenedor correcto, p. ej. `gesfer_db`).
  6. Opcionalmente: levantar Admin API en local (`dotnet run` en el directorio de la API) y/o clientes indicados en el JSON, usando si existe `scripts/run-service-with-log.ps1` para logs.
  7. Mostrar resumen de URLs y estado.

### 2.3 Configuración JSON

- **`prepare-env.json`** (machine-readable):
  - `dockerServices`: lista de servicios de docker-compose a levantar (o `"default"` = db, cache, adminer).
  - `startApi`: booleano o objeto con `enabled`, `workingDir`, `command` (ej. `dotnet run`).
  - `startClients`: array de entradas con `name`, `workingDir`, `command` (ej. npm run dev).
  - `dockerComposePath`: ruta relativa al repo del `docker-compose.yml`.
  - `mysqlContainerName`: nombre del contenedor MySQL para el healthcheck.

### 2.4 Documentación

- **`prepare-env.md`**: descripción del objetivo, requisitos (Docker Desktop, .NET SDK, Node si aplica), uso del `.bat` y del `.ps1`, parámetros, estructura del JSON y troubleshooting.

## 3. Salida

- Entorno listo: Docker corriendo, DB accesible, opcionalmente API y clientes en ejecución.
- Logs de servicios en `logs/services/` si se usa `run-service-with-log.ps1`.

## 4. Restricciones

- Solo PowerShell / Batch; sin bash.
- Rutas relativas a la raíz del repositorio.
- No hacer commit en `master`; trabajo en rama `feat/prepare-full-env`.
