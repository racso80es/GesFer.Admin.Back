# SPEC: Estándar Rust para tools y skills

**ID:** SPEC-RUST-TOOLS-SKILLS-2026-01  
**Rama:** feat/rust-tools-skills-standard  
**Contexto (Cúmulo):** paths.featurePath/rust-tools-skills-standard/

---

## 1. Propósito

Alinear tools y skills con el contrato: implementación por defecto en Rust; PS1 como fallback cuando no exista binario. Corregir gaps en tools-rs (Cargo.toml) y skills-rs (Push-And-CreatePR sin Rust).

---

## 2. Alcance

### Incluido
- **tools-rs:** Añadir [[bin]] para invoke_mysql_seeds y prepare_full_env en Cargo.toml.
- **skills-rs:** Crear push_and_create_pr.rs; actualizar install.ps1; launcher en finalizar-git para pre_pr.
- **Verificación:** cargo build en tools-rs y skills-rs exitoso; install.ps1 ejecutables.

### Fuera de alcance (clarify)
- Migración de skills bash (pr-skill.sh, commit-skill.sh, security-validation-skill.sh).
- Cambios en lógica de negocio de tools/skills existentes.

---

## 3. Touchpoints

| Área | Archivo | Cambio |
|------|---------|--------|
| tools-rs | Cargo.toml | [[bin]] invoke_mysql_seeds, prepare_full_env |
| skills-rs | Cargo.toml | [[bin]] push_and_create_pr |
| skills-rs | src/bin/push_and_create_pr.rs | Nuevo binario |
| skills-rs | install.ps1 | Copiar push_and_create_pr.exe a finalizar-git/bin/ |
| finalizar-git | Push-And-CreatePR.bat | Invocar .exe si existe; fallback PS1 |
| finalizar-git | manifest.json | Componente bin push_and_create_pr.exe |

---

## 4. Requisitos

- Rust en PATH (cargo, rustc).
- Launcher .bat: bin/<nombre>.exe si existe; si no, .ps1.
- tools-contract y skills-contract: default_implementation: rust.
