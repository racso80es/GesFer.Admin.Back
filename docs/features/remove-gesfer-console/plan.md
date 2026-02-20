1. *Create Rust Tool for Tests*
   - Create `scripts/skills-rs/src/bin/run_tests.rs` which executes `dotnet test src/GesFer.Admin.Back.sln` and returns the appropriate exit code.
   - Update `scripts/skills-rs/Cargo.toml` to include the new binary `run_tests`.
   - Ensure the new tool is executable and builds correctly.

2. *Update `pr-skill.sh`*
   - Modify `scripts/skills/pr-skill.sh` to invoke the new Rust tool (`cargo run --bin run_tests ...`) instead of `GesFer.Console`.

3. *Remove `GesFer.Console` References from Documentation*
   - Remove or update references to `GesFer.Console` in:
     - `SddIA/actions/spec.md`
     - `SddIA/actions/clarify.md`
     - `SddIA/actions/planning.md`
     - `SddIA/agents/*.json`
     - `scripts/skills/pr-skill.md`
     - `docs/DeudaTecnica/DT-2025-001-MissingConsoleTool.md` (Update status to reflect removal of the tool concept).
     - `docs/features/standardize-nomenclature/clarify.md` (Update status).

4. *Complete pre commit steps*
   - Ensure proper testing, verification, review, and reflection are done.

5. *Submit the change*
   - Submit the changes with a descriptive commit message.