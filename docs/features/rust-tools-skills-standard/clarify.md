# Clarificación: Estándar Rust para tools y skills

**Proceso:** feature  
**Ruta (Cúmulo):** docs/features/rust-tools-skills-standard/  
**Fecha:** 2026-02-21  
**Especificación:** spec.md  

---

## 1. Gaps resueltos

### 1.1 tools-rs: ¿gesfer_manager en Cargo.toml?
**Gap:** Cargo.toml tiene gesfer_manager; install.ps1 usa prepare_full_env e invoke_mysql_seeds.

**Resolución:** Añadir [[bin]] para invoke_mysql_seeds y prepare_full_env. Mantener gesfer_manager si existe src/bin/gesfer_manager.rs; si no, puede quedar como binario adicional o eliminarse según exista fuente. Verificación: glob mostró invoke_mysql_seeds.rs y prepare_full_env.rs; gesfer_manager.rs existe en Cargo.toml. Los tres son válidos.

### 1.2 push_and_create_pr: parámetros CLI
**Gap:** Push-And-CreatePR.ps1 usa -BranchName, -Persist, -Title, -BaseBranch. ¿API del binario Rust?

**Resolución:** push_and_create_pr.exe aceptará argumentos posicionales o flags: --branch, --persist, --title, --base (main|master). Compatible con invocación desde .bat: push_and_create_pr.exe --persist "docs/features/xxx/".

### 1.3 Skills bash (pr-skill, commit-skill, security-validation-skill)
**Gap:** ¿Incluir migración a Rust/PS1 en esta feature?

**Resolución:** **Fuera de alcance** en esta feature. Documentar como legacy; migración futura si se requiere. No bloquea objetivos actuales.

---

## 2. Sin gaps pendientes

Listo para planning e implementation.
