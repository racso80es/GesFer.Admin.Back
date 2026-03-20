---
constraints:
- Cumplimiento obligatorio de Karma2Token para trazabilidad y seguridad cuando aplique security_model.
- skill-id en kebab-case.
- Rutas canónicas solo desde Cúmulo (paths.skillsDefinitionPath, paths.skillCapsules, paths.skillsIndexPath).
- Un skill sin spec.md (con frontmatter YAML) en su carpeta no se considera completo.
- Las acciones y agentes resuelven la definición en SddIA y la implementación vía Cúmulo.
- E/S JSON para agentes según SddIA/norms/capsule-json-io.md (stdin / stdout UTF-8).
consumers:
- paths.actionsPath
- SddIA/agents/*.json
- paths.processPath
contract_version: 2.0.0
default_implementation:
  delivery:
  - Cada skill con ejecutable reside en una cápsula paths.skillCapsules[skill-id]. Los ejecutables se construyen en paths.skillsRustPath (Cúmulo) y se copian a la raíz de la cápsula (<cápsula>/<nombre>.exe), sin subcarpeta bin/.
  - Launcher .bat opcional en la cápsula, solo para uso humano (envoltorio del .exe); no es la interfaz del agente.
  - 'manifest.json, documentación .md obligatorios en la cápsula. Rutas canónicas: Cúmulo paths.skillCapsules.'
  language: rust
  rationale: Rendimiento, seguridad de memoria, portabilidad y distribución como binario. Las implementaciones por defecto de scripts de skills (y de tools) han de ser en Rust.
definition_artefacts:
- ext: .md
  format: frontmatter_yaml
  naming: spec.md
  path: paths.skillsDefinitionPath/<skill-id>/
  purpose: 'Especificación: frontmatter YAML (metadatos) + cuerpo Markdown. Campos: skill_id, name, description, implementation_path_ref. es-ES.'
description: 'Contrato que todo skill debe cumplir: definición en paths.skillsDefinitionPath/<skill-id>/, implementación en paths.skillCapsules[skill-id]. Implementación por defecto en Rust; invocación por agente vía .exe y JSON (capsule-json-io).'
implementation:
  format: Rust executable (.exe)
  location_pattern: scripts/skills/{skill-id}/{nombre}.exe
  migration_note: Corte limpio v2. sin bin/ ni .ps1 en cápsula. Agente invoca el .exe con stdin/stdout JSON.
  prohibited_formats:
  - .ps1
  - .sh
  source_location: scripts/skills-rs/src/bin/{nombre}.rs
  standard: Solo ejecutables .exe compilados desde Rust. PowerShell (.ps1) prohibido como implementación. .bat solo como launcher humano sin lógica de negocio.
implementation_requirements:
- El ejecutable debe desarrollarse en Rust en paths.skillsRustPath (Cúmulo) y cumplir SddIA/norms/capsule-json-io.md para invocación por agentes.
required_artefacts_capsule:
- ext: .rs
  path: paths.skillsRustPath (Cúmulo)/src/bin
  purpose: Implementación por defecto en Rust.
- ext: binary
  path: <cápsula>/<skill_bin>.exe
  purpose: Ejecutable en la raíz de la cápsula (copiado desde skills-rs/install.ps1). OBLIGATORIO.
- ext: .bat
  purpose: 'Opcional: launcher humano que invoca el .exe en la misma carpeta; no usar como interfaz del agente.'
- ext: .json
  naming: manifest
  purpose: 'Manifest de la cápsula: skillId, components, contract_ref.'
- ext: .md
  purpose: Documentación en la cápsula. es-ES.
json_io_ref: SddIA/norms/capsule-json-io.md
scope: SddIA/skills/
security_model:
  description: Todo skill debe ser invocado bajo el contexto de un Karma2Token válido cuando el contrato lo exija.
  required_token: Karma2Token
  token_ref: SddIA/tokens/karma2-token/spec.json
---

# Contrato de skills (Cúmulo: paths.skillsPath / paths.skillCapsules)

**Alcance:** Todas las entidades en **paths.skillsPath** y en cada **paths.skillCapsules[&lt;skill-id&gt;]** (Cúmulo (`SddIA/agents/cumulo.json`) que actúen como skills con implementación ejecutable. Listado: **paths.skillsIndexPath**.

**Desacoplamiento definición / implementación:** La **definición** está en **paths.skillsDefinitionPath**/&lt;skill-id&gt;/ (spec.md, spec.json). La **implementación** está en **paths.skillCapsules[&lt;skill-id&gt;]**. En spec.json debe figurar **implementation_path_ref** (ej. `paths.skillCapsules.&lt;skill-id&gt;`).

**E/S JSON (agentes):** formato único compartido con tools; ver **SSOT:** [SddIA/norms/capsule-json-io.md](../norms/capsule-json-io.md).

---

## 1. Definición por skill (paths.skillsDefinitionPath/&lt;skill-id&gt;/)

- **spec.md** — Especificación legible; debe documentar el objeto **`request`** y el objeto **`result`** (nombres de campos y semántica) para ese skill.
- **spec.json** — Metadatos; **implementation_path_ref** obligatorio si hay ejecutable.

---

## 2. Implementación por defecto: Rust

- **Binario:** `paths.skillsRustPath` → compilación → copia a **raíz** de `paths.skillCapsules[&lt;skill-id&gt;]` como `&lt;nombre&gt;.exe` (**sin** carpeta `bin/`).
- **Agente / IA:** invocar únicamente el `.exe`, pasando el JSON de petición por **stdin** y leyendo el JSON de respuesta por **stdout** (norma capsule-json-io).
- **Humano:** opcionalmente `.bat` en la cápsula que invoque el mismo `.exe`; **prohibido** usar `.ps1` como implementación o fallback.
- **manifest.json** y documentación `.md` obligatorios en la cápsula.

---

## 3. Artefactos por skill (con ejecutable)

- Código Rust: `scripts/skills-rs/src/bin/&lt;skill_bin&gt;.rs` (ruta lógica; canónica vía Cúmulo paths.skillsRustPath).
- Ejecutable en **raíz de cápsula**.
- **manifest.json:** skillId, version, description, contract_ref, components (launcher_bat opcional, doc, exe); no listar `launcher_ps1`.
- Documentación del skill en la cápsula y en paths.skillsDefinitionPath.

---

## 4. Restricciones

- skill-id en kebab-case.
- Rutas solo desde Cúmulo.
- Skills sin ejecutable: solo definición en paths.skillsDefinitionPath.

---

## 5. Consumidores

Acciones, agentes y procesos consumen skills mediante rutas Cúmulo y el contrato JSON de capsule-json-io.

**Referencia machine-readable (si existe):** `paths.skillsDefinitionPath/skills-contract.json`.
