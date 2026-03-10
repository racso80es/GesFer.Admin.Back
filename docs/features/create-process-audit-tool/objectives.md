# Objetivos: Creación del proceso audit-tool

**Proceso:** feature (creación de proceso)  
**Tarea:** create-process-audit-tool  
**Fecha:** 2026-03-10  
**Rama:** feat/create-process-audit-tool (pendiente de crear con skill iniciar-rama)

---

## Objetivo principal

Definir un proceso que contemple y establezca el circuito necesario para la **realización de auditorías de herramientas**. El proceso debe permitir verificar empíricamente el correcto funcionamiento de una herramienta dada.

## Descripción

Se desarrolla con un **caso práctico**: auditoría de la herramienta **start-api**.

### Resultado deseado

Garantizar que la herramienta cumple con sus objetivos:
- La API se encuentra levantada.
- Respuesta correcta en endpoint de health.

### Funcionalidad buscada

- Poder solicitar la auditoría de una herramienta.
- Obtener un informe con el resultado obtenido.

---

## Criterios de éxito

| ID | Criterio | Descripción |
|:---|:---------|:------------|
| C1 | Ejecución con .exe | La herramienta se lanza mediante su ejecutable (`start_api.exe`). |
| C2 | Retorno JSON válido | Se contempla y valida el retorno JSON según el contrato de herramientas. |
| C3 | Validación directa | Se verifica el objetivo funcional (API levantada, health OK). |
| C4 | Informe generado | Se produce un informe legible (.md) y machine-readable (.json). |

---

## Entregables

1. **Definición del proceso** en `SddIA/process/audit-tool/`:
   - `spec.md` — Especificación legible del proceso.
   - `spec.json` — Metadatos machine-readable.

2. **Actualización de índices**:
   - `SddIA/process/README.md` — Incluir nuevo proceso.
   - Actualizar `.cursor/rules/process-suggestions.mdc` (difusión SddIA).
   - Actualizar `SddIA/norms/interaction-triggers.md` (listado canónico de procesos).

3. **Caso práctico (auditoría de start-api)**:
   - Ejecución del proceso audit-tool con start-api como objetivo.
   - Informe en `docs/audits/tools/start-api/`.

---

## Restricciones

- Entorno: Windows 11 + PowerShell 7+.
- La ejecución de la herramienta debe ser mediante el .exe cuando exista.
- No se ejecutan comandos directamente; toda ejecución vía skill, tool o proceso.
- El informe debe ser reproducible (documentar comandos/invocaciones).

---

## Referencias

- Contrato de procesos: `SddIA/process/process-contract.json`
- Contrato de herramientas: `SddIA/tools/tools-contract.json`
- Cúmulo: `SddIA/agents/cumulo.paths.json` (paths.auditsPath, paths.toolCapsules)
- Herramienta objetivo: `scripts/tools/start-api/`
