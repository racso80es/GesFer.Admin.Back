# Specification: Enforce PR Acceptance Protocol

## 1. Overview
The goal is to enforce a strict protocol for Pull Request (PR) acceptance, ensuring all changes pass mandatory checks (compilation, tests, nomenclature) before merge. This protocol applies to AI agents (Jules via `finalize`), local developers (Cursor via `.cursorrules`/hooks), and CI/CD (GitHub Actions).

## 2. Components

### 2.1. New Skill: `verify-pr-protocol` (Rust)
- **Purpose:** Centralize all validation logic into a single executable to guarantee consistency across environments.
- **Location:** `scripts/skills-rs/src/bin/verify_pr_protocol.rs`.
- **Functionality:**
    1.  **Nomenclature Check:** Executes `scripts/validate-nomenclatura.ps1`. Failing this blocks everything.
    2.  **Compilation (.NET):** Executes `dotnet build src/GesFer.Admin.Back.sln`.
    3.  **Testing (.NET):** Executes `dotnet test src/GesFer.Admin.Back.sln`.
    4.  **SddIA Compliance:** Checks for the presence of `objectives.md` in the current task context (if applicable/detectable).
- **Exit Code:** Returns `0` on success, non-zero on failure.

### 2.2. SddIA Action Update: `finalize`
- **Location:** `SddIA/actions/finalize/spec.md` and implementation.
- **Change:** The `finalize` action must invoke the `verify-pr-protocol` skill **before** pushing code or creating a PR.
- **Logic:**
    - If `verify-pr-protocol` fails: The action aborts. The agent must fix issues before retrying.
    - If succeeds: The action proceeds to `git push` and `gh pr create`.

### 2.3. GitHub Action Workflow
- **Location:** `.github/workflows/pr-validation.yml`
- **Trigger:** `on: [pull_request]`
- **Steps:**
    1.  Checkout code.
    2.  Setup .NET 8.
    3.  Setup Rust (latest stable).
    4.  Compile `verify-pr-protocol` skill (`cargo build --release`).
    5.  Execute `verify-pr-protocol`.
- **Constraint:** Status check must be required for merging protected branches.

### 2.4. Local Enforcement (Cursor)
- **Location:** `.cursorrules`
- **Change:** Add a rule explicitly stating: "Before submitting a PR, you MUST execute the 'verify-pr-protocol' skill."
- **Note:** This acts as a "soft" enforcement for the AI/User in the editor, while GitHub Actions provides the "hard" gate.

### 2.5. Protocol Definition (SddIA Norm)
- **Location:** `SddIA/norms/pr-acceptance-protocol.md`
- **Content:** Explicitly lists the checks performed by `verify-pr-protocol`.
- **Integration:** Referenced in `AGENTS.norms.md`.

## 3. Data Flow
1.  **Agent/User** initiates PR creation.
2.  **Environment** (Local/CI) triggers `verify-pr-protocol`.
3.  **Skill** runs checks:
    - `validate-nomenclatura.ps1` -> Pass/Fail
    - `dotnet build` -> Pass/Fail
    - `dotnet test` -> Pass/Fail
4.  **Result:**
    - Pass -> PR created/merged.
    - Fail -> Process aborted, errors reported.

## 4. Security & Compliance
- **Karma2Token:** The skill execution must be logged/traceable if possible, adhering to `Karma2Token` principles (though for now it's a local/CI check).
- **Zero Trust:** We assume all code is broken until verified by this protocol.

## 5. Success Criteria
- [ ] `verify-pr-protocol` binary exists and works.
- [ ] `finalize` action fails if checks fail.
- [ ] GitHub PRs are automatically checked.
- [ ] `.cursorrules` instructs usage of the protocol.
