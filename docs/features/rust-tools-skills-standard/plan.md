# PLAN: Estándar Rust para tools y skills

**Proceso:** feature  
**Ruta (Cúmulo):** docs/features/rust-tools-skills-standard/  
**Especificación:** spec.md | **Clarificación:** clarify.md  

---

## 1. Fases

### Fase 1 — tools-rs Cargo.toml
- Añadir [[bin]] para invoke_mysql_seeds y prepare_full_env.
- Verificar: cargo build --release produce ambos .exe.

### Fase 2 — skills-rs push_and_create_pr
- Crear src/bin/push_and_create_pr.rs (git push, gh pr create).
- Añadir [[bin]] en Cargo.toml.
- Actualizar install.ps1: copiar push_and_create_pr.exe a finalizar-git/bin/.

### Fase 3 — finalizar-git launcher pre_pr
- Crear Push-And-CreatePR.bat: invocar bin/push_and_create_pr.exe si existe; fallback PS1.
- Actualizar manifest.json.

---

## 2. Orden de ejecución

1. Fase 1 — tools-rs  
2. Fase 2 — skills-rs  
3. Fase 3 — finalizar-git
