# Skill: Security Audit Automation & Git Hooks

**skill_id:** `security-audit` (security-audit-automation)

## Objetivo

Especificación para auditorías de seguridad automatizadas en commit y PR/push: validación de token del Auditor Process y cumplimiento de gobernanza. Los hooks referencian scripts en la cápsula cuando exista implementación Windows (PowerShell); los .sh son legacy y en entorno Windows se recomienda equivalente en paths.skillCapsules (Cúmulo) o cápsula.

## Requisitos

- SEC-AUDIT-001: All git commits must trigger the pre-commit hook (commit-skill).
- SEC-AUDIT-002: All git pushes must trigger the pre-push hook (pr-skill).
- SEC-AUDIT-003: Any failure in the audit script (exit code != 0) must block the git operation.
- SEC-AUDIT-004: Execution must be logged to paths.auditsPath + paths.accessLogFile regardless of outcome.
- SEC-AUDIT-005: Auditor Process Token must be validated before allowing any action.

## Implementación (hooks)

- pre-commit: Validar commit token y ejecutar tests unitarios.
- pre-push: Validar push token y ejecutar tests de integración/E2E.

Ruta de scripts: según Cúmulo; en Windows usar PowerShell o binarios Rust en cápsula si se implementan.

---
*Definición en paths.skillsDefinitionPath/security-audit/ (contrato paths.skillsDefinitionPath/skills-contract.md).*
