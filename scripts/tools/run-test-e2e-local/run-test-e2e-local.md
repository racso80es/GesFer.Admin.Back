# Herramienta: run-test-e2e-local

**toolId:** `run-test-e2e-local`  
**Contrato:** `SddIA/tools/tools-contract.md`, envelope `SddIA/norms/capsule-json-io.md`.

## Propósito

Ejecutar de forma recurrente las pruebas E2E manuales validadas contra la API Admin en local: salud y documentación OpenAPI, autenticación admin, listado y consulta de empresas (JWT y `X-Internal-Secret`), y un flujo completo crear → actualizar → eliminar → verificar 404 sobre una empresa nueva (sin dejar datos activos).

## Requisitos

- API escuchando en la URL base configurada (por defecto `http://localhost:5010`).
- Base de datos con seeds esperados si se ejecutan las fases que comprueban **Empresa Demo** (id y nombre en config).
- Variables de entorno opcionales: `E2E_ADMIN_USER`, `E2E_ADMIN_PASSWORD`, `E2E_INTERNAL_SECRET` (alineadas con `appsettings.Development.json` y uso local).

## Invocación

| Actor | Método |
|-------|--------|
| **IA / agente** | `run_test_e2e_local.exe` con JSON en stdin, o variable `GESFER_CAPSULE_REQUEST` con el envelope completo. |
| **Humano** | `Run-Test-E2E-Local.bat` desde la raíz del repo (establece `GESFER_REPO_ROOT`). |

Compilación y copia del `.exe` a esta cápsula: `scripts/tools-rs/install.ps1`.

## Salida

Un único JSON en stdout (envelope v2): `success`, `exitCode`, `message`, `feedback[]`, `result` con:

- `baseUrl`, flags ejecutados (`runSmoke`, `runCompanyRead`, `runCompanyCrud`).
- `smoke`: `healthOk`, `swaggerOk`, `loginOk`, `loginInvalidUnauthorized`.
- `companyRead`: `listJwtOk`, `empresaDemoInList`, `listSecretOk`, `getByIdOk`, `unauthorizedWithoutAuth`.
- `companyCrud`: `ok`, `companyId` (solo id devuelto por la API; no es secreto), `steps`.

No se incluyen contraseñas ni tokens en la salida.

## Parámetros (`request` del envelope o CLI)

| Campo | Descripción |
|-------|-------------|
| `baseUrl` | URL base de la API. |
| `configPath` | Ruta al JSON de configuración (opcional). |
| `runSmoke` | Ejecutar smoke (default true). |
| `runCompanyRead` | Ejecutar lecturas company (default true). |
| `runCompanyCrud` | Ejecutar CRUD de prueba (default true). |
| `demoCompanyId` / `demoCompanyName` | Override de seeds esperados para company read. |

CLI: `run_test_e2e_local.exe --base-url http://localhost:5010 --output-json`. Fases: `--run-smoke true|false`, `--run-company-read true|false`, `--run-company-crud true|false` (valores `true`/`false`/`1`/`0`). Ver `--help`.
