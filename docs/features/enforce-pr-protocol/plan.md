# Plan: Enforce PR Protocol

## 1. Rust Skill Implementation
- **Goal:** Create the `verify-pr-protocol` executable.
- **File:** `scripts/skills-rs/src/bin/verify_pr_protocol.rs`
- **Steps:**
    1.  Update `scripts/skills-rs/Cargo.toml` to add `[[bin]]` entry for `verify_pr_protocol`.
    2.  Implement `verify_pr_protocol.rs`:
        - Use `std::process::Command` to run `dotnet build`.
        - Use `std::process::Command` to run `dotnet test`.
        - Use `std::process::Command` to run `scripts/validate-nomenclatura.ps1`.
        - Check exit codes. If any fail, print error and exit with 1.
    3.  Verify compilation with `cargo check`.

## 2. Protocol Definition (SddIA Norms)
- **Goal:** Document the protocol as a binding norm.
- **File:** `SddIA/norms/pr-acceptance-protocol.md`
- **Steps:**
    1.  Create the file listing mandatory checks.
    2.  Update `AGENTS.norms.md` to reference this new protocol.

## 3. Skill Registration (SddIA Skills)
- **Goal:** Register the skill in the system.
- **File:** `SddIA/skills/verify-pr-protocol/spec.md` & `spec.json`
- **Steps:**
    1.  Create `spec.md` describing usage and inputs/outputs.
    2.  Create `spec.json` with metadata.

## 4. Action Update (Finalize)
- **Goal:** Enforce protocol in AI workflow.
- **File:** `SddIA/actions/finalize/spec.md`
- **Steps:**
    1.  Modify the "Flujo de ejecución" section.
    2.  Insert a step before "Subir la rama (push)" called "Ejecutar Protocolo de Aceptación".
    3.  Specify that `verify-pr-protocol` must be run and passed.

## 5. CI/CD Integration (GitHub Actions)
- **Goal:** Enforce protocol on every PR.
- **File:** `.github/workflows/pr-validation.yml`
- **Steps:**
    1.  Create workflow file.
    2.  Define job: `build-and-verify`.
    3.  Steps: Checkout -> Setup Dotnet -> Setup Rust -> Build Skill -> Run Skill.

## 6. Local Enforcement (Cursor)
- **Goal:** Guide local development.
- **File:** `.cursorrules`
- **Steps:**
    1.  Append a rule: "Always run `cargo run --bin verify_pr_protocol` before pushing or creating a PR."
