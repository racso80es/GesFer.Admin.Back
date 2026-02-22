# IMPL: Estándar Rust para tools y skills

**Plan:** plan.md | **SPEC:** spec.md  

---

## Fase 1 — tools-rs

### 1.1 Cargo.toml
Añadir [[bin]] para invoke_mysql_seeds y prepare_full_env (después de gesfer_manager).

---

## Fase 2 — skills-rs

### 2.1 src/bin/push_and_create_pr.rs
Crear binario: clap --branch, --persist, --title, --base. Ejecutar git push origin <branch>; si gh existe, gh pr create. Exit 0/1.

### 2.2 Cargo.toml
Añadir [[bin]] name = "push_and_create_pr".

### 2.3 install.ps1
Añadir `@{ exe = "push_and_create_pr"; capsule = "finalizar-git" }` en $capsules. Destino: finalizar-git/bin/push_and_create_pr.exe.

---

## Fase 3 — finalizar-git

### 3.1 Push-And-CreatePR.ps1
Al inicio, si existe bin/push_and_create_pr.exe, invocarlo con parámetros convertidos (-Persist -> --persist, etc.) y exit; si no, continuar lógica PS1 existente.
