---
entity_kind: sddia_evolution_contract
contrato_version: 1.1
schema_version: "1.1"
contract_ref: SddIA/agents/cumulo.paths.json
persist_paths:
  index: paths.sddiaEvolutionPath + paths.sddiaEvolutionLogFile
  detail: paths.sddiaEvolutionPath + "{id_cambio}.md"
norm_ref: SddIA/norms/sddia-evolution-sync.md
plan_ref: docs/features/sddia-evolution-sync-norma/plan.md
---

# Contrato de Cambio SddIA (YAML v1.1)

Documento técnico de referencia para **replicación** entre entornos SSDD IA y lectura por otras IAs. Cada intervención en el protocolo de actuación bajo `./SddIA/` debe materializarse como:

1. **Índice maestro:** `Evolution_log.md` (ruta vía Cúmulo: `paths.sddiaEvolutionPath` + `paths.sddiaEvolutionLogFile`).
2. **Detalle atómico:** un fichero `{id_cambio}.md` en la misma carpeta, con **frontmatter YAML** obligatorio (contrato v1.1) + cuerpo Markdown (resumen humano).

## Identificador `id_cambio`

- **Formato:** **UUID versión 4** (RFC 4122), string canónico con guiones en minúsculas (ej. `a1b2c3d4-e5f6-47a8-9abc-def012345678`).
- **Nombre de fichero:** exactamente `{id_cambio}.md` (el GUID evita colisiones entre intervenciones en el mismo minuto).
- **Histórico:** el formato legible `SSDD-LOG-YYYYMMDD-HHMM` queda **obsoleto** como id principal; puede usarse como columna opcional de presentación si una herramienta lo genera, pero no sustituye al GUID.

## Campos obligatorios (frontmatter)

| Campo | Tipo | Descripción |
| :--- | :--- | :--- |
| `contrato_version` | string | Versión del esquema; valor actual `1.1`. |
| `id_cambio` | string | UUID v4 (único). |
| `fecha` | string (ISO date) | Fecha del cambio (`YYYY-MM-DD`). |
| `autor` | string | Quién aplica el cambio (p. ej. `Cursor/Agente`, `Jules`, `Humano`). |
| `proyecto_origen_cambio` | string | Proyecto o instancia origen. |
| `contexto` | string | Ámbito (p. ej. `SSDD_IA_CORE`). |
| `descripcion_breve` | string | Una línea, apta para el índice. |
| `tipo_operacion` | string | `alta` \| `baja` \| `modificacion` |
| `cambios_realizados` | lista de objetos | Cada ítem: `anterior` y `nuevo` (strings). |
| `impacto` | string | Uno de: `Bajo`, `Medio`, `Alto`. |
| `replicacion` | objeto | `instrucciones` (string), `hash_integridad` (SHA-256 del bloque YAML canónico o `SHA-256-PENDIENTE` hasta generar). |

## Campos condicionales (baja)

Si `tipo_operacion` es `baja`:

| Campo | Tipo | Descripción |
| :--- | :--- | :--- |
| `rutas_eliminadas` | lista de strings | Paths relativos al repo que dejan de existir. |
| `commit_referencia_previo` | string | SHA corto (7–40 hex) o identificador del commit donde el path aún existía (para auditoría). |

## Ejemplo mínimo

```yaml
---
contrato_version: 1.1
id_cambio: "a1b2c3d4-e5f6-47a8-9abc-def012345678"
fecha: 2026-03-27
autor: Cursor/Tekton
proyecto_origen_cambio: GesFer.Admin.Back
contexto: SSDD_IA_CORE
descripcion_breve: "Ejemplo: registro con GUID"
tipo_operacion: modificacion
cambios_realizados:
  - anterior: "contrato v1.0 solo"
    nuevo: "contrato v1.1 con GUID y tipo_operacion"
impacto: Bajo
replicacion:
  instrucciones: "Copiar paths.sddiaEvolutionPath y fusionar Evolution_log.md"
  hash_integridad: "SHA-256-PENDIENTE"
---
```

## Integridad

- El campo `hash_integridad` debe ser el **SHA-256** (hex minúsculas) del bloque YAML del frontmatter **sin** el delimitador final `---` de cierre, normalizado a UTF-8, o la cadena literal `SHA-256-PENDIENTE` hasta el primer cálculo.
- El binario Rust de registro (cápsula en Cúmulo, contrato capsule-json-io) calculará y volcará el hash; no se usa `.ps1` como implementación (ver feature `sddia-evolution-sync-norma`).

## Relación con `docs/evolution/`

- `paths.evolutionPath` + `paths.evolutionLogFile` siguen siendo el registro de **cierres de features** y evolución del producto documental.
- `paths.sddiaEvolutionPath` es **exclusivo** del protocolo y normas bajo `./SddIA/` (no sustituye al Evolution Log del producto).
