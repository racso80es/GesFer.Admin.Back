# Contrato de herramientas (scripts/tools/)

**Alcance:** Todas las entidades en `scripts/tools/` que actúen como herramientas ejecutables.

**Objetivo:** Unificar la salida en JSON adecuada al fin de cada herramienta y garantizar un **feedback adecuado** (trazable, por fases y niveles).

---

## 1. Salida JSON

Toda herramienta debe producir un **resultado final en JSON** que cumpla al menos:

| Campo      | Tipo     | Obligatorio | Descripción |
|------------|----------|-------------|-------------|
| `toolId`   | string   | Sí          | Identificador de la herramienta (kebab-case). |
| `exitCode` | number   | Sí          | Código de salida (0 = éxito). |
| `success`  | boolean  | Sí          | `true` si la ejecución fue correcta. |
| `timestamp`| string   | Sí          | ISO 8601 de finalización. |
| `message`  | string   | Sí          | Resumen breve del resultado. |
| `feedback` | array    | Sí          | Lista ordenada de eventos de feedback. |
| `data`     | object   | No          | Datos específicos del fin de la herramienta. |
| `duration_ms` | number | No       | Duración total en milisegundos. |

**Formas de entrega del JSON:**

- **Fichero:** si la herramienta recibe un parámetro de salida (p. ej. `-OutputPath`), escribe el JSON en esa ruta.
- **Stdout:** si se indica `-OutputJson` o `TOOLS_OUTPUT_JSON=1`, emitir el JSON por stdout al final (para piping o integración).

---

## 2. Feedback adecuado

El array `feedback` es el registro de lo que fue ocurriendo durante la ejecución. Cada entrada debe tener:

| Campo       | Tipo   | Descripción |
|-------------|--------|-------------|
| `phase`     | string | Fase o paso (ej. `docker`, `mysql`, `api`, `clients`). |
| `level`     | string | `info` \| `warning` \| `error`. |
| `message`   | string | Mensaje breve y legible. |
| `timestamp` | string | ISO 8601 del evento. |
| `detail`    | string | (Opcional) Detalle o código de error. |
| `duration_ms` | number | (Opcional) Duración del paso en ms. |

**Reglas:**

1. **Trazabilidad:** Cada fase o paso significativo debe generar al menos una entrada en `feedback`.
2. **Errores:** Si algo falla, debe existir una entrada con `level: "error"` que describa el fallo.
3. **Advertencias:** Situaciones recuperables (timeouts parciales, recursos no encontrados pero opcionales) deben registrarse con `level: "warning"`.
4. **Orden:** Las entradas deben ir en orden cronológico (primera ocurrencia primero).

Así se mantiene un **feedback adecuado** tanto para humanos (mensajes claros) como para máquinas (fases, niveles, tiempos).

---

## 3. Artefactos por herramienta

Cada herramienta en `scripts/tools/` debe contar con:

- **Ejecutable:** script `.ps1` (o `.bat` que invoque un `.ps1`).
- **Configuración:** cuando sea parametrizable, un `.json` de configuración (p. ej. `prepare-env.json`).
- **Documentación:** un `.md` que describa uso, parámetros y formato de la salida JSON. Idioma: es-ES.

---

## 4. Restricciones

- `toolId` en kebab-case.
- JSON de salida válido y UTF-8.
- No incluir datos sensibles (contraseñas, tokens) en `message`, `feedback` ni `data`.
- Coherencia: `exitCode === 0` solo cuando `success === true`; en fallo, `success === false` y `exitCode !== 0`.

---

## 5. Consumidores

El contrato permite que acciones, agentes, otros scripts y pipelines (CI/CD) consuman un resultado uniforme y un feedback estructurado de todas las herramientas en `scripts/tools/`.

**Referencia machine-readable:** `SddIA/tools/tools-contract.json`.
