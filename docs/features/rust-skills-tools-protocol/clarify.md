# Clarificación: rust-skills-tools-protocol

**Feature:** rust-skills-tools-protocol  
**Rama:** feat/rust-skills-tools-protocol

---

## Resolución de ambigüedades

1. **Relación con git-via-skills-or-process:** La nueva norma **comandos-via-skills-or-tools** generaliza el principio: no solo git, sino cualquier comando (dotnet, npm, pwsh, etc.) debe pasar por skill/tool/acción/proceso. Se mantiene git-via-skills-or-process como norma específica de git; la nueva norma es la de ámbito general y se referencia desde AGENTS.
2. **Ubicación del contexto Rust:** Se añade en SddIA/constitution.json en `configuration` (p. ej. `skills_tools_implementation: "rust"`) y se menciona en paths_ref. No se crea un documento separado de "contexto" si constitution basta.
3. **sddia-difusion:** Se actualiza el spec de la acción para que sus criterios de aceptación incluyan "norma comandos vía skills/tools" y "contexto Rust"; la ejecución de la acción en este feature generará/actualizará la regla en .cursor/rules.
4. **Regla .cursor:** Se puede crear `commands-via-skills-tools.mdc` o integrar el texto en `sddia-ssot.mdc`; se prefiere un .mdc dedicado para visibilidad y búsqueda.

## Gaps cerrados

- Ningún gap crítico; alcance acotado a norma, contexto y difusión.
