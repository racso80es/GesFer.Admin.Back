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

## 3. Implementación por defecto: Rust

**Las implementaciones por defecto de las herramientas (y de los scripts de skills) han de ser en Rust.**

- **Motivo:** rendimiento, seguridad de memoria, portabilidad y distribución como binario único.
- **Entrega:** todos los ejecutables en **scripts/tools/** (ej. `prepare_full_env.exe`, `invoke_mysql_seeds.exe`). Se construyen en `scripts/tools-rs` y se copian a `scripts/tools/`.
- **Launcher:** el `.bat` o `.ps1` en `scripts/tools/` invoca el `.exe` de la misma carpeta si existe; en caso contrario, fallback al script PowerShell.
- **Config** (`.json`) y **documentación** (`.md`) siguen siendo obligatorios.

Referencia: agente Security Engineer (`SddIA/agents/security-engineer.json`).

## 4. Artefactos por herramienta

Cada herramienta en `scripts/tools/` debe contar con:

- **Implementación Rust:** código en `scripts/tools-rs/src/bin/<tool_id>.rs`; binario final en **scripts/tools/** (copiado tras `cargo build --release`).
- **Fallback:** script `.ps1` cuando no exista o no se compile el binario Rust.
- **Launcher:** `.bat` que invoque Rust si existe, si no `.ps1`.
- **Configuración:** cuando sea parametrizable, un `.json` de configuración.
- **Documentación:** un `.md` que describa uso, parámetros y formato de la salida JSON. Idioma: es-ES.

---

## 5. Restricciones

- `toolId` en kebab-case.
- JSON de salida válido y UTF-8.
- No incluir datos sensibles (contraseñas, tokens) en `message`, `feedback` ni `data`.
- Coherencia: `exitCode === 0` solo cuando `success === true`; en fallo, `success === false` y `exitCode !== 0`.

---

## 6. Consumidores

El contrato permite que acciones, agentes, otros scripts y pipelines (CI/CD) consuman un resultado uniforme y un feedback estructurado de todas las herramientas en `scripts/tools/`.

**Referencia machine-readable:** `SddIA/tools/tools-contract.json`.
