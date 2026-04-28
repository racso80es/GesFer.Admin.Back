# Especificación técnica: Prepare-FullEnv

**Feature:** prepare-full-env  
**Ubicación (cápsula):** **paths.toolCapsules['prepare-full-env']** (Cúmulo, `SddIA/agents/cumulo.json`).

## 1. Entrada

- Configuración en la cápsula: `prepare-env.json` (servicios a levantar, rutas, puertos).
- Parámetros opcionales por línea de comandos (por ejemplo `-DockerOnly`, `-NoDocker`, `-ConfigPath`, `-OutputPath`, `-OutputJson`).

## 2. Componentes

### 2.1 Ejecutable de entrada

- **`Prepare-FullEnv.bat`**: wrapper humano en la cápsula. Invoca `prepare_full_env.exe` (Rust) en la **raíz** de la cápsula.

### 2.2 Ejecutable Rust (contrato tools v2)

- **prepare_full_env.exe** (en la cápsula **paths.toolCapsules['prepare-full-env']**):
  1. Cargar opciones desde `prepare-env.json` en la cápsula (con valores por defecto si no existe).
  2. Levantar servicios Docker indicados (`docker compose up -d` para los servicios configurados).
  3. Esperar a que MySQL esté listo (healthcheck si existe; si no, contenedor “running”).
  4. Opcionalmente: iniciar clientes indicados en el JSON.
  5. Emitir JSON de salida (envelope v2) a stdout y/o a fichero.

### 2.3 Configuración JSON

- **prepare-env.json** (en la cápsula; machine-readable):
  - `dockerServices`: lista de servicios de docker-compose a levantar (o `"default"` = db, cache, adminer).
  - `startClients`: array de entradas con `name`, `workingDir`, `command` (ej. npm run dev).
  - `dockerComposePath`: ruta relativa al repo del `docker-compose.yml`.
  - `mysqlContainerName`: nombre del contenedor MySQL para el healthcheck.

### 2.4 Documentación

- **prepare-env.md** (en la cápsula): descripción del objetivo, requisitos (Docker Desktop, .NET SDK, Node si aplica), uso del `.bat` y del `.ps1`, parámetros, estructura del JSON y troubleshooting. La cápsula incluye además `manifest.json` (toolId, components, contract_ref).

## 3. Salida

- Entorno listo: Docker corriendo, DB accesible y opcionalmente clientes en ejecución.
- Logs de servicios en `logs/services/` si se usa `run-service-with-log.ps1`.

## 4. Restricciones

- Solo PowerShell / Batch; sin bash.
- Rutas relativas a la raíz del repositorio.
- No hacer commit en `master`; trabajo en rama `feat/prepare-full-env`.
