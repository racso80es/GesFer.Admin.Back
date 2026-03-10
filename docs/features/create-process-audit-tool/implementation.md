# Implementación: Proceso audit-tool

**Tarea:** create-process-audit-tool  
**Fecha:** 2026-03-10  
**Basado en:** plan.md

---

## Ítems de implementación

### 1.1 – Modificar: spec.json del proceso

- **Acción:** Modificar
- **Ruta:** `SddIA/process/audit-tool/spec.json`
- **Ubicación:** Raíz del objeto JSON
- **Propuesta:**
  - Añadir campo `configuration` con opciones de cleanup y parámetros.
  - Añadir campo `templates_ref` apuntando a plantillas.
- **Dependencias:** Ninguna

### 1.2 – Modificar: spec.md del proceso

- **Acción:** Modificar
- **Ruta:** `SddIA/process/audit-tool/spec.md`
- **Ubicación:** Después de sección "Restricciones"
- **Propuesta:**
  - Añadir sección "Configuración" con opciones de cleanup y health endpoint.
- **Dependencias:** 1.1

### 2.1 – Crear: plantilla de informe MD

- **Acción:** Crear
- **Ruta:** `SddIA/process/audit-tool/templates/audit-report-template.md`
- **Ubicación:** Nuevo archivo
- **Propuesta:**
  - Plantilla con secciones: Resumen, Resultado por fase, Validación JSON, Validación funcional, Evidencias, Recomendaciones.
- **Dependencias:** Ninguna

### 2.2 – Crear: esquema de resultado JSON

- **Acción:** Crear
- **Ruta:** `SddIA/process/audit-tool/templates/audit-result-schema.json`
- **Ubicación:** Nuevo archivo
- **Propuesta:**
  - JSON Schema para audit-result.json con campos: tool_id, audit_date, result, phases_results, json_validation, functional_validation, recommendations.
- **Dependencias:** Ninguna

### 3.1 – Crear: carpeta de auditorías de tools

- **Acción:** Crear
- **Ruta:** `docs/audits/tools/`
- **Ubicación:** Nueva carpeta
- **Propuesta:**
  - Crear estructura de carpetas si no existe.
- **Dependencias:** Ninguna

### 3.2 – Crear: carpeta para auditoría de start-api

- **Acción:** Crear
- **Ruta:** `docs/audits/tools/start-api/`
- **Ubicación:** Nueva carpeta
- **Propuesta:**
  - Carpeta para almacenar informes de auditoría de start-api.
- **Dependencias:** 3.1

### 4.1 – Crear: objectives.md para auditoría de start-api

- **Acción:** Crear
- **Ruta:** `docs/audits/tools/start-api/objectives.md`
- **Ubicación:** Nuevo archivo
- **Propuesta:**
  - Objetivos específicos: ejecución de start_api.exe, validación JSON, health endpoint.
  - Parámetros de invocación (desde start-api-config.json).
  - Endpoint de health a validar.
- **Dependencias:** 3.2

### 5.1 – Verificar: AGENTS.md

- **Acción:** Verificar
- **Ruta:** `AGENTS.md`
- **Ubicación:** Sección "Inicio de tarea (Procesos)"
- **Propuesta:**
  - Verificar si audit-tool debe añadirse a la tabla de procesos en AGENTS.md.
- **Dependencias:** Ninguna

---

## Resumen por archivo

| Archivo | Ítems |
|---------|-------|
| `SddIA/process/audit-tool/spec.json` | 1.1 |
| `SddIA/process/audit-tool/spec.md` | 1.2 |
| `SddIA/process/audit-tool/templates/audit-report-template.md` | 2.1 |
| `SddIA/process/audit-tool/templates/audit-result-schema.json` | 2.2 |
| `docs/audits/tools/` | 3.1 |
| `docs/audits/tools/start-api/` | 3.2 |
| `docs/audits/tools/start-api/objectives.md` | 4.1 |
| `AGENTS.md` | 5.1 |

---

## Orden sugerido

1. Tareas 1.1, 2.1, 2.2, 3.1 pueden ejecutarse en paralelo.
2. Tarea 1.2 después de 1.1.
3. Tarea 3.2 después de 3.1.
4. Tarea 4.1 después de 3.2.
5. Tarea 5.1 al final (verificación).

---

**Estado:** Documento de implementación completo. Pendiente: ejecución.
