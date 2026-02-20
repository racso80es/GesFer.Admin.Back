# Skill invoke-command — Cápsula de implementación

**skillId:** invoke-command  
**Ruta canónica:** Cúmulo paths.skillCapsules["invoke-command"] (scripts/skills/invoke-command/)

## Uso

Cualquier comando de sistema que el agente deba ejecutar ha de invocarse a través de esta skill. Desde la raíz del repo:

```powershell
.\scripts\skills\invoke-command\Invoke-Command.ps1 -Command 'git status' -Fase Accion
.\scripts\skills\invoke-command\Invoke-Command.ps1 -Command 'dotnet build' -Fase Accion
```

Parámetros: Command (obligatorio), Contexto (default GesFer), Fase (Triaje|Analisis|Evaluacion|Marcado|Accion). Si existe `bin/invoke_command.exe` (Rust), el .bat lo invoca. Definición: SddIA/skills/invoke-command/spec.md y spec.json.
