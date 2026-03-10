# Informe de Auditoría: start-api

**Fecha:** 2026-03-10  
**Versión herramienta:** 1.0.0  
**Auditor:** agent

---

## Resumen ejecutivo

| Campo | Valor |
|-------|-------|
| Herramienta | start-api |
| Resultado | **PARTIAL** |
| Duración auditoría | 42352 ms |
| Duración herramienta | 34028 ms |

---

## Resultado por fase

| Fase | Nombre | Resultado | Observaciones |
|------|--------|-----------|---------------|
| 0 | Preparar entorno | PASS | Ejecutable y archivos requeridos presentes |
| 1 | Definir objetivos | PASS | Objetivos documentados |
| 2 | Analizar especificación | PASS | Manifest y config válidos |
| 3 | Diseñar pruebas | PASS | Casos de prueba definidos |
| 4 | Ejecutar herramienta | PASS | Ejecución completada con exitCode 7 |
| 5 | Validar retorno JSON | PASS | Estructura válida según contrato |
| 6 | Validar objetivos funcionales | PASS | API levantada y health responde HTTP 200 |
| 7 | Generar informe | PASS | Este documento |
| 8 | Cierre | PENDING | Cleanup pendiente |

---

## Validación de retorno JSON

| Campo | Esperado | Encontrado | Válido |
|-------|----------|------------|--------|
| toolId | start-api | start-api | ✅ |
| exitCode | 0 | 7 | ⚠️ No es 0, coherente con success:false |
| success | true | false | ⚠️ Herramienta reportó fallo (timeout health) |
| timestamp | ISO 8601 | 2026-03-10T18:43:37.498805500+00:00 | ✅ |
| message | No vacío | "Health no respondió a tiempo" | ✅ |
| feedback | Array | 15 entradas | ✅ |
| data | Opcional | Presente (healthy, pid, port, profile, url_base) | ✅ |
| duration_ms | Opcional | 34028 | ✅ |

**Resultado validación JSON: PASS** — Todos los campos requeridos presentes y válidos.

---

## Validación funcional

### API levantada

- **Criterio:** Proceso activo en puerto configurado
- **Resultado:** PASS
- **Detalle:** PID 1928, puerto 5010

### Health endpoint responde HTTP 200

- **Criterio:** HTTP 200 en `http://localhost:5010/health`
- **Resultado:** PASS
- **Detalle:** StatusCode 200, Content: "Healthy"

**Resultado validación funcional: PASS** — Objetivos funcionales cumplidos.

---

## Evidencias

### Comando ejecutado

```powershell
& "c:\Proyectos\GesFer.Admin.Back\scripts\tools\start-api\start_api.exe" --output-json
```

### Parámetros utilizados

Extraídos de `start-api-config.json`:

```json
{
  "apiWorkingDir": "src/GesFer.Admin.Back.Api",
  "command": "dotnet run",
  "defaultProfile": "Development",
  "defaultPort": 5010,
  "healthUrl": "http://localhost:5010/health",
  "healthCheckTimeoutSeconds": 30
}
```

### Salida JSON capturada

```json
{
  "toolId": "start-api",
  "exitCode": 7,
  "success": false,
  "timestamp": "2026-03-10T18:43:37.498805500+00:00",
  "message": "Health no respondió a tiempo",
  "feedback": [
    {"phase": "init", "level": "info", "message": "Iniciando start-api (Rust)", "timestamp": "2026-03-10T18:43:03.470462600+00:00"},
    {"phase": "port-check", "level": "info", "message": "Comprobando puerto 5010...", "timestamp": "2026-03-10T18:43:03.470686800+00:00"},
    {"phase": "port-check", "level": "info", "message": "Puerto libre.", "timestamp": "2026-03-10T18:43:03.471901900+00:00"},
    {"phase": "build", "level": "info", "message": "Compilando proyecto...", "timestamp": "2026-03-10T18:43:03.471945200+00:00"},
    {"phase": "build", "level": "info", "message": "Build OK", "timestamp": "2026-03-10T18:43:07.123818100+00:00"},
    {"phase": "launch", "level": "info", "message": "Levantando API en C:\\Proyectos\\GesFer.Admin.Back\\src/GesFer.Admin.Back.Api (Profile: Development, Port: 5010)", "timestamp": "2026-03-10T18:43:07.123828600+00:00"},
    {"phase": "launch", "level": "info", "message": "API iniciada con PID 1928", "timestamp": "2026-03-10T18:43:07.127010+00:00"},
    {"phase": "healthcheck", "level": "warning", "message": "Timeout salud; API arrancada (PID 1928). Compruebe http://localhost:5010/health", "timestamp": "2026-03-10T18:43:37.498802200+00:00"}
  ],
  "data": {
    "healthy": false,
    "pid": 1928,
    "port": 5010,
    "profile": "Development",
    "url_base": "http://localhost:5010/health"
  },
  "duration_ms": 34028
}
```

### Verificación manual del health endpoint

```
StatusCode: 200
Content: Healthy
```

---

## Recomendaciones

1. **Timeout de health check:** La herramienta reportó timeout (30s), pero el health endpoint responde correctamente tras la auditoría. Considerar aumentar el timeout a 45s o implementar reintentos con backoff exponencial.

2. **Discrepancia success:false:** La herramienta marcó `success: false` debido al timeout del health check, pero funcionalmente la API se levantó y responde correctamente. El contrato de la herramienta es estricto: solo marca success si el health responde dentro del timeout configurado. Esto es correcto según la especificación de la herramienta.

3. **Coherencia:** El retorno JSON es coherente con el comportamiento: `exitCode: 7`, `success: false`, feedback con warning de timeout, y `data.healthy: false`.

---

## Referencias

- Herramienta: `scripts/tools/start-api/`
- Contrato: `SddIA/tools/tools-contract.json`
- Proceso: `SddIA/process/audit-tool/`
- Salida raw: `docs/audits/tools/start-api/tool-output-raw.json`

---

*Informe generado por proceso audit-tool. Versión del proceso: 1.0.0*
