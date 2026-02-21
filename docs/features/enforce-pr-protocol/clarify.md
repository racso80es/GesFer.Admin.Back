# Clarification: Enforce PR Protocol

## 1. Questions & Answers

### Q1: Execution Environment for `verify-pr-protocol`
- **Context:** The skill will run in:
    1.  **Local Dev (Cursor/Windows):** Needs PowerShell/dotnet/cargo.
    2.  **CI (GitHub Actions/Linux):** Needs bash/dotnet/cargo.
    3.  **AI Agent (Jules/Container):** Needs correct shell.
- **Answer:** The Rust binary must be cross-platform compatible. It should use `std::process::Command` with platform-specific shells (`powershell -Command` on Windows, `sh -c` on Linux) or call executables directly where possible.
- **Refinement:** The `invoke-command` skill already handles this logic. We can reuse or replicate its pattern.

### Q2: Impact on Developer Velocity (Local Tests)
- **Context:** Running *all* tests locally before every PR might be slow.
- **Answer:** Safety > Speed. The protocol demands "absolute certainty".
- **Refinement:** The default behavior is to run everything. Future iterations could add a `--fast` flag (e.g., run only unit tests, skip integration), but for V1, full validation is mandatory as per the "Force Protocol" requirement.

### Q3: Integration with `finalize` Action
- **Context:** The `finalize` action currently has a step "Push and Create PR".
- **Answer:** We must inject a new step *before* this: "Execute Protocol".
- **Refinement:** The `finalize` spec needs to be updated to explicitly call `verify-pr-protocol`. If it fails, `finalize` must abort.

### Q4: GitHub Token for PR Check
- **Context:** GitHub Actions needs to report status checks.
- **Answer:** Standard `GITHUB_TOKEN` is sufficient for reporting check runs or simply failing the workflow (which blocks the merge if the branch is protected).
- **Refinement:** We will rely on the workflow exit code. If the workflow fails (due to `verify-pr-protocol` returning non-zero), the PR check fails.

## 2. Assumptions
1.  **Rust Toolchain:** Available in all environments (Local, CI, Container). If not, the skill execution will fail (fail-safe).
2.  **Dotnet Toolchain:** Available in all environments.
3.  **Nomenclature Script:** `scripts/validate-nomenclatura.ps1` exists and is the source of truth for naming conventions.

## 3. Implementation Details (Refined)
- **Binary Name:** `verify_pr_protocol`
- **CLI Args:**
    - `--target <path>` (Optional, defaults to current repo root).
    - `--skip-tests` (DEBUG ONLY - strictly controlled, maybe requires a flag file). **Decision:** NO skip flags for V1.
- **Output:**
    - Stdout: Human-readable logs of what is being checked.
    - Stderr: Error details.
    - Exit Code: 0 (Pass), 1 (Fail).
