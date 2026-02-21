# Implementation: Enforce PR Protocol

This document lists the files to be created or modified to implement the PR Acceptance Protocol.

## 1. Rust Skill (Protocol Enforcer)
- **`scripts/skills-rs/Cargo.toml`**: Add `[[bin]]` entry for `verify_pr_protocol`.
- **`scripts/skills-rs/src/bin/verify_pr_protocol.rs`**: The main executable.
    - Logic:
        1.  Execute `scripts/validate-nomenclatura.ps1`.
        2.  Execute `dotnet build src/GesFer.Admin.Back.sln`.
        3.  Execute `dotnet test src/GesFer.Admin.Back.sln`.
    - Dependencies: `std::process::Command`, `std::env`.

## 2. SddIA Norms & Skills
- **`SddIA/norms/pr-acceptance-protocol.md`**: New file defining the protocol.
- **`AGENTS.norms.md`**: Update to reference `pr-acceptance-protocol.md`.
- **`SddIA/skills/verify-pr-protocol/spec.md`**: New file. Spec for the skill.
- **`SddIA/skills/verify-pr-protocol/spec.json`**: New file. Metadata for the skill.

## 3. SddIA Action (Finalize)
- **`SddIA/actions/finalize/spec.md`**: Update to include the mandatory execution step of `verify-pr-protocol`.

## 4. CI/CD (GitHub Actions)
- **`.github/workflows/pr-validation.yml`**: New workflow file.
    - Triggers: `pull_request` [branches: master].
    - Steps: Checkout, Setup .NET, Setup Rust, Build Skill, Run Skill.

## 5. Local Enforcement (Cursor)
- **`.cursorrules`**: Append rule requiring `verify-pr-protocol` execution.

## 6. Documentation
- **`docs/features/enforce-pr-protocol/implementation.json`**: Tracking file.
