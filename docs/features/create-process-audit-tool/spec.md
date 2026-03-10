# Especificación: Proceso audit-tool

**Tarea:** create-process-audit-tool  
**Fecha:** 2026-03-10  
**Versión:** 1.0.0

---

## 1. Contexto

Se necesita un proceso formal para auditar herramientas (tools) del ecosistema SddIA. El proceso debe verificar empíricamente que una herramienta:

1. Se ejecuta correctamente (invocación del `.exe` o script fallback).
2. Produce un retorno JSON conforme al contrato de herramientas (`tools-contract.json`).
3. Cumple sus objetivos funcionales declarados (validación directa del resultado).

**Caso práctico inicial:** Auditoría de la herramienta `start-api`, verificando que la API queda levantada y responde en el endpoint de health.

---

## 2. Arquitectura del proceso

### 2.1. Entradas

| Entrada | Tipo | Descripción |
|---------|------|-------------|
| `tool-id` | string | Identificador de la herramienta a auditar (kebab-case). |
| `capsule_path` | path | Ruta de la cápsula: `paths.toolCapsules[<tool-id>]`. |
| `validation_criteria` | object | Criterios específicos de validación funcional (opcional, derivados del manifest). |

### 2.2. Salidas

| Salida | Tipo | Descripción |
|--------|------|-------------|
| `audit-report.md` | file | Informe legible: resultado, evidencias, recomendaciones. |
| `audit-result.json` | file | Resultado machine-readable con estructura definida. |

### 2.3. Ubicaciones (Cúmulo)

- **Documentación de tarea:** `paths.featurePath/audit-tool-<tool-id>/`
- **Informe de auditoría:** `paths.auditsPath/tools/<tool-id>/`

---

## 3. Fases del proceso

### Fase 0: Preparar entorno

- Verificar existencia de la herramienta en `paths.toolCapsules`.
- Verificar artefactos requeridos: manifest.json, .exe (o .ps1 fallback).
- Rama opcional: `feat/audit-tool-<tool-id>`.

### Fase 1: Definir objetivos

- Crear `objectives.md` con criterios de éxito específicos.
- Derivar criterios del `manifest.json` y documentación de la herramienta.

### Fase 2: Analizar especificación

- Leer `manifest.json`: toolId, description, components.
- Leer documentación: `<tool-id>.md`.
- Identificar qué debe validarse funcionalmente.

### Fase 3: Diseñar pruebas

- Definir casos de prueba:
  - Invocación estándar (sin parámetros / con parámetros por defecto).
  - Validación de salida JSON.
  - Validación funcional específica.

### Fase 4: Ejecutar herramienta

- **Informar al usuario** de los parámetros que se van a utilizar (leídos de config).
- Invocar el `.exe` (o script fallback) con parámetros de `<tool-id>-config.json` si existe.
- Capturar salida JSON (stdout o archivo según configuración).
- Registrar tiempo de ejecución y código de salida.

### Fase 5: Validar retorno JSON

Verificar campos requeridos según `tools-contract.json`:

| Campo | Validación |
|-------|------------|
| `toolId` | Coincide con el tool-id auditado |
| `exitCode` | 0 para éxito |
| `success` | true para ejecución correcta |
| `timestamp` | Formato ISO 8601 válido |
| `message` | No vacío |
| `feedback` | Array con al menos una entrada |

### Fase 6: Validar objetivos funcionales

Depende de la herramienta. Para `start-api`:

| Validación | Criterio |
|------------|----------|
| API levantada | Proceso activo en puerto configurado |
| Health endpoint | HTTP 200 en URL de config (`healthEndpoint`) o fallback `http://localhost:<port>/health` |

**Nota:** HTTP 200 es suficiente; no se valida el body.

### Fase 7: Generar informe

- **audit-report.md**: Resumen ejecutivo, resultado por fase, evidencias, recomendaciones.
- **audit-result.json**: Estructura machine-readable.

### Fase 8: Cierre

- Guardar informe en `paths.auditsPath/tools/<tool-id>/` con versionado por fecha:
  - `audit-report-YYYY-MM-DD.md`
  - `audit-result-YYYY-MM-DD.json`
- **Cleanup:** Detener proceso de la herramienta (configurable, default: sí).
- **Evolution Log:** Solo si resultado es FAIL o PARTIAL.

---

## 4. Esquema de audit-result.json

```json
{
  "spec_version": "1.0.0",
  "tool_id": "<tool-id>",
  "audit_date": "ISO 8601",
  "result": "PASS | FAIL | PARTIAL",
  "auditor": "agent | human",
  "phases_results": [
    {
      "phase_id": "0-8",
      "name": "Nombre fase",
      "result": "PASS | FAIL | SKIP",
      "message": "Descripción",
      "evidence": "Referencia o snippet"
    }
  ],
  "json_validation": {
    "valid": true,
    "fields_checked": ["toolId", "exitCode", "success", "..."],
    "errors": []
  },
  "functional_validation": {
    "valid": true,
    "checks": [
      { "name": "API running", "result": "PASS", "detail": "..." }
    ]
  },
  "recommendations": [],
  "duration_ms": 0
}
```

---

## 5. Seguridad

- El proceso opera bajo contexto de **Karma2Token** válido.
- No se registran datos sensibles (contraseñas, tokens) en el informe.
- Los comandos se ejecutan vía skill `invoke-command` (no directamente).

---

## 6. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| CA1 | El proceso puede auditar cualquier tool de paths.toolCapsules | Ejecución con start-api |
| CA2 | Se genera audit-result.json con estructura válida | Parseo JSON sin errores |
| CA3 | Se valida el retorno JSON contra tools-contract | Campos requeridos presentes |
| CA4 | Se valida el objetivo funcional declarado | Para start-api: health OK |
| CA5 | El informe es reproducible | Documentar comandos ejecutados |

---

## Referencias

- Contrato de herramientas: `SddIA/tools/tools-contract.json`
- Contrato de procesos: `SddIA/process/process-contract.json`
- Cúmulo: `SddIA/agents/cumulo.paths.json`
- Herramienta objetivo: `scripts/tools/start-api/`
