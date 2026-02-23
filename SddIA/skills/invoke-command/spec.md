# Skill: Invoke Command (Interceptor de ejecuciones de sistema)

**skill_id:** `invoke-command`

## Objetivo

Skill obligatoria para ejecutar comandos de sistema (git, dotnet, npm, pwsh, etc.) mediante un contrato implementado: telemetría, validación y registro por rama. Cualquier comando de sistema que el agente deba ejecutar ha de invocarse a través de esta skill; no hay excepciones.

## Alcance

- **Entrada:** Command (obligatorio) o --command-file &lt;ruta&gt; (lee el comando desde archivo; evita inyección en terminal), Contexto (default GesFer), Fase (Triaje | Analisis | Evaluacion | Marcado | Accion).
- **Salida:** Ejecución del comando con registro en docs/diagnostics/{branch}/execution_history.json; cumplimiento AC-001 y Protocolo Racso-Tormentosa.

## Reglas

- **MANDATORY:** Cualquier comando de sistema que el agente deba ejecutar ha de invocarse a través de esta skill.
- **Interface:** Usar el script o el .bat con Command (obligatorio) o --command-file, Contexto (default GesFer), Fase (Triaje|Analisis|Evaluacion|Marcado|Accion). El exe (Rust) acepta -Command/-Fase y --command-file.
- **Compliance:** AC-001 validación sintáctica; registro en docs/diagnostics/{branch}/execution_history.json; alineación Protocolo Racso-Tormentosa.
- **Scope:** Aplica siempre que el agente ejecute cualquier comando de sistema: sin excepción para git (status, add, commit, push, pull, branch, checkout), dotnet, npm, pwsh.

## Integración con la cápsula

**Implementación:** Cápsula en paths.skillCapsules[\"invoke-command\"] (Cúmulo). Launcher en la cápsula: Invoke-Command.bat (bin/invoke_command.exe si existe, si no Invoke-Command.ps1). Ejecutable por defecto en Rust (paths.skillsRustPath (Cúmulo)).

Uso (desde la raíz del repo):

```powershell
.\scripts\skills\invoke-command\Invoke-Command.ps1 -Command 'git status' -Fase Accion
.\scripts\skills\invoke-command\Invoke-Command.bat --command "git status" --fase Accion
.\scripts\skills\invoke-command\Invoke-Command.bat --command-file "docs\features\<nombre>\commit_cmd.txt" --fase Accion
```

El .bat invoca bin/invoke_command.exe (Rust) si existe; el exe acepta -Command/-Fase y --command-file.

**Rutas con --command-file:** (1) Con el .bat, usar **ruta absoluta** al archivo de comando (p. ej. `--command-file "c:\Proyectos\Repo\docs\features\X\commit_cmd.txt"`) para que el exe resuelva bien desde cualquier directorio. (2) Alternativa: ejecutar el exe directamente desde la **raíz del repo**; entonces las rutas relativas (p. ej. `docs\features\X\commit_cmd.txt`) se resuelven correctamente.

Prohibido ejecutar comandos directamente en el shell sin pasar por esta skill.

---
*Definición en paths.skillsDefinitionPath/invoke-command/ (contrato paths.skillsDefinitionPath/skills-contract.md).*
