# Objetivos de Auditoría: start-api

**Proceso:** audit-tool  
**Herramienta:** start-api  
**Fecha:** 2026-03-10  
**Estado:** Pendiente de ejecución

---

## Objetivo

Verificar empíricamente que la herramienta `start-api` funciona correctamente según su especificación y el contrato de herramientas.

---

## Información de la herramienta

| Campo | Valor |
|-------|-------|
| tool-id | `start-api` |
| Cápsula | `scripts/tools/start-api/` |
| Ejecutable | `start_api.exe` |
| Versión | 1.0.0 |

---

## Parámetros de invocación

Extraídos de `start-api-config.json`:

| Parámetro | Valor |
|-----------|-------|
| apiWorkingDir | `src/GesFer.Admin.Back.Api` |
| command | `dotnet run` |
| defaultProfile | `Development` |
| defaultPort | `5010` |
| healthUrl | `http://localhost:5010/health` |
| healthCheckTimeoutSeconds | `30` |

**Nota:** Estos parámetros serán informados al usuario antes de la ejecución.

---

## Criterios de validación

### Validación de ejecución

| Criterio | Descripción |
|----------|-------------|
| E1 | El ejecutable `start_api.exe` existe en la cápsula |
| E2 | El ejecutable se invoca sin errores de sistema |
| E3 | El proceso no termina inmediatamente (API queda activa) |

### Validación de retorno JSON

| Campo | Criterio |
|-------|----------|
| toolId | Igual a `start-api` |
| exitCode | `0` |
| success | `true` |
| timestamp | Formato ISO 8601 válido |
| message | No vacío |
| feedback | Array con al menos una entrada |

### Validación funcional

| Criterio | Descripción |
|----------|-------------|
| F1 | API levantada en puerto 5010 |
| F2 | Health endpoint responde HTTP 200 en `http://localhost:5010/health` |

---

## Configuración de auditoría

| Opción | Valor |
|--------|-------|
| cleanup_after_audit | `true` (detener API tras validación) |
| health_endpoint | `http://localhost:5010/health` (desde config) |
| report_naming | `audit-report-2026-03-10.md`, `audit-result-2026-03-10.json` |

---

## Resultado esperado

- **PASS:** Ejecución exitosa, JSON válido, health endpoint responde HTTP 200.
- **FAIL:** Error en ejecución, JSON inválido, o health no responde.
- **PARTIAL:** Ejecución OK y JSON válido, pero warnings en feedback.

---

## Referencias

- Cápsula: `scripts/tools/start-api/`
- Manifest: `scripts/tools/start-api/manifest.json`
- Config: `scripts/tools/start-api/start-api-config.json`
- Contrato: `SddIA/tools/tools-contract.json`
- Proceso: `SddIA/process/audit-tool/`

---

**Siguiente paso:** Ejecutar la auditoría según el proceso definido.
