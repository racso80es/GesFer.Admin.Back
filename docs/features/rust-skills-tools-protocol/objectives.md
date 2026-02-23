# Objetivos: Protocolo Rust para skills y tools

**Feature:** rust-skills-tools-protocol  
**Rama:** feat/rust-skills-tools-protocol  
**Cúmulo:** paths.featurePath/rust-skills-tools-protocol/

---

## 1. Objetivos principales

1. **Estándar exe (Rust):** Asegurar que toda interacción con skills y tools utilice el estándar definido: implementación por defecto en Rust (binarios .exe); PS1/.bat como fallback cuando no exista binario.
2. **Contexto SddIA:** Dejar en SddIA correctamente configurado e identificado que la implementación de skills y tools es en Rust (paths.skillsRustPath, paths.toolsRustPath; contrato y constitution).
3. **Norma de ejecución:** Integrar en los protocolos/comportamiento IA la norma: **ejecutar comandos siempre mediante skills o tools, nunca directamente** (extender más allá de git a todo comando de sistema).
4. **Difusión:** Ejecutar la acción sddia-difusion para mantener .cursor/rules y otros touchpoints alineados con SddIA (incl. nueva norma y contexto Rust).

## 2. Criterios de éxito

- SddIA/norms incluye norma explícita "comandos solo vía skills o tools".
- AGENTS.md y SddIA/constitution (o documento de contexto) referencian Rust como implementación estándar de skills y tools.
- .cursor/rules actualizados vía sddia-difusion con la norma y el listado/canon.
- Proceso documentado en paths.featurePath/rust-skills-tools-protocol/ con spec, plan, implementación y validación.

## 3. Fuentes

- paths.normsPath (git-via-skills-or-process.md como base a generalizar).
- paths.actionsPath/sddia-difusion/.
- paths.processPath/feature/.
- Cúmulo: paths.skillsRustPath, paths.toolsRustPath, paths.skillCapsules, paths.toolCapsules.
