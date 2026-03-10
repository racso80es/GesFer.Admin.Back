# Plan: Proceso audit-tool

**Tarea:** create-process-audit-tool  
**Fecha:** 2026-03-10  
**Basado en:** spec.md, clarify.md (decisiones resueltas)

---

## Objetivo del plan

Transformar la especificación del proceso `audit-tool` en una hoja de ruta ejecutable. El resultado será:

1. Definición completa del proceso en `SddIA/process/audit-tool/`.
2. Plantillas para informes de auditoría.
3. Caso práctico: auditoría de `start-api`.

---

## Fases y tareas

### Fase 1: Completar definición del proceso

| Tarea | Descripción | Entregable |
|-------|-------------|------------|
| 1.1 | Actualizar `spec.json` del proceso con campos faltantes | `SddIA/process/audit-tool/spec.json` |
| 1.2 | Añadir sección de configuración (cleanup, parámetros) | `SddIA/process/audit-tool/spec.md` |

### Fase 2: Crear plantillas de informe

| Tarea | Descripción | Entregable |
|-------|-------------|------------|
| 2.1 | Crear plantilla `audit-report-template.md` | `SddIA/process/audit-tool/templates/audit-report-template.md` |
| 2.2 | Crear esquema `audit-result-schema.json` | `SddIA/process/audit-tool/templates/audit-result-schema.json` |

### Fase 3: Preparar estructura de auditorías

| Tarea | Descripción | Entregable |
|-------|-------------|------------|
| 3.1 | Crear carpeta `docs/audits/tools/` si no existe | Estructura de carpetas |
| 3.2 | Crear carpeta `docs/audits/tools/start-api/` para caso práctico | Estructura de carpetas |

### Fase 4: Documentar caso práctico (start-api)

| Tarea | Descripción | Entregable |
|-------|-------------|------------|
| 4.1 | Crear `objectives.md` para auditoría de start-api | `docs/audits/tools/start-api/objectives.md` |
| 4.2 | Documentar parámetros de invocación (desde config) | En objectives.md |
| 4.3 | Documentar endpoint de health a validar | En objectives.md |

### Fase 5: Actualizar índices y difusión

| Tarea | Descripción | Entregable |
|-------|-------------|------------|
| 5.1 | Verificar AGENTS.md incluye audit-tool | AGENTS.md (si necesario) |
| 5.2 | Verificar interaction-triggers.md actualizado | Ya completado |

---

## Orden de ejecución

```
1.1 → 1.2 → 2.1 → 2.2 → 3.1 → 3.2 → 4.1 → 4.2 → 4.3 → 5.1 → 5.2
```

Las tareas 2.1 y 2.2 pueden ejecutarse en paralelo.
Las tareas 4.1, 4.2, 4.3 son secuenciales (mismo archivo).

---

## Verificación

| Criterio | Cómo verificar |
|----------|----------------|
| Proceso definido | `SddIA/process/audit-tool/` contiene spec.md y spec.json válidos |
| Plantillas creadas | Carpeta templates/ con archivos de plantilla |
| Estructura de auditorías | `docs/audits/tools/start-api/` existe |
| Caso práctico documentado | objectives.md con criterios específicos de start-api |

---

## Seguridad

- No se ejecutan comandos en esta fase (solo documentación).
- Los parámetros sensibles no se incluyen en plantillas.
- La ejecución del caso práctico queda pendiente para fase de ejecución.

---

## Referencias

- Especificación: `docs/features/create-process-audit-tool/spec.md`
- Clarificaciones: `docs/features/create-process-audit-tool/clarify.md`
- Herramienta objetivo: `scripts/tools/start-api/`
