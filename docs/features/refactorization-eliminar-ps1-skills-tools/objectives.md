# Refactorización: Eliminación de ejecutables PS1 en Skills y Tools

## Objetivo

Eliminar todos los archivos `.ps1` de las cápsulas de **skills** y **tools**, manteniendo únicamente los ejecutables `.exe` (Rust). Además, documentar que las futuras generaciones de estas entidades solo deben producir `.exe`.

## Alcance

### Entidades afectadas

**Skills:**
- `iniciar-rama` (paths.skillCapsules["iniciar-rama"])
- `finalizar-git` (paths.skillCapsules["finalizar-git"])
- `invoke-command` (paths.skillCapsules["invoke-command"])

**Tools:**
- `invoke-mysql-seeds` (paths.toolCapsules["invoke-mysql-seeds"])
- `postman-mcp-validation` (paths.toolCapsules["postman-mcp-validation"])
- `prepare-full-env` (paths.toolCapsules["prepare-full-env"])
- `run-tests-local` (paths.toolCapsules["run-tests-local"])
- `start-api` (paths.toolCapsules["start-api"])

### Archivos a eliminar

**Skills (14 archivos PS1):**
1. `scripts/skills/Iniciar-Rama.ps1` (raíz, legacy)
2. `scripts/skills/Invoke-Command.ps1` (raíz, legacy)
3. `scripts/skills/Merge-To-Master-Cleanup.ps1` (raíz, legacy)
4. `scripts/skills/Unificar-Rama.ps1` (raíz, legacy)
5. `scripts/skills/finalizar-git/Merge-To-Master-Cleanup.ps1`
6. `scripts/skills/finalizar-git/Push-And-CreatePR.ps1`
7. `scripts/skills/finalizar-git/Unificar-Rama.ps1`
8. `scripts/skills/iniciar-rama/Iniciar-Rama.ps1`
9. `scripts/skills/invoke-command/Invoke-Command.ps1`

**Tools (5 archivos PS1):**
1. `scripts/tools/invoke-mysql-seeds/Invoke-MySqlSeeds.ps1`
2. `scripts/tools/postman-mcp-validation/Postman-Mcp-Validation.ps1`
3. `scripts/tools/prepare-full-env/Prepare-FullEnv.ps1`
4. `scripts/tools/run-tests-local/Run-Tests-Local.ps1`
5. `scripts/tools/start-api/Start-Api.ps1`

**Excluir (instaladores, no parte de cápsulas):**
- `scripts/skills-rs/install.ps1`
- `scripts/tools-rs/install.ps1`

### Ejecutables EXE confirmados

**Skills:**
- ✅ `scripts/skills/iniciar-rama/bin/iniciar_rama.exe`
- ✅ `scripts/skills/finalizar-git/bin/merge_to_master_cleanup.exe`
- ✅ `scripts/skills/finalizar-git/bin/push_and_create_pr.exe`
- ✅ `scripts/skills/invoke-command/bin/invoke_command.exe`

**Tools:**
- ✅ `scripts/tools/invoke-mysql-seeds/bin/invoke_mysql_seeds.exe`
- ✅ `scripts/tools/prepare-full-env/bin/prepare_full_env.exe`
- ✅ `scripts/tools/start-api/start_api.exe`
- ⚠️ `postman-mcp-validation`: **Sin EXE confirmado**
- ⚠️ `run-tests-local`: **Sin EXE confirmado**

## Análisis de situación actual

Actualmente, las cápsulas de skills y tools contienen tanto archivos `.ps1` (scripts PowerShell) como `.exe` (binarios Rust). Esta duplicidad:

1. **Genera confusión** sobre cuál es la implementación canónica.
2. **Dificulta el mantenimiento** al requerir sincronización entre dos formatos.
3. **Contradice la constitución del proyecto** (SddIA/constitution.json → `configuration.skills_tools_implementation: "Rust"`).
4. **Aumenta la superficie de ataque** en auditorías de seguridad.

La implementación estándar definida en la constitución es **Rust** (binarios `.exe`), por lo que los `.ps1` son **legacy** y deben eliminarse.

## Ley aplicada

**Ley COMANDOS** (AGENTS.md): Las skills y tools deben tener una implementación clara y única. La constitución establece Rust como estándar (paths.skillsRustPath, paths.toolsRustPath).

**Ley SOBERANÍA** (AGENTS.md): SddIA (incluida la constitución) es la verdad absoluta.

## Impacto esperado

- **Reducción de complejidad:** Una única implementación por entidad.
- **Claridad para desarrolladores:** No hay ambigüedad sobre qué ejecutar.
- **Alineación con estándares:** Cumplimiento de la constitución del proyecto.
- **Menor deuda técnica:** Eliminación de código legacy.

## Rama

`feat/refactorization-eliminar-ps1-skills-tools`

## Fecha de inicio

2026-03-10
