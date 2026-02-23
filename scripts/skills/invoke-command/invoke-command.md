# Skill invoke-command — Cápsula de implementación

**skillId:** invoke-command  
**Ruta canónica:** Cúmulo paths.skillCapsules["invoke-command"] (scripts/skills/invoke-command/)

## Uso

Cualquier comando de sistema que el agente deba ejecutar ha de invocarse a través de esta skill. Desde la raíz del repo:

```powershell
.\scripts\skills\invoke-command\Invoke-Command.ps1 -Command 'git status' -Fase Accion
.\scripts\skills\invoke-command\Invoke-Command.bat --command "git status" --fase Accion
.\scripts\skills\invoke-command\Invoke-Command.bat --command-file "docs\features\mi-feature\commit_cmd.txt" --fase Accion
```

Parámetros: **Command** (obligatorio) o **--command-file** &lt;ruta&gt; (lee el comando desde un archivo; evita inyección de trailers en entornos automatizados), Contexto (default GesFer), Fase (Triaje|Analisis|Evaluacion|Marcado|Accion). Si existe `bin/invoke_command.exe` (Rust), el .bat lo invoca. Definición: SddIA/skills/invoke-command/spec.md.
