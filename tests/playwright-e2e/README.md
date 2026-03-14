# Tests E2E Playwright - Login GesFer.Admin.Back

Tests E2E con Playwright que garantizan el endpoint de login:

1. **Rechazar logins incorrectos** — Devuelve 401 con credenciales inválidas
2. **Rechazar credenciales vacías** — Devuelve 400
3. **Aceptar login correcto** — admin/admin123 devuelve 200 y token JWT
4. **Registro de AuditLogs** — LoginSuccess y LoginFailed se persisten en la base de datos

## Requisitos previos

- Node.js 18+
- API en ejecución (puerto 5010 por defecto)

## Levantar la API

Antes de ejecutar los tests, la API debe estar corriendo. Opciones:

```powershell
# Opción 1: Docker (MySQL + API)
docker-compose up -d gesfer-db cache
# Ejecutar migraciones y seeds, luego la API
dotnet run --project src/GesFer.Admin.Back.Api/GesFer.Admin.Back.Api.csproj

# Opción 2: Usar la tool start-api (Cúmulo)
.\scripts\tools\start-api\Start-Api.bat
```

## Ejecución

```powershell
cd tests/playwright-e2e
npm install
npm test
```

### Variables de entorno

| Variable   | Descripción                    | Por defecto           |
|-----------|--------------------------------|------------------------|
| `BASE_URL`| URL base de la API             | `http://localhost:5010` |

```powershell
$env:BASE_URL = "http://localhost:5010"
npm test
```

## Estructura

```
tests/playwright-e2e/
├── package.json
├── playwright.config.ts
├── tsconfig.json
├── README.md
└── tests/
    └── login.spec.ts
```
