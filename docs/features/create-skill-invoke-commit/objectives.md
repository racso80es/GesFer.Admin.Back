# Objetivos: create-skill-invoke-commit

**Rama:** feat/create-skill-invoke-commit  
**Proceso:** feature (creación de skill)  
**Persistencia:** paths.featurePath/create-skill-invoke-commit/ (Cúmulo)

## Objetivo

Crear una nueva **skill** que se encargue de las operaciones de **commit** en Git, siendo consumible por acciones, procesos y cualquier flujo de SddIA. La skill debe aceptar **parámetros adecuados** para evitar la generación de ficheros `.txt` intermedios (como `commit_cmd.txt` o ficheros para el body del PR).

## Alcance

### Entidad a crear

- **skill_id:** `invoke-commit` (propuesto)
- **Definición:** paths.skillsDefinitionPath/invoke-commit/ (spec.md, spec.json)
- **Cápsula:** paths.skillCapsules["invoke-commit"] (paths.skillsPath/invoke-commit/)
- **Implementación:** Ejecutable Rust (`.exe`) en `scripts/skills-rs/`

### Problema que resuelve

Actualmente, para realizar commits desde flujos automatizados (acciones, procesos) se usa **invoke-command** con `--command-file`, lo que obliga a:

1. Crear un fichero `.txt` con el comando (p. ej. `git add ...; git commit -m "..."`).
2. Para PRs, el body/descripción se construye desde rutas o se deja genérico.

La nueva skill **invoke-commit** debe permitir:

- **Commit:** Parámetros directos: `--message`, `--files` (o `--all`), sin fichero intermedio.
- **PR (opcional):** Parámetros `--title`, `--body` para el PR, sin fichero `.txt` para el comentario.

### Consumidores previstos

- **Acción finalize:** Commit atómico antes de push.
- **Acción spec, clarify, planning, etc.:** Persistir cambios con commit.
- **Procesos:** feature, bug-fix, create-tool, etc.
- **Agentes:** Tekton, Cúmulo, cualquier flujo SddIA.

## Ley aplicada

- **Ley COMANDOS:** Toda ejecución de comandos ha de pasar por skill, herramienta, acción o proceso (SddIA/norms/commands-via-skills-or-tools.md).
- **Ley GIT:** Ningún commit en master; mensajes convencionales (feat:, fix:, chore:, etc.).

## Fases del proceso (feature)

| Fase | Nombre | Estado |
|------|--------|--------|
| 0 | Preparar entorno | ✅ Rama feat/create-skill-invoke-commit creada |
| 1 | Objetivos | ✅ Este documento |
| 2 | Especificación | Pendiente |
| 3 | Clarificación | Pendiente |
| 4 | Planificación | Pendiente |
| 5 | Implementación | Pendiente |
| 6 | Ejecución | Pendiente |
| 7 | Validación | Pendiente |
| 8 | Finalizar | Pendiente |

## Referencias

- Contrato skills: SddIA/skills/skills-contract.json
- invoke-command (actual): SddIA/skills/invoke-command/spec.md
- finalizar-git: SddIA/skills/finalizar-git/spec.md
- create-tool (proceso análogo): SddIA/process/create-tool/spec.md
