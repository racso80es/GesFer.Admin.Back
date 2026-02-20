# Skill: Invoke Command (Interceptor de ejecuciones de sistema)

**skill_id:** `invoke-command`

## Objetivo

Skill obligatoria para ejecutar comandos de sistema (git, dotnet, npm, pwsh, etc.) mediante un contrato implementado: telemetría, validación y registro por rama. Cualquier comando de sistema que el agente deba ejecutar ha de invocarse a través de esta skill; no hay excepciones.

## Alcance

- **Entrada:** Command (obligatorio), Contexto (default GesFer), Fase (Triaje | Analisis | Evaluacion | Marcado | Accion).
- **Salida:** Ejecución del comando con registro en docs/diagnostics/{branch}/execution_history.json; cumplimiento AC-001 y Protocolo Racso-Tormentosa.

## Reglas

- **MANDATORY:** Cualquier comando de sistema que el agente deba ejecutar ha de invocarse a través de esta skill.
- **Interface:** Usar el script con parámetros Command (obligatorio), Contexto (default GesFer), Fase (Triaje|Analisis|Evaluacion|Marcado|Accion).
- **Compliance:** AC-001 validación sintáctica; registro en docs/diagnostics/{branch}/execution_history.json; alineación Protocolo Racso-Tormentosa.
- **Scope:** Aplica siempre que el agente ejecute cualquier comando de sistema: sin excepción para git (status, add, commit, push, pull, branch, checkout), dotnet, npm, pwsh.

## Integración con la cápsula

**Implementación:** Cápsula en paths.skillCapsules[\"invoke-command\"] (Cúmulo). Launcher en la cápsula: Invoke-Command.bat (bin/invoke_command.exe si existe, si no Invoke-Command.ps1). Ejecutable por defecto en Rust (scripts/skills-rs).

Uso (desde la raíz del repo):

```powershell
.\scripts\skills\invoke-command\Invoke-Command.ps1 -Command 'git status' -Fase Accion
# o vía .bat si existe
.\scripts\skills\invoke-command\Invoke-Command.bat "git status" Accion
```

Prohibido ejecutar comandos directamente en el shell sin pasar por esta skill.

---
*Definición en paths.skillsDefinitionPath/invoke-command/ (contrato SddIA/skills/skills-contract.md).*
