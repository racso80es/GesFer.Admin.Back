# Contrato de herramientas (Cúmulo: paths.toolsPath / paths.toolCapsules)

**Alcance:** Todas las entidades en **paths.toolsPath** y en cada **paths.toolCapsules[&lt;tool-id&gt;]** (Cúmulo, `SddIA/agents/cumulo.json`) que actúen como herramientas ejecutables. Listado de herramientas: **paths.toolsIndexPath** (índice en raíz de tools).

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
- **Entrega:** cada herramienta reside en una **cápsula** **paths.toolCapsules[&lt;tool-id&gt;]** (Cúmulo). Los ejecutables se construyen en `scripts/tools-rs` y se copian a `&lt;cápsula&gt;/bin/`. Opcional: wrapper `.bat` en **paths.toolsPath** que delegue a la cápsula.
- **Launcher:** dentro de la cápsula, el `.bat` invoca el `.exe` en `bin/` si existe; en caso contrario, fallback al script `.ps1` de la cápsula.
- **Config** (`.json`), **documentación** (`.md`) y **manifest.json** (toolId, components, contract_ref) son obligatorios en la cápsula. **Rutas canónicas:** Cúmulo `SddIA/agents/cumulo.json` → **paths.toolsPath**, **paths.toolCapsules**. En documentación .md no usar rutas literales; referenciar vía Cúmulo.

Referencia: agente Security Engineer (`SddIA/agents/security-engineer.json`).

## 4. Artefactos por herramienta

Cada herramienta reside en una **cápsula** **paths.toolCapsules[&lt;tool-id&gt;]** (Cúmulo) y debe contar con:

- **Implementación Rust:** código en `scripts/tools-rs/src/bin/&lt;tool_bin&gt;.rs`; binario final en `&lt;cápsula&gt;/bin/` (copiado tras `scripts/tools-rs/install.ps1`).
- **Fallback:** script `.ps1` en la cápsula cuando no exista o no se compile el binario Rust.
- **Launcher:** `.bat` en la cápsula que invoque el binario en `bin/` si existe, si no el `.ps1`. Opcional: wrapper `.bat` en **paths.toolsPath** que delegue a la cápsula.
- **manifest.json:** toolId, components (launcher_bat, launcher_ps1, config, doc, bin), contract_ref.
- **Configuración:** cuando sea parametrizable, un `.json` de configuración en la cápsula.
- **Documentación:** un `.md` en la cápsula que describa uso, parámetros y formato de la salida JSON. Idioma: es-ES.

---

## 5. Restricciones

- `toolId` en kebab-case.
- JSON de salida válido y UTF-8.
- No incluir datos sensibles (contraseñas, tokens) en `message`, `feedback` ni `data`.
- Coherencia: `exitCode === 0` solo cuando `success === true`; en fallo, `success === false` y `exitCode !== 0`.

---

## 6. Consumidores

El contrato permite que acciones, agentes, otros scripts y pipelines (CI/CD) consuman un resultado uniforme y un feedback estructurado de todas las herramientas en **paths.toolsPath** y en cada cápsula **paths.toolCapsules[&lt;tool-id&gt;]** (Cúmulo).

**Referencia machine-readable:** `SddIA/tools/tools-contract.json`.
