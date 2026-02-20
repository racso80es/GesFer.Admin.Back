# Skill: .NET Core Development Standards

**skill_id:** `dotnet-development`

## Objetivo

Estándares para desarrollo C#/.NET: build, testing y buenas prácticas.

## Reglas

- **Error Handling:** Use try/catch in upper layers (Controllers/Commands).
- **Logging:** Use structured logging (e.g. _logger.LogInformation(\"Item {Id}\", id)).
- **Clean Code:** No 'TODO', no commented-out code.
- **Build:** Code must compile with 'dotnet build' before any commit.
- **Testing:** Run 'dotnet test' to ensure no regressions.

## Comandos

- build: dotnet build
- test: dotnet test
- run: dotnet run --project <project_path>

---
*Definición en paths.skillsDefinitionPath/dotnet-development/ (contrato paths.skillsDefinitionPath/skills-contract.md).*
