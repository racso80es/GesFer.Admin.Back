---
process_id: create-skill
name: "Creación de skills (create-skill)"
description: "Proceso de tarea para crear una nueva skill (definición + cápsula ejecutable) cumpliendo contrato de skills, rutas vía Cúmulo y ejecución por binario Rust + envelope JSON."
contract_ref: paths.processPath/process-contract.md
tools_contract_ref: SddIA/skills/skills-contract.md
principles_ref: paths.principlesPath
inputs:
  description: "Descripción breve de la skill. Obligatorio."
  skill_id: "kebab-case. Obligatorio."
paths:
  featurePath_ref: paths.featurePath
  skillsPath_ref: paths.skillsPath
  skillCapsules_ref: paths.skillCapsules
  skillsDefinitionPath_ref: paths.skillsDefinitionPath
  skillsIndexPath_ref: paths.skillsIndexPath
  skillsRustPath_ref: paths.skillsRustPath
persist_ref: paths.featurePath/create-skill-<skill-id>
process_doc_ref: paths.processPath/create-skill/
process_interface_compliance: "Definición del proceso: spec.md + spec.json (paths.processPath). Artefactos de tarea: solo `.md` con frontmatter YAML (paths.featurePath) según Cúmulo y features-contract."
related_actions:
  - spec
  - implementation
  - validate
  - finalize
related_skills:
  - iniciar-rama
spec_version: 1.0.0
triggers:
  - "Crear nueva skill en paths.skillsPath"
  - "Solicitud de creación de skill con skill-id"
---

# Proceso: Creación de skills (create-skill)

Este documento define el **proceso de tarea** para crear una nueva **skill** en el proyecto. Está ubicado en `paths.processPath/create-skill/` (Cúmulo). Las rutas de skills se obtienen de **Cúmulo**: `paths.skillsPath`, `paths.skillCapsules`, `paths.skillsIndexPath`, `paths.skillsDefinitionPath`, `paths.skillsRustPath`.

## Propósito

El proceso **create-skill** define el procedimiento para incorporar una nueva skill al ecosistema de ejecución del repositorio:

- **Definición (SddIA)**: `paths.skillsDefinitionPath/<skill-id>/` con `spec.md` (y `spec.json` si el contrato de skills lo requiere).
- **Cápsula (entregable ejecutable)**: `paths.skillCapsules[<skill-id>]` (bajo `paths.skillsPath`) con **`<skill>.exe` en la raíz de la cápsula**, `manifest.json` y documentación (contrato `SddIA/skills/skills-contract.md`).
- **Índice**: actualización de `paths.skillsIndexPath`.
- **Trazabilidad**: todo bajo **Karma2Token** y registro de evolution si cambia `./SddIA/`.

## Alcance del procedimiento

- **Documentación de la tarea**: `paths.featurePath/create-skill-<skill-id>/` (Cúmulo).
- **Definición (SddIA)**: `paths.skillsDefinitionPath/<skill-id>/`.
- **Cápsula (implementación)**: `paths.skillCapsules[<skill-id>]`.

Fases recomendadas: 0 Preparar entorno | 1 Objetivos y especificación | 2 Definición en SddIA | 3 Cápsula (Rust + envelope JSON) | 4 Índice + rutas | 5 Validación | 6 Cierre.

## Restricciones

- `skill_id` en **kebab-case**.
- Rama: `feat/create-skill-<skill-id>`.
- Entorno: **Windows 11 + PowerShell 7+**.
- **Prohibido** implementar la skill como scripts `.ps1`, `.bat`, `.sh`. La entrega ejecutable es **`.exe` compilado desde Rust**.
- Toda ejecución debe respetar `SddIA/norms/capsule-json-io.md` (entrada/salida JSON por stdin/stdout).

## Implementación de la skill

La skill debe implementarse como ejecutable Rust (`.exe`) y exponer un contrato estable (envelope JSON) consumible por acciones, procesos y agentes.

**Estructura esperada de la cápsula:**

```
scripts/skills/<skill-id>/
├── <skill-name>.exe            # Ejecutable Rust compilado (OBLIGATORIO, en raíz de cápsula)
├── manifest.json               # Metadatos de la skill (OBLIGATORIO)
└── <skill-name>.md             # Documentación de uso (OBLIGATORIO)
```

**Fuente Rust:**

```
scripts/skills-rs/src/<skill-name>.rs
```

O, si es complejo:

```
scripts/skills-rs/src/<skill-name>/
├── main.rs
├── lib.rs
└── ...
```

## Referencias

- Contrato de procesos: `paths.processPath/process-contract.md`.
- Contrato de skills: `SddIA/skills/skills-contract.md`.
- Norma: `SddIA/norms/commands-via-skills-or-tools.md`.
- Envelope JSON: `SddIA/norms/capsule-json-io.md`.
- Rutas: `SddIA/norms/paths-via-cumulo.md` (Cúmulo).
- Portar este proceso a otro repo SddIA: `paths.processPath/create-skill/reproducir-create-skill-en-otros-entornos-sddia.md`.
