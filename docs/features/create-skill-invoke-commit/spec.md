# Especificación: Skill invoke-commit

**task_id:** create-skill-invoke-commit  
**skill_id (propuesto):** invoke-commit  
**Rama:** feat/create-skill-invoke-commit  
**Persistencia:** paths.featurePath/create-skill-invoke-commit/ (Cúmulo)

## 1. Objetivo

Crear la skill **invoke-commit** que centralice las operaciones de **commit** en Git, consumible por acciones, procesos y flujos SddIA. La skill acepta **parámetros directos** (--message, --files, --all, etc.) para evitar la generación de ficheros `.txt` intermedios.

## 2. Alcance funcional

### 2.1 Operaciones soportadas

| Operación | Descripción | Parámetros clave |
|-----------|-------------|------------------|
| **commit** | git add + git commit | --message, --files, --all, --scope |

*Nota: pr-create no se incluye (decisión C1: funcionalidad atómica). Los PRs siguen en finalizar-git.*

### 2.2 Entradas (commit)

| Parámetro | Tipo | Obligatorio | Descripción |
|-----------|------|-------------|-------------|
| --message, -m | string | Sí | Mensaje del commit (Conventional Commits recomendado). |
| --files | string | Condicional | Lista de rutas separadas por coma (ej. `"a.md,b.json"`). Si no se especifica, ver --all. |
| --all, -a | flag | Condicional | Añadir todos los cambios (git add -A). Excluyente con --files. |
| --scope | string | No | Scope para Conventional Commits (ej. start-api, invoke-command). |
| --type | string | No | Tipo: feat, fix, chore, docs, refactor. Por defecto: feat. |
| --fase | string | No | Fase para telemetría (Triaje, Analisis, Accion, etc.). |
| --contexto | string | No | Contexto para registro (default: GesFer). |

### 2.3 Salidas

- **commit:** exitCode 0 si commit exitoso; registro en docs/diagnostics/{branch}/execution_history.json.

## 3. Relación con invoke-command

- **invoke-command:** Comandos genéricos (git, dotnet, npm, pwsh). Requiere --command o --command-file.
- **invoke-commit:** Especializada en commits. Parámetros directos, sin fichero.

Para operaciones de **commit**, invoke-commit se invoca directamente; **no es necesario** usar invoke-command (decisión C4). Ambas son skills del mismo nivel; invoke-commit es la vía preferida para commits.

## 4. Implementación

- **Formato:** Ejecutable Rust (`.exe`)
- **Ubicación:** paths.skillCapsules["invoke-commit"]/bin/invoke_commit.exe
- **Fuente:** scripts/skills-rs/src/bin/invoke_commit.rs
- **Launcher:** Invoke-Commit.bat en la cápsula

## 5. Criterios de aceptación

**invoke-commit:**
- [ ] Commit con --message y --files (comma-separated) o --all funciona sin fichero.
- [ ] Mensaje sigue Conventional Commits cuando se especifica --type y --scope.
- [ ] Registro en execution_history.json (telemetría).
- [ ] Cumple skills-contract.json (spec.md, spec.json, manifest, .exe, .bat).

**push_and_create_pr (--body, --body-file):**
- [ ] --body acepta descripción directa del PR sin fichero.
- [ ] --body-file acepta ruta a fichero con body largo.
- [ ] Precedencia: body-file > body > persist > rama.

## 6. Entregable adicional: push_and_create_pr (finalizar-git)

**Decisión C2 (A+C):** Extender push_and_create_pr con:

| Parámetro | Tipo | Descripción |
|-----------|------|-------------|
| --body | string | Descripción del PR (texto directo). Prioridad sobre --persist si ambos se pasan. |
| --body-file | string | Ruta a fichero con el body (para cuerpos largos; evita límites de línea de comandos). Prioridad: body-file > body > persist. |

**Orden de precedencia para el body del PR:** `--body-file` > `--body` > `--persist` > `Rama: {rama}`.

## 7. Dependencias

- Git instalado y configurado.
- Karma2Token (contexto de seguridad).
