# SPEC: Protocolo Rust para skills y tools + norma comandos vía skills/tools

**ID:** SPEC-RUST-SKILLS-TOOLS-PROTOCOL-2026-02  
**Rama:** feat/rust-skills-tools-protocol  
**Cúmulo:** paths.featurePath/rust-skills-tools-protocol/

---

## 1. Propósito

1. Establecer en SddIA que la implementación estándar de skills y tools es **Rust** (binarios .exe en paths.skillsRustPath y paths.toolsRustPath; fallback PS1/bat en cápsulas).
2. Introducir la norma universal: **toda ejecución de comandos** (sistema, git, dotnet, npm, etc.) debe realizarse **solo mediante skills o tools**, nunca invocando comandos directamente en la shell por parte de la IA.
3. Integrar esta norma y el contexto Rust en AGENTS.md, constitution y en la acción sddia-difusion para que .cursor/rules y demás touchpoints la reflejen.

## 2. Alcance

### Incluido

- Nueva norma en SddIA/norms: comandos solo vía skills o tools (generalización de git-via-skills-or-process).
- Actualización de AGENTS.md: ley o referencia a la norma de ejecución de comandos.
- Actualización de SddIA/constitution.json (o documento de contexto): identificación explícita de Rust como implementación de skills y tools.
- Actualización de la acción sddia-difusion (spec/salidas) para incluir la nueva norma y el contexto Rust en los criterios de difusión.
- Ejecución de sddia-difusion: actualizar .cursor/rules (regla explícita o ampliación de sddia-ssot / nueva regla commands-via-skills-tools.mdc).
- Documentación en paths.featurePath/rust-skills-tools-protocol/.

### Fuera de alcance

- Migrar implementaciones existentes de PS1 a Rust (ya cubierto por feature rust-tools-skills-standard u otros).
- Cambiar la implementación binaria de skills/tools individuales en este feature.

## 3. Touchpoints

| Área | Archivo / Ubicación | Cambio |
|------|---------------------|--------|
| SddIA/norms | commands-via-skills-or-tools.md (nuevo) | Norma: ejecutar comandos solo vía skill/tool/acción/proceso. |
| SddIA/norms | README.md | Entrada en índice de normas. |
| AGENTS.md | Leyes / sección GIT o nueva | Referencia a norma comandos vía skills/tools. |
| SddIA/constitution.json | configuration | Añadir skills_tools_implementation: "rust", paths_ref ampliado. |
| SddIA/actions/sddia-difusion | spec.md | Criterios: incluir norma comandos y contexto Rust. |
| .cursor/rules | commands-via-skills-tools.mdc o sddia-ssot.mdc | Difusión: no ejecutar comandos directamente; Rust estándar. |
| docs/features/rust-skills-tools-protocol | spec, plan, implementation, validacion | Artefactos del proceso. |

## 4. Requisitos

- La norma debe ser machine-readable (opcional: interaction-triggers o norms index .json).
- Cúmulo ya expone paths.skillsRustPath y paths.toolsRustPath; no cambiar rutas en este feature.
- Entorno: Windows 11 + PowerShell 7+; la invocación de skills/tools sigue siendo vía .bat/.ps1 que llaman a .exe si existe.
