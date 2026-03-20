---
constraints:
- Cumplimiento obligatorio de Karma2Token para trazabilidad y seguridad cuando aplique security_model.
- toolId en kebab-case.
- El JSON de salida debe ser válido y codificado en UTF-8.
- No escribir feedback sensible (contraseñas, tokens) en message ni en result.
- exitCode 0 únicamente cuando success es true; en caso de fallo, success false y exitCode != 0.
- E/S JSON para agentes según SddIA/norms/capsule-json-io.md (stdin / stdout UTF-8).
consumers:
- paths.actionsPath
- SddIA/agents/*.json
- paths.toolsPath
- CI/CD
contract_version: 2.0.0
default_implementation:
  delivery:
  - Cada herramienta reside en paths.toolCapsules/<tool-id>/ con manifest.json. Los ejecutables se construyen en paths.toolsRustPath (Cúmulo) y se copian a la raíz de la cápsula (<cápsula>/<nombre>.exe), sin subcarpeta bin/.
  - Launcher .bat opcional en la capsula, solo para uso humano (envoltorio del .exe).
  - 'Config (.json), documentacion (.md) y manifest.json obligatorios en la capsula. Rutas canonicas: Cúmulo paths.toolCapsules.'
  language: rust
  rationale: Rendimiento, seguridad de memoria, portabilidad y distribución como binario. Las implementaciones por defecto de tools y skills han de ser en Rust formato exe.
definition_artefacts:
- ext: .md
  format: frontmatter_yaml
  naming: spec.md
  path: paths.toolsDefinitionPath/<tool-id>/
  purpose: 'Especificación: frontmatter YAML (metadatos) + cuerpo Markdown. Campos: toolId, version, description, implementation_path_ref. es-ES.'
description: 'Contrato que toda herramienta (tool) debe cumplir: definición en paths.toolsDefinitionPath/<tool-id>/; implementación en paths.toolCapsules; invocación por agente vía .exe y JSON (capsule-json-io).'
feedback:
  description: 'Las herramientas deben mantener un feedback adecuado: trazabilidad de fases, niveles y mensajes (array feedback del envelope de salida).'
  entry_schema:
    level: info | warning | error.
    message: Texto breve y legible del evento.
    phase: Identificador de la fase o paso (ej. docker, mysql, api, clients).
    timestamp: ISO 8601 del momento del evento.
  optional_per_entry:
    detail: Texto adicional o código de error.
    duration_ms: Duración del paso en ms.
  rules:
  - Cada fase o paso significativo debe generar al menos una entrada en feedback.
  - En caso de error, debe existir una entrada con level 'error' que describa el fallo.
  - Las advertencias (recursos no disponibles, timeouts parciales) deben registrarse con level 'warning'.
implementation:
  format: Rust executable (.exe)
  location_pattern: scripts/tools/{tool-id}/{nombre}.exe
  migration_note: Corte limpio v2. sin bin/ ni .ps1. Campo de salida data renombrado a result (ver capsule-json-io). Agente usa solo stdin/stdout JSON.
  prohibited_formats:
  - .ps1
  - .sh
  source_location: scripts/tools-rs/src/bin/{nombre}.rs
  standard: Solo ejecutables .exe compilados desde Rust. .bat solo como launcher humano sin lógica de negocio.
output:
  description: >-
    Salida para agentes un único JSON en stdout según SddIA/norms/capsule-json-io.md.
    El payload específico va en result (antes data en v1.x).
json_io_ref: SddIA/norms/capsule-json-io.md
required_artefacts_per_tool:
- ext: .rs
  path: paths.toolsRustPath (Cúmulo)/src/bin
  purpose: Implementación por defecto en Rust.
- ext: binary
  path: paths.toolCapsules/<tool-id>/<tool_bin>.exe
  purpose: Ejecutable en la raíz de la cápsula (copiado desde tools-rs/install.ps1). OBLIGATORIO.
- ext: .bat
  purpose: 'Opcional: launcher humano que invoca el .exe en la misma carpeta.'
- ext: .json
  naming: manifest
  purpose: 'Manifest de la capsula: toolId, components, contract_ref (obligatorio en cada capsula).'
- ext: .json
  naming: <tool-id>-config
  purpose: Configuración machine-readable.
- ext: .md
  naming: <tool-id>.md
  purpose: 'Documentación. Idioma: es-ES.'
scope: paths.toolsPath (Cúmulo), paths.toolsDefinitionPath (SddIA/tools/)
security_model:
  description: Toda tool debe ejecutarse bajo el contexto de un Karma2Token válido cuando el contrato lo exija.
  required_token: Karma2Token
  token_ref: SddIA/tokens/karma2-token/spec.json
---

# Contrato de herramientas (Cúmulo: paths.toolsPath / paths.toolCapsules)

**Alcance:** Todas las entidades en **paths.toolsPath** y en cada **paths.toolCapsules[&lt;tool-id&gt;]** (Cúmulo). Listado: **paths.toolsIndexPath**.

**Desacoplamiento definición / implementación:** definición en **paths.toolsDefinitionPath**/&lt;tool-id&gt;/; implementación en **paths.toolCapsules[&lt;tool-id&gt;]** con **implementation_path_ref** en spec.

**E/S JSON (agentes):** mismo envelope que skills; ver **SSOT:** [SddIA/norms/capsule-json-io.md](../norms/capsule-json-io.md).

---

## 1. Salida JSON (resumen)

Los campos obligatorios y la forma de `meta`, `success`, `exitCode`, `message`, `feedback`, `result` y `duration_ms` están definidos en **capsule-json-io.md**. Las specs de cada tool documentan el contenido de **`request`** y **`result`**.

**Corte limpio v2:** el campo histórico `data` de la v1 queda sustituido por **`result`**.

**Entrega:** exclusivamente **stdout** (una respuesta JSON) para invocación por agente.

---

## 2. Feedback adecuado

Mismo modelo de entradas en `feedback` que en la norma capsule-json-io (phase, level, message, timestamp, detalle y duration opcionales).

---

## 3. Implementación por defecto: Rust

- Binarios en **raíz** de la cápsula; **sin** `bin/` ni `.ps1`.
- **Agente:** solo `.exe` + JSON stdin/stdout.
- **Humano:** `.bat` opcional.

---

## 4. Artefactos por herramienta

- Implementación Rust bajo paths.toolsRustPath; ejecutable en raíz de paths.toolCapsules/&lt;tool-id&gt;.
- **manifest.json:** toolId, components (launcher_bat opcional, config, doc, exe); no listar launcher_ps1.
- Config y documentación según contrato.

---

## 5. Restricciones

- toolId en kebab-case.
- JSON válido UTF-8; sin secretos en message, feedback ni result.
- Coherencia success / exitCode.

---

## 6. Consumidores

Acciones, agentes, scripts y pipelines consumen el envelope unificado.

**Referencia machine-readable (si existe):** `SddIA/tools/tools-contract.json`.
