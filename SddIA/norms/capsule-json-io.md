---
norm_id: capsule-json-io
contract_ref: SddIA/skills/skills-contract.md, SddIA/tools/tools-contract.md
related:
  - SddIA/tokens/karma2-token/spec.json
  - SddIA/norms/commands-via-skills-or-tools.md
description: >-
  Envelope JSON único para invocación de skills y tools por agentes (stdin/stdout UTF-8).
schema_version: "2.0"
---

# Norma: E/S JSON de cápsulas (skills y tools)

**Ámbito:** toda **skill** con ejecutable en `paths.skillCapsules` y toda **tool** en `paths.toolCapsules`, cuando las invoca un **agente de IA**.

**Transporte (decisión cerrada):** una única línea o un único documento JSON en **stdin**; respuesta única en **stdout**. Sin ficheros de intercambio obligatorios.

**Codificación:** UTF-8. El proceso debe terminar con código de salida del proceso igual a `exitCode` del JSON de respuesta (salvo fallo de serialización previo, que debe usar código distinto de cero).

---

## 1. Envelope de petición (`stdin`)

| Campo | Tipo | Obligatorio | Descripción |
|-------|------|-------------|-------------|
| `meta` | object | Sí | Metadatos comunes (ver abajo). |
| `request` | object | Sí | Parámetros específicos del skill o tool. Puede ser `{}` si no hay argumentos. |

### 1.1 `meta` (petición)

| Campo | Tipo | Obligatorio | Descripción |
|-------|------|-------------|-------------|
| `schema_version` | string | Sí | Versión del envelope; valor actual: `"2.0"`. |
| `entity_kind` | string | Sí | `"skill"` \| `"tool"`. |
| `entity_id` | string | Sí | `skill_id` o `toolId` en **kebab-case** (debe coincidir con el binario invocado). |
| `token` | string | Condicional | **Karma2Token** u objeto serializado según `SddIA/tokens/karma2-token/spec.json`. Obligatorio cuando el contrato de la entidad declare `security_model.required_token`. |

### 1.2 `request`

Objeto libre definido por la **spec** de cada skill (paths.skillsDefinitionPath) o tool (paths.toolsDefinitionPath). Aquí se personalizan flags, rutas, opciones de negocio, etc.

---

## 2. Envelope de respuesta (`stdout`)

Misma forma lógica para **skills** y **tools**.

| Campo | Tipo | Obligatorio | Descripción |
|-------|------|-------------|-------------|
| `meta` | object | Sí | Metadatos de respuesta (ver abajo). |
| `success` | boolean | Sí | `true` si la ejecución lógica fue correcta. |
| `exitCode` | number | Sí | Código de salida (0 = éxito). Debe alinearse con `success`. |
| `message` | string | Sí | Resumen breve, apto para logs y UI. |
| `feedback` | array | Sí | Lista ordenada cronológicamente de eventos (ver §3). Puede ser `[]`. |
| `result` | object | Sí | Salida específica de la skill/tool; usar `{}` si no hay payload. |
| `duration_ms` | number \| null | No | Duración total en milisegundos; `null` si no se mide. |

### 2.1 `meta` (respuesta)

| Campo | Tipo | Obligatorio | Descripción |
|-------|------|-------------|-------------|
| `schema_version` | string | Sí | Igual que la petición; `"2.0"`. |
| `entity_kind` | string | Sí | `"skill"` \| `"tool"`. |
| `entity_id` | string | Sí | Mismo valor que en la petición (eco o comprobación). |
| `timestamp` | string | Sí | ISO 8601 en UTC, instante de finalización. |

### 2.2 Reglas de coherencia

- `exitCode === 0` **si y solo si** `success === true`.
- No incluir secretos (contraseñas, tokens completos, cookies) en `message`, `feedback` ni `result`.
- En caso de error de validación del JSON de entrada, responder con `success: false`, exitCode ≠ 0, `message` descriptivo y `result` opcional con detalle no sensible.

---

## 3. Entradas del array `feedback`

Cada elemento:

| Campo | Tipo | Obligatorio | Descripción |
|-------|------|-------------|-------------|
| `phase` | string | Sí | Paso o fase (ej. `git`, `docker`, `build`). |
| `level` | string | Sí | `info` \| `warning` \| `error`. |
| `message` | string | Sí | Texto breve. |
| `timestamp` | string | Sí | ISO 8601 del evento. |
| `detail` | string | No | Detalle o código de error. |
| `duration_ms` | number | No | Duración del paso en ms. |

**Reglas:** Trazabilidad por fases; ante fallo debe existir al menos una entrada con `level: "error"`. Skills que hoy no emiten fases pueden devolver `feedback: []`.

---

## 4. Invocación humana vs agente

| Actor | Entrada | Notas |
|-------|---------|--------|
| **Agente / IA** | `.exe` con JSON por **stdin**; leer JSON por **stdout**. | No usar `.bat` como interfaz del agente. |
| **Humano** | `.bat` opcional en la cápsula: envoltorio mínimo sobre el mismo `.exe` sin scripts `.ps1`. | La semántica debe coincidir con la invocación JSON cuando sea posible; si el `.bat` solo pasa argumentos legacy, debe documentarse en la cápsula. |

### 4.1 Entornos donde stdin no es fiable (agentes / CI)

Si **stdin no es TTY** y el proceso padre deja el pipe **abierto sin cerrar** (EOF), una lectura bloqueante hasta EOF puede **colgar** el binario. El crate **`gesfer-capsule`** (`scripts/gesfer-capsule`) resuelve la petición en este orden:

| Prioridad | Mecanismo | Efecto |
|-----------|-----------|--------|
| 1 | Variable de entorno **`GESFER_CAPSULE_REQUEST`** | JSON del envelope completo (UTF-8). **No** se lee stdin. |
| 2 | **`GESFER_SKIP_STDIN=1`** (o `true`) | No se lee stdin; modo **CLI** con los mismos argumentos que en TTY (`clap` / flags documentados). |
| 3 | stdin TTY | Modo CLI (`Ok(None)` en la capa de lectura). |
| 4 | stdin no TTY | Lectura hasta EOF (comportamiento estándar). |

Recomendación para **IA / runners**: usar **`GESFER_CAPSULE_REQUEST`** o **`GESFER_SKIP_STDIN`** + flags CLI cuando no se pueda garantizar pipe + EOF.

---

## 5. Migración desde contrato anterior (tools)

- Campo histórico `data` en la salida JSON de tools queda **reemplazado** por **`result`** (corte limpio v2.0).
- Ubicación del binario: **raíz de cápsula**, no subcarpeta `bin/`.

---

*Norma canónica del envelope compartido. Los contratos skills-contract y tools-contract incorporan la referencia a este documento como SSOT del formato.*
