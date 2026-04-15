1. Verify the project compile state
2. Identify async voids and `.Result` using auditor script
3. Analyze `List<T>` usage in Application/Api layers vs `IEnumerable<T>`
4. Analyze `class` vs `sealed record` / `record` for DTOs/Requests/Responses and `sealed class` for Handlers in the Application layer
5. Write the output to `docs/audits/AUDITORIA_YYYY_MM_DD.md`
6. Commit the report
7. Trigger `SddIA/process/correccion-auditorias` to execute corrections
