# Especificación: create-tool-start-api

**Proceso:** create-tool  
**Rama:** feat/create-tool-start-api  
**Persistencia:** paths.featurePath/create-tool-start-api/ (Cúmulo)

## 1. Resumen

Crear la herramienta **start-api** que permita levantar la API del proyecto (GesFer.Admin.Back) de forma programática, con salida JSON según tools-contract y ejecutable por terceros (agentes, CI/CD, orquestadores). Implementación por defecto en Rust; cápsula en paths.toolCapsules.start-api.

## 2. Alcance

- **Definición (SddIA):** paths.toolsDefinitionPath/start-api/ (spec.md, spec.json).
- **Cápsula:** paths.toolCapsules.start-api (paths.toolsPath/start-api/): manifest.json, start-api-config.json, start-api.md, launcher .bat/.ps1, opcional binario en bin/.
- **Implementación Rust:** paths.toolsRustPath (Cúmulo), binario copiado a cápsula/bin/.
- **Índice y Cúmulo:** scripts/tools/index.json y SddIA/agents/cumulo.paths.json (toolCapsules.start-api).

## 3. Contrato de la herramienta start-api

### 3.1 Identificación

- **toolId:** start-api
- **contract_ref:** SddIA/tools/tools-contract.json
- **implementation_path_ref:** paths.toolCapsules.start-api

### 3.2 Descripción

Herramienta que levanta la API del proyecto (Admin Back): compila si es necesario, inicia el host (dotnet run / Kestrel) y opcionalmente comprueba salud. Pensada para ejecución por terceros y para completar el ciclo infra (prepare-full-env) → datos (invoke-mysql-seeds) → API (start-api).

### 3.3 Entradas

| Parámetro   | Tipo    | Descripción |
|------------|---------|-------------|
| NoBuild    | boolean | (opcional) No compilar; solo ejecutar si ya hay build. |
| Profile    | string  | (opcional) Perfil de ejecución (ej. Development). Por defecto según config. |
| Port       | number  | (opcional) Puerto del host (override). |
| PortBlocked| fail \| kill | (opcional) Si el puerto está ocupado: **fail** = error y salir (por defecto); **kill** = cerrar el proceso que usa el puerto y continuar. |
| ConfigPath | string  | (opcional) Ruta al JSON de configuración. |
| OutputPath | string  | (opcional) Fichero donde escribir el resultado JSON. |
| OutputJson | boolean | (opcional) Emitir resultado JSON por stdout. |

### 3.4 Validación de éxito y puerto

- **Éxito:** La herramienta considera la ejecución correcta **solo si el endpoint health responde adecuadamente** (HTTP 200 en la URL configurada).
- **Puerto ocupado:** Se valida si el puerto está ocupado antes de arrancar. Comportamiento según **PortBlocked** (fail o kill).

### 3.5 Salida

Conforme a tools-contract.json: toolId, exitCode, success, timestamp, message, feedback[], data (url_base, pid, port, healthy), duration_ms.

### 3.6 Fases (feedback)

init → port-check → [port-kill si aplica] → build (opcional) → launch → healthcheck → done | error.

### 3.7 Dependencias lógicas

- **depends_on_tools:** ["prepare-full-env", "invoke-mysql-seeds"] — recomendado que infra y datos estén listos; no bloqueante si el invocador ya los ejecutó.

### 3.8 Entorno

Windows 11, PowerShell 7+, .NET SDK 8. Opcional Docker/MySQL si se asume que prepare-full-env e invoke-mysql-seeds ya se ejecutaron.

## 4. Interfaz de proceso

Esta tarea genera al menos un .md y un .json en la carpeta de la tarea (objectives.md, spec.md, spec.json). El entregable ejecutable es la cápsula en paths.toolCapsules.start-api.

## 5. Referencias

- Proceso: SddIA/process/create-tool/spec.md, spec.json
- Contrato tools: SddIA/tools/tools-contract.json
- Cúmulo: SddIA/agents/cumulo.json, cumulo.paths.json
- Herramientas de referencia: prepare-full-env, invoke-mysql-seeds (spec en SddIA/tools/).
