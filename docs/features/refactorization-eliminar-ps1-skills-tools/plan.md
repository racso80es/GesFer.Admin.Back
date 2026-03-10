# Plan de Implementación: Eliminación de PS1 en Skills y Tools

**Versión:** 1.0.0  
**Fecha:** 2026-03-10  
**Proceso:** refactorization  
**Rama:** `feat/refactorization-eliminar-ps1-skills-tools`

---

## 1. Resumen Ejecutivo

Este plan define la secuencia de acciones para eliminar los archivos `.ps1` de las cápsulas de skills y tools, actualizar las especificaciones y procesos correspondientes, y validar el funcionamiento tras la refactorización.

## 2. Hallazgos de Verificación

### 2.1. Estado de Tools Pendientes

| Tool ID | EXE Confirmado | Decisión |
|---------|----------------|----------|
| `postman-mcp-validation` | ❌ **NO** | **NO eliminar** `.ps1`. Documentar como pendiente de migración a Rust. |
| `run-tests-local` | ❌ **NO** | **NO eliminar** `.ps1`. Documentar como pendiente de migración a Rust. |

**Conclusión:** Solo se eliminarán 3 archivos `.ps1` de tools (los que tienen `.exe` confirmado):
- `invoke-mysql-seeds/Invoke-MySqlSeeds.ps1`
- `prepare-full-env/Prepare-FullEnv.ps1`
- `start-api/Start-Api.ps1`

### 2.2. Totales Finales

| Categoría | Cantidad |
|-----------|----------|
| Skills PS1 (cápsulas) | 5 archivos |
| Skills PS1 (raíz legacy) | 4 archivos |
| Tools PS1 (eliminar) | 3 archivos |
| Tools PS1 (mantener) | 2 archivos |
| **TOTAL A ELIMINAR** | **12 archivos** |

---

## 3. Fases de Implementación

### FASE 1: Eliminación de Archivos PS1

#### 1.1. Eliminar PS1 de Skills (Cápsulas)

```powershell
# Cápsula: iniciar-rama
Remove-Item "scripts/skills/iniciar-rama/Iniciar-Rama.ps1" -Force

# Cápsula: finalizar-git
Remove-Item "scripts/skills/finalizar-git/Merge-To-Master-Cleanup.ps1" -Force
Remove-Item "scripts/skills/finalizar-git/Push-And-CreatePR.ps1" -Force
Remove-Item "scripts/skills/finalizar-git/Unificar-Rama.ps1" -Force

# Cápsula: invoke-command
Remove-Item "scripts/skills/invoke-command/Invoke-Command.ps1" -Force
```

**Archivos:** 5  
**Verificación:** Confirmar que los `.exe` existen antes de eliminar.

#### 1.2. Eliminar PS1 de Skills (Raíz Legacy)

```powershell
Remove-Item "scripts/skills/Iniciar-Rama.ps1" -Force
Remove-Item "scripts/skills/Invoke-Command.ps1" -Force
Remove-Item "scripts/skills/Merge-To-Master-Cleanup.ps1" -Force
Remove-Item "scripts/skills/Unificar-Rama.ps1" -Force
```

**Archivos:** 4  
**Nota:** Estos son duplicados legacy en la raíz de `scripts/skills/`.

#### 1.3. Eliminar PS1 de Tools (Solo con EXE)

```powershell
# Tool: invoke-mysql-seeds
Remove-Item "scripts/tools/invoke-mysql-seeds/Invoke-MySqlSeeds.ps1" -Force

# Tool: prepare-full-env
Remove-Item "scripts/tools/prepare-full-env/Prepare-FullEnv.ps1" -Force

# Tool: start-api
Remove-Item "scripts/tools/start-api/Start-Api.ps1" -Force
```

**Archivos:** 3  
**Excluir:** `postman-mcp-validation` y `run-tests-local` (sin `.exe`).

#### 1.4. Checkpoint de Eliminación

- [ ] 5 PS1 eliminados de cápsulas de skills
- [ ] 4 PS1 eliminados de raíz legacy de skills
- [ ] 3 PS1 eliminados de tools con `.exe`
- [ ] **TOTAL: 12 archivos eliminados**

---

### FASE 2: Actualización de Especificaciones de Skills

#### 2.1. Actualizar `SddIA/skills/iniciar-rama/spec.md`

Agregar sección "Implementación":

```markdown
## Implementación

**Formato:** Ejecutable Rust (`.exe`)  
**Ubicación:** `scripts/skills/iniciar-rama/bin/iniciar_rama.exe`  
**Fuente Rust:** `scripts/skills-rs/src/iniciar_rama.rs`  

**Estándar:** Solo se generan ejecutables `.exe`. No se deben crear archivos `.ps1`.

### Invocación

```powershell
& "scripts/skills/iniciar-rama/bin/iniciar_rama.exe" [argumentos]
```
```

#### 2.2. Actualizar `SddIA/skills/finalizar-git/spec.md`

Agregar sección "Implementación":

```markdown
## Implementación

**Formato:** Ejecutable Rust (`.exe`)  
**Ubicación:**
- `scripts/skills/finalizar-git/bin/merge_to_master_cleanup.exe`
- `scripts/skills/finalizar-git/bin/push_and_create_pr.exe`

**Fuente Rust:** `scripts/skills-rs/src/finalizar_git/`

**Estándar:** Solo se generan ejecutables `.exe`. No se deben crear archivos `.ps1`.

### Invocación

```powershell
# Merge y cleanup
& "scripts/skills/finalizar-git/bin/merge_to_master_cleanup.exe" [argumentos]

# Push y crear PR
& "scripts/skills/finalizar-git/bin/push_and_create_pr.exe" [argumentos]
```
```

#### 2.3. Actualizar `SddIA/skills/invoke-command/spec.md`

Agregar sección "Implementación":

```markdown
## Implementación

**Formato:** Ejecutable Rust (`.exe`)  
**Ubicación:** `scripts/skills/invoke-command/bin/invoke_command.exe`  
**Fuente Rust:** `scripts/skills-rs/src/invoke_command.rs`

**Estándar:** Solo se generan ejecutables `.exe`. No se deben crear archivos `.ps1`.

### Invocación

```powershell
& "scripts/skills/invoke-command/bin/invoke_command.exe" [argumentos]
```
```

#### 2.4. Checkpoint de Skills

- [ ] `iniciar-rama/spec.md` actualizado
- [ ] `finalizar-git/spec.md` actualizado
- [ ] `invoke-command/spec.md` actualizado

---

### FASE 3: Actualización de Especificaciones de Tools

#### 3.1. Actualizar `SddIA/tools/invoke-mysql-seeds/spec.md`

Agregar sección "Implementación":

```markdown
## Implementación

**Formato:** Ejecutable Rust (`.exe`)  
**Ubicación:** `scripts/tools/invoke-mysql-seeds/bin/invoke_mysql_seeds.exe`  
**Fuente Rust:** `scripts/tools-rs/src/invoke_mysql_seeds.rs`

**Estándar:** Solo se generan ejecutables `.exe`. No se deben crear archivos `.ps1`.

### Invocación

```powershell
& "scripts/tools/invoke-mysql-seeds/bin/invoke_mysql_seeds.exe" [argumentos]
```
```

#### 3.2. Actualizar `SddIA/tools/prepare-full-env/spec.md`

Agregar sección "Implementación":

```markdown
## Implementación

**Formato:** Ejecutable Rust (`.exe`)  
**Ubicación:** `scripts/tools/prepare-full-env/bin/prepare_full_env.exe`  
**Fuente Rust:** `scripts/tools-rs/src/prepare_full_env.rs`

**Estándar:** Solo se generan ejecutables `.exe`. No se deben crear archivos `.ps1`.

### Invocación

```powershell
& "scripts/tools/prepare-full-env/bin/prepare_full_env.exe" [argumentos]
```
```

#### 3.3. Actualizar `SddIA/tools/start-api/spec.md`

Agregar sección "Implementación":

```markdown
## Implementación

**Formato:** Ejecutable Rust (`.exe`)  
**Ubicación:** `scripts/tools/start-api/start_api.exe`  
**Fuente Rust:** `scripts/tools-rs/src/start_api.rs`

**Estándar:** Solo se generan ejecutables `.exe`. No se deben crear archivos `.ps1`.

### Invocación

```powershell
& "scripts/tools/start-api/start_api.exe" [argumentos]
```
```

#### 3.4. Documentar Tools Pendientes

##### 3.4.1. `SddIA/tools/postman-mcp-validation/spec.md`

Agregar sección "Estado de Implementación":

```markdown
## Estado de Implementación

**Formato actual:** Script PowerShell (`.ps1`)  
**Ubicación:** `scripts/tools/postman-mcp-validation/Postman-Mcp-Validation.ps1`

**Migración pendiente a Rust:**
- Estado: Pendiente
- Prioridad: Media
- Notas: Esta herramienta aún no ha sido migrada a Rust. Se mantiene temporalmente el script `.ps1`.

**Formato objetivo:** Ejecutable Rust (`.exe`)  
**Ubicación objetivo:** `scripts/tools/postman-mcp-validation/bin/postman_mcp_validation.exe`

**Estándar futuro:** Una vez migrado, solo existirá el ejecutable `.exe`. No se deberá mantener el `.ps1`.
```

##### 3.4.2. `SddIA/tools/run-tests-local/spec.md`

Agregar sección "Estado de Implementación":

```markdown
## Estado de Implementación

**Formato actual:** Script PowerShell (`.ps1`)  
**Ubicación:** `scripts/tools/run-tests-local/Run-Tests-Local.ps1`

**Migración pendiente a Rust:**
- Estado: Pendiente
- Prioridad: Media
- Notas: Esta herramienta aún no ha sido migrada a Rust. Se mantiene temporalmente el script `.ps1`.

**Formato objetivo:** Ejecutable Rust (`.exe`)  
**Ubicación objetivo:** `scripts/tools/run-tests-local/bin/run_tests_local.exe`

**Estándar futuro:** Una vez migrado, solo existirá el ejecutable `.exe`. No se deberá mantener el `.ps1`.
```

#### 3.5. Checkpoint de Tools

- [ ] `invoke-mysql-seeds/spec.md` actualizado
- [ ] `prepare-full-env/spec.md` actualizado
- [ ] `start-api/spec.md` actualizado
- [ ] `postman-mcp-validation/spec.md` documentado como pendiente
- [ ] `run-tests-local/spec.md` documentado como pendiente

---

### FASE 4: Actualización de Contratos

#### 4.1. Actualizar `SddIA/skills/skills-contract.json`

Agregar o actualizar el campo `implementation`:

```json
{
  "spec_version": "1.0.0",
  "contract_id": "skills-contract",
  "description": "Contrato de estructura y comportamiento de skills",
  "implementation": {
    "format": "Rust executable (.exe)",
    "location_pattern": "scripts/skills/{skill-id}/bin/{nombre}.exe",
    "source_location": "scripts/skills-rs/src/{nombre}.rs",
    "prohibited_formats": [".ps1", ".bat", ".sh"],
    "standard": "Solo se deben generar ejecutables .exe compilados desde Rust. Los scripts PowerShell (.ps1) están prohibidos en nuevas implementaciones."
  }
}
```

#### 4.2. Actualizar `SddIA/tools/tools-contract.json`

Agregar o actualizar el campo `implementation`:

```json
{
  "spec_version": "1.0.0",
  "contract_id": "tools-contract",
  "description": "Contrato de estructura y comportamiento de tools",
  "implementation": {
    "format": "Rust executable (.exe)",
    "location_pattern": "scripts/tools/{tool-id}/bin/{nombre}.exe",
    "source_location": "scripts/tools-rs/src/{nombre}.rs",
    "prohibited_formats": [".ps1", ".bat", ".sh"],
    "standard": "Solo se deben generar ejecutables .exe compilados desde Rust. Los scripts PowerShell (.ps1) están prohibidos en nuevas implementaciones.",
    "migration_note": "Las herramientas existentes con .ps1 deben migrar a .exe. Mientras no exista .exe, se mantiene temporalmente el .ps1 documentado como 'pendiente de migración'."
  }
}
```

#### 4.3. Checkpoint de Contratos

- [ ] `skills-contract.json` actualizado
- [ ] `tools-contract.json` actualizado

---

### FASE 5: Actualización de Procesos

#### 5.1. Actualizar `SddIA/process/create-tool/spec.md`

Localizar la sección "Entregables" y agregar/actualizar:

```markdown
### 5. Implementación

La herramienta debe implementarse **únicamente como ejecutable Rust** (`.exe`).

**Estructura esperada:**

```
scripts/tools/<tool-id>/
├── bin/
│   └── <tool-name>.exe        # Ejecutable Rust compilado (OBLIGATORIO)
├── manifest.json               # Metadatos de la herramienta
├── <tool-name>-config.json     # Configuración (si aplica)
└── <tool-name>.md              # Documentación de uso
```

**Fuente Rust:**

El código fuente Rust debe ubicarse en:
```
scripts/tools-rs/src/<tool-name>.rs
```

O, si es complejo:
```
scripts/tools-rs/src/<tool-name>/
├── main.rs
├── lib.rs
└── ...
```

**Prohibiciones:**

❌ **NO se deben crear:**
- Archivos `.ps1` (PowerShell scripts)
- Archivos `.bat` (Batch files)
- Scripts shell (`.sh`)
- Cualquier otro formato de script

✅ **Solo se debe generar:** Ejecutable `.exe` compilado desde Rust.

**Proceso de Compilación:**

1. Desarrollar en `scripts/tools-rs/src/<tool-name>.rs`
2. Compilar:
   ```powershell
   cargo build --release --manifest-path scripts/tools-rs/Cargo.toml
   ```
3. Copiar el `.exe` generado a `scripts/tools/<tool-id>/bin/<nombre>.exe`
4. Actualizar el índice: `scripts/tools/index.json`
5. Actualizar Cúmulo: `SddIA/agents/cumulo.paths.json` (campo `toolCapsules`)

**Migración desde .ps1:**

Si estás migrando una herramienta existente desde `.ps1` a `.exe`:
1. Implementar en Rust
2. Validar funcionamiento del `.exe`
3. Eliminar el `.ps1`
4. Actualizar la spec de la herramienta
```

#### 5.2. Checkpoint de Procesos

- [ ] `create-tool/spec.md` actualizado con estándar `.exe`

---

### FASE 6: Validación

#### 6.1. Validación de Compilación

```powershell
# Compilar el proyecto backend
dotnet build

# Verificar resultado
if ($LASTEXITCODE -ne 0) {
    Write-Error "Error de compilación. Revisar logs."
    exit 1
}
```

**Criterio de éxito:** Compilación sin errores.

#### 6.2. Validación de Skills

```powershell
# Skill: iniciar-rama
& "scripts/skills/iniciar-rama/bin/iniciar_rama.exe" --help

# Skill: invoke-command
& "scripts/skills/invoke-command/bin/invoke_command.exe" --help

# Skill: finalizar-git (merge)
& "scripts/skills/finalizar-git/bin/merge_to_master_cleanup.exe" --help

# Skill: finalizar-git (push)
& "scripts/skills/finalizar-git/bin/push_and_create_pr.exe" --help
```

**Criterio de éxito:** Cada comando retorna código de salida 0 o muestra ayuda correctamente.

#### 6.3. Validación de Tools

```powershell
# Tool: invoke-mysql-seeds
& "scripts/tools/invoke-mysql-seeds/bin/invoke_mysql_seeds.exe" --help

# Tool: prepare-full-env
& "scripts/tools/prepare-full-env/bin/prepare_full_env.exe" --help

# Tool: start-api
& "scripts/tools/start-api/start_api.exe" --help
```

**Criterio de éxito:** Cada comando retorna código de salida 0 o muestra ayuda correctamente.

#### 6.4. Checkpoint de Validación

- [ ] Compilación del backend exitosa
- [ ] Skills ejecutables funcionan correctamente
- [ ] Tools ejecutables funcionan correctamente
- [ ] No hay referencias rotas a archivos `.ps1` eliminados

---

### FASE 7: Documentación de Cierre

#### 7.1. Actualizar Evolution Log

Archivo: `docs/evolution/EVOLUTION_LOG.md`

Agregar entrada:

```markdown
## 2026-03-10 - Refactorización: Eliminación de PS1 en Skills y Tools

**Tipo:** Refactorización técnica  
**Rama:** `feat/refactorization-eliminar-ps1-skills-tools`  
**Documentación:** `docs/features/refactorization-eliminar-ps1-skills-tools/`

### Resumen

Se eliminaron los archivos `.ps1` (PowerShell scripts) de las cápsulas de skills y tools, manteniendo únicamente los ejecutables `.exe` (Rust) como implementación estándar.

### Archivos Eliminados

- **Skills:** 9 archivos `.ps1` (5 de cápsulas + 4 legacy en raíz)
- **Tools:** 3 archivos `.ps1` (solo los que tenían `.exe` confirmado)
- **Total:** 12 archivos

### Entidades Actualizadas

- **Skills:** `iniciar-rama`, `finalizar-git`, `invoke-command`
- **Tools:** `invoke-mysql-seeds`, `prepare-full-env`, `start-api`
- **Contratos:** `skills-contract.json`, `tools-contract.json`
- **Procesos:** `create-tool/spec.md`

### Pendientes de Migración

- **Tools sin `.exe`:** `postman-mcp-validation`, `run-tests-local`
- **Acción:** Documentadas como "pendiente de migración a Rust" en sus specs.

### Justificación

- **Alineación con constitución:** La implementación estándar es Rust (`.exe`).
- **Reducción de deuda técnica:** Eliminación de duplicidad.
- **Claridad:** Una única implementación por entidad.

### Referencias

- Spec: `docs/features/refactorization-eliminar-ps1-skills-tools/spec.md`
- Objectives: `docs/features/refactorization-eliminar-ps1-skills-tools/objectives.md`
```

#### 7.2. Checkpoint de Documentación

- [ ] Evolution Log actualizado

---

## 4. Checklist Final de Ejecución

### Eliminación (12 archivos)
- [ ] 5 PS1 de skills (cápsulas)
- [ ] 4 PS1 de skills (raíz legacy)
- [ ] 3 PS1 de tools (con `.exe`)

### Especificaciones (8 archivos)
- [ ] 3 specs de skills actualizadas
- [ ] 5 specs de tools actualizadas (3 migradas + 2 pendientes documentadas)

### Contratos (2 archivos)
- [ ] `skills-contract.json` actualizado
- [ ] `tools-contract.json` actualizado

### Procesos (1 archivo)
- [ ] `create-tool/spec.md` actualizado

### Validación
- [ ] Compilación exitosa
- [ ] Skills ejecutables funcionan
- [ ] Tools ejecutables funcionan

### Documentación
- [ ] Evolution Log actualizado

---

## 5. Tiempo Estimado

| Fase | Duración Estimada |
|------|-------------------|
| 1. Eliminación | 5 min |
| 2. Specs Skills | 15 min |
| 3. Specs Tools | 20 min |
| 4. Contratos | 10 min |
| 5. Procesos | 15 min |
| 6. Validación | 15 min |
| 7. Documentación | 10 min |
| **TOTAL** | **~90 min** |

---

## 6. Comandos de Rollback

Si se detectan problemas:

```powershell
# Revertir último commit
git revert HEAD

# O restaurar archivos específicos desde HEAD~1
git checkout HEAD~1 -- scripts/skills/
git checkout HEAD~1 -- scripts/tools/
```

---

**Fin del Plan de Implementación.**
