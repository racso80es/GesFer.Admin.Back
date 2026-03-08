# Especificación: start-api

**toolId:** `start-api`  
**Definición (SddIA):** Este directorio.  
**Implementación (scripts):** Ruta canónica en Cúmulo → **implementation_path_ref:** `paths.toolCapsules.start-api` (consultar `SddIA/agents/cumulo.json`). La raíz del path de implementación la indica Cúmulo.

## Objetivo

Herramienta que **levanta la API** del proyecto (GesFer.Admin.Back): compila si es necesario, inicia el host (dotnet run / Kestrel) y opcionalmente comprueba salud. Completa el ciclo de vida junto con **prepare-full-env** (infraestructura) e **invoke-mysql-seeds** (datos), permitiendo la gestión de la solución por terceros (agentes, CI/CD, orquestadores).

## Entradas

| Parámetro     | Tipo   | Descripción |
|---------------|--------|-------------|
| NoBuild       | switch | No compilar; solo ejecutar si ya hay build. |
| Profile       | string | Perfil de ejecución (ej. Development). Por defecto según config. |
| Port          | number | Puerto del host (override). |
| PortBlocked   | enum   | Comportamiento si el puerto está ocupado: **fail** (valor por defecto) — fallar con error; **kill** — intentar cerrar el proceso que usa el puerto y continuar. |
| ConfigPath    | string | Ruta al JSON de configuración (por defecto en la implementación). |
| OutputPath    | string | Fichero donde escribir el resultado JSON (contrato). |
| OutputJson    | switch | Emitir el resultado JSON por stdout. |

## Validación de éxito

La herramienta considera la ejecución **correcta** si y solo si el endpoint **health** responde adecuadamente (HTTP 200 en la URL configurada, p. ej. `http://localhost:<port>/health`). Si el healthcheck no responde en el tiempo configurado, la herramienta devuelve error.

## Puerto ocupado

Antes de levantar la API se comprueba si el puerto está en uso. Si está ocupado:

- **PortBlocked=fail:** se emite error y se termina con exitCode distinto de 0.
- **PortBlocked=kill:** se intenta identificar y cerrar el proceso que usa el puerto (en Windows: netstat + taskkill); tras liberar el puerto se continúa con el arranque.

## Salida

Cumple `SddIA/tools/tools-contract.json`: objeto JSON con toolId, exitCode, success, timestamp, message, feedback[], data (url_base, pid, port, healthy), duration_ms.

## Fases (feedback)

init → port-check → [port-kill si aplica] → build (opcional) → launch → healthcheck → done | error.

## Dependencias lógicas

Recomendado tener ejecutadas antes **prepare-full-env** e **invoke-mysql-seeds** (infra y datos listos). No bloqueante si el invocador ya las ejecutó.

## Implementación

**Obligatoria en Rust.** La implementación reside en paths.toolsRustPath (binario `start_api.exe`), copiado a la cápsula en `bin/`. Launcher .bat/.ps1 en la cápsula invoca el .exe en `bin/` si existe; si no, fallback a .ps1. La raíz del path de implementación la indica Cúmulo.
