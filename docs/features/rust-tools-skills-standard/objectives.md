# Objetivos: Estándar Rust para tools y skills

**Proceso:** feature  
**Ruta (Cúmulo):** paths.featurePath/rust-tools-skills-standard/  
**Rama:** feat/rust-tools-skills-standard  
**Contrato:** SddIA/tools/tools-contract.json, SddIA/skills/skills-contract.json

---

## 1. Objetivo

Aplicar el estándar de implementación por defecto en **Rust** (con PS1 como fallback) a los items que actualmente carecen de binario Rust o tienen inconsistencias en su definición.

---

## 2. Items identificados (análisis previo)

### 2.1 tools-rs Cargo.toml
- **Gap:** Cargo.toml declara solo `gesfer_manager` como `[[bin]]`; install.ps1 espera `prepare_full_env` e `invoke_mysql_seeds`.
- **Objetivo:** Añadir `[[bin]]` para `invoke_mysql_seeds` y `prepare_full_env` en scripts/tools-rs/Cargo.toml.

### 2.2 finalizar-git pre_pr (Push-And-CreatePR)
- **Gap:** Push-And-CreatePR.ps1 no tiene binario Rust; es el único componente de skill sin Rust.
- **Objetivo:** Crear `push_and_create_pr.rs` en skills-rs; actualizar finalizar-git para invocar .exe si existe (patrón launcher .bat).

### 2.3 Skills bash legacy (opcional)
- **Gap:** pr-skill.sh, commit-skill.sh, security-validation-skill.sh usan Bash; no siguen patrón Rust/PS1.
- **Objetivo:** Documentar como legacy o migrar a Rust/PS1 según alcance acordado en clarify.

---

## 3. Criterios de aceptación

- [ ] tools-rs Cargo.toml incluye [[bin]] para invoke_mysql_seeds y prepare_full_env.
- [ ] cargo build --release en tools-rs produce ambos binarios.
- [ ] skills-rs incluye push_and_create_pr; install.ps1 lo copia a finalizar-git/bin/.
- [ ] Push-And-CreatePR.bat (o launcher equivalente) invoca push_and_create_pr.exe si existe.
- [ ] Sin regresión en build de skills-rs y tools-rs.

---

## 4. Referencias

- paths.toolsRustPath, paths.skillsRustPath (Cúmulo)
- SddIA/tools/tools-contract.json (default_implementation: rust)
- SddIA/skills/skills-contract.json (default_implementation: rust)
