---
contract_ref: SddIA/tools/tools-contract.md
cumulo_ref: SddIA/agents/cumulo.json
depends_on_tools: []
env:
  - Windows 11
  - API Admin en ejecución (típ. http://localhost:5010)
  - MySQL con seeds si se valida Empresa Demo
implementation_path_ref: paths.toolCapsules.run-test-e2e-local
inputs:
  baseUrl: string (opcional). Por defecto http://localhost:5010 o config.
  configPath: string (opcional). JSON de cápsula o repo.
  runSmoke: boolean (opcional, default true). Health, swagger, login OK, login inválido 401.
  runCompanyRead: boolean (opcional, default true). Lista JWT, secret, GET por id demo, 401 sin auth.
  runCompanyCrud: boolean (opcional, default true). Crear, actualizar, eliminar empresa temporal; GET 404.
  demoCompanyId: string (opcional). UUID empresa demo en seeds.
  demoCompanyName: string (opcional). Nombre para comprobar en lista.
  adminUser: string (opcional). Override; env E2E_ADMIN_USER.
  adminPassword: string (opcional). Preferir env E2E_ADMIN_PASSWORD; no registrar en salida.
  internalSecret: string (opcional). Env E2E_INTERNAL_SECRET.
  outputPath: string (opcional). Escribir JSON envelope a fichero.
  outputJson: boolean (opcional). Emitir envelope por stdout (CLI).
output:
  envelope: capsule-json-io v2 (success, exitCode, message, feedback, result, duration_ms).
  result_fields:
    baseUrl: string
    runSmoke: boolean
    runCompanyRead: boolean
    runCompanyCrud: boolean
    smoke: objeto con healthOk, swaggerOk, loginOk, loginInvalidUnauthorized
    companyRead: objeto con listJwtOk, empresaDemoInList, listSecretOk, getByIdOk, unauthorizedWithoutAuth
    companyCrud: objeto con ok, companyId (id API), steps
toolId: run-test-e2e-local
version: 1.0.0
---

# Especificación: run-test-e2e-local

**toolId:** `run-test-e2e-local`  
**Definición (SddIA):** `paths.toolsDefinitionPath/run-test-e2e-local/`  
**Implementación:** `paths.toolCapsules.run-test-e2e-local` → ejecutable `run_test_e2e_local.exe` (Rust).

## Objetivo

Automatizar las pruebas E2E HTTP validadas manualmente contra la API Admin en local: smoke, lectura de empresas y flujo CRUD con empresa nueva, con salida JSON única conforme al contrato de herramientas.

## Entrada (`request` en envelope)

Ver frontmatter `inputs`. Las credenciales por defecto de desarrollo local pueden sobreescribirse por variables de entorno; no deben aparecer en `message`, `feedback` ni `result`.

## Salida

Envelope v2; el campo `result` agrega las fases ejecutadas y booleanos de éxito por comprobación. Códigos de salida distintos de 0 indican la primera fase fallida (10–13 smoke, 20–21 company read, 30–34 company CRUD).

## Referencias

- `SddIA/norms/capsule-json-io.md`
- `scripts/tools/run-test-e2e-local/run-test-e2e-local.md`
