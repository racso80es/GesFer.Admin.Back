# Norma: Git solo vía skill, herramienta, acción o proceso

**Fuente:** SddIA/norms. Aplicable a todo agente o IA que opere en el repositorio.

## Principio

**La IA nunca debe ejecutar comandos git directamente** en la shell (ni `git status`, `git add`, `git commit`, `git push`, `git pull`, `git branch`, `git checkout`, etc.). Cualquier operación git ha de realizarse **siempre** a través de al menos uno de los siguientes canales:

- **Skill (Git S+):** paths.skillCapsules.git-workspace-recon, paths.skillCapsules.git-branch-manager, paths.skillCapsules.git-save-snapshot, paths.skillCapsules.git-sync-remote, paths.skillCapsules.git-tactical-retreat, paths.skillCapsules.git-create-pr. Contrato: paths.skillsDefinitionPath/\<skill-id\>/ (archivo .md con frontmatter YAML).
- **Skill (otros):** p. ej. paths.skillCapsules.invoke-commit (commits parametrizados). **No** usar paths.skillCapsules.invoke-command ni terminal nativa para **Git**; ver **SddIA/norms/git-operations.md** (Ley de Hierro). Contrato: paths.skillsDefinitionPath/\<skill-id\>/ (archivo .md con frontmatter YAML).
- **Herramienta (tool):** definida en paths.toolsDefinitionPath, implementación en paths.toolCapsules[tool-id].
- **Acción:** definida en paths.actionsPath (spec, planning, implementation, execution, validate, finalize-process, etc.). Las acciones pueden orquestar skills o herramientas que ejecuten git.
- **Proceso:** definido en paths.processPath (feature, bug-fix, create-tool, create-principle, etc.). Los procesos invocan acciones y skills; la IA sigue el proceso en lugar de lanzar git por su cuenta.

## Justificación

- **Trazabilidad:** Las skills/herramientas/acciones registran y auditan las operaciones (p. ej. Karma2Token, historiales por rama). Los bypass con shell para Git destruyen esa trazabilidad; ver **git-operations.md**.
- **Consistencia:** Mismo flujo para humanos y agentes; no hay “atajos” que eviten la validación.
- **Cumplimiento:** Ramas feat/fix, commits convencionales y principio nomenclatura se aplican cuando el flujo pasa por los artefactos definidos en SddIA.

## Aplicación

- Si la IA necesita hacer una operación git (crear rama, commit, push, etc.), debe **proponer** o **ejecutar** la invocación mediante la skill, herramienta, acción o proceso correspondiente (p. ej. “Ejecutar git-branch-manager” o “Ejecutar git-sync-remote”), **nunca** escribiendo `git ...` directamente en la terminal.
- Los agentes que ejecutan código (p. ej. Tekton) ya tienen esta restricción en su contrato (execution_contract, constraints); esta norma la extiende a **cualquier** contexto de IA en el proyecto.
- **Referencia en protocolo:** AGENTS.md, ley GIT, esta norma y **SddIA/norms/git-operations.md**. Skills: suite Git S+ (git-workspace-recon, git-branch-manager, git-save-snapshot, git-sync-remote, git-tactical-retreat, git-create-pr) + invoke-commit cuando aplique. **Prohibido** invoke-command para operaciones Git.
