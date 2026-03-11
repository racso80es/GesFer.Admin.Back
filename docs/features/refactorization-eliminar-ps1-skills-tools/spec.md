# SPEC: Eliminación de ejecutables PS1 en Skills y Tools

**Versión:** 1.0.0  
**Fecha:** 2026-03-10  
**Proceso:** refactorization  
**Rama:** `feat/refactorization-eliminar-ps1-skills-tools`

---

## 1. Resumen Ejecutivo

Esta refactorización elimina los archivos `.ps1` (PowerShell scripts) de las cápsulas de **skills** y **tools**, manteniendo únicamente los ejecutables `.exe` (Rust) como implementación estándar. Se actualizan las especificaciones de las entidades afectadas para documentar que futuras generaciones solo deben producir `.exe`.

## 2. Contexto

### 2.1. Situación actual

Las cápsulas de skills y tools contienen:
- Archivos `.ps1`: Implementaciones PowerShell (legacy).
- Archivos `.exe`: Binarios Rust (estándar actual según constitución).

### 2.2. Problema

1. **Duplicidad de implementación:** Dos formatos para la misma funcionalidad.
2. **Falta de claridad:** No está explícito cuál es la implementación canónica.
3. **Incumplimiento de estándares:** La constitución (`SddIA/constitution.json`) establece Rust como implementación estándar.
4. **Deuda técnica:** Código legacy que debe mantenerse sincronizado.

### 2.3. Decisión

Eliminar todos los `.ps1` de las cápsulas de skills y tools, manteniendo solo `.exe`. Documentar en las especificaciones de las entidades que solo se debe generar `.exe` en futuras implementaciones.

## 3. Entidades Afectadas

### 3.1. Skills

| Skill ID | Cápsula (Cúmulo) | Archivos PS1 a eliminar | EXE confirmado |
|----------|------------------|-------------------------|----------------|
| `iniciar-rama` | paths.skillCapsules["iniciar-rama"] | `iniciar-rama/Iniciar-Rama.ps1` | ✅ `bin/iniciar_rama.exe` |
| `finalizar-git` | paths.skillCapsules["finalizar-git"] | `finalizar-git/Merge-To-Master-Cleanup.ps1`<br>`finalizar-git/Push-And-CreatePR.ps1`<br>`finalizar-git/Unificar-Rama.ps1` | ✅ `bin/merge_to_master_cleanup.exe`<br>✅ `bin/push_and_create_pr.exe` |
| `invoke-command` | paths.skillCapsules["invoke-command"] | `invoke-command/Invoke-Command.ps1` | ✅ `bin/invoke_command.exe` |

**Legacy en raíz de skills (eliminar):**
- `scripts/skills/Iniciar-Rama.ps1`
- `scripts/skills/Invoke-Command.ps1`
- `scripts/skills/Merge-To-Master-Cleanup.ps1`
- `scripts/skills/Unificar-Rama.ps1`

### 3.2. Tools

| Tool ID | Cápsula (Cúmulo) | Archivos PS1 a eliminar | EXE confirmado |
|---------|------------------|-------------------------|----------------|
| `invoke-mysql-seeds` | paths.toolCapsules["invoke-mysql-seeds"] | `invoke-mysql-seeds/Invoke-MySqlSeeds.ps1` | ✅ `bin/invoke_mysql_seeds.exe` |
| `prepare-full-env` | paths.toolCapsules["prepare-full-env"] | `prepare-full-env/Prepare-FullEnv.ps1` | ✅ `bin/prepare_full_env.exe` |
| `start-api` | paths.toolCapsules["start-api"] | `start-api/Start-Api.ps1` | ✅ `start_api.exe` |
| `postman-mcp-validation` | paths.toolCapsules["postman-mcp-validation"] | `postman-mcp-validation/Postman-Mcp-Validation.ps1` | ⚠️ **NO CONFIRMADO** |
| `run-tests-local` | paths.toolCapsules["run-tests-local"] | `run-tests-local/Run-Tests-Local.ps1` | ⚠️ **NO CONFIRMADO** |

### 3.3. Archivos excluidos (instaladores)

**NO eliminar** (son instaladores de las herramientas Rust, no parte de cápsulas):
- `scripts/skills-rs/install.ps1`
- `scripts/tools-rs/install.ps1`

## 4. Acciones Requeridas

### 4.1. Fase de Verificación

**Acción 4.1.1:** Verificar existencia de EXE para tools con estado ⚠️
- `postman-mcp-validation`
- `run-tests-local`

**Criterio de decisión:**
- Si existe `.exe`: Proceder con eliminación del `.ps1`.
- Si NO existe `.exe`: Documentar como pendiente de migración a Rust; NO eliminar `.ps1` hasta que exista el `.exe`.

### 4.2. Fase de Eliminación

**Acción 4.2.1:** Eliminar archivos PS1 de skills

Archivos a eliminar:
```
scripts/skills/Iniciar-Rama.ps1
scripts/skills/Invoke-Command.ps1
scripts/skills/Merge-To-Master-Cleanup.ps1
scripts/skills/Unificar-Rama.ps1
scripts/skills/finalizar-git/Merge-To-Master-Cleanup.ps1
scripts/skills/finalizar-git/Push-And-CreatePR.ps1
scripts/skills/finalizar-git/Unificar-Rama.ps1
scripts/skills/iniciar-rama/Iniciar-Rama.ps1
scripts/skills/invoke-command/Invoke-Command.ps1
```

**Acción 4.2.2:** Eliminar archivos PS1 de tools (solo si existe EXE)

Archivos confirmados para eliminación:
```
scripts/tools/invoke-mysql-seeds/Invoke-MySqlSeeds.ps1
scripts/tools/prepare-full-env/Prepare-FullEnv.ps1
scripts/tools/start-api/Start-Api.ps1
```

Archivos pendientes de verificación:
```
scripts/tools/postman-mcp-validation/Postman-Mcp-Validation.ps1  # ⚠️ Verificar EXE
scripts/tools/run-tests-local/Run-Tests-Local.ps1                # ⚠️ Verificar EXE
```

### 4.3. Fase de Actualización de Especificaciones

**Acción 4.3.1:** Actualizar specs de skills

Para cada skill en `SddIA/skills/<skill-id>/spec.md`:
- Agregar en sección "Implementación" (o crear si no existe):
  ```markdown
  ## Implementación
  
  **Formato:** Ejecutable Rust (`.exe`)  
  **Ubicación:** `scripts/skills/<skill-id>/bin/<nombre>.exe`  
  **Estándar:** Solo se generan ejecutables `.exe`. No se deben crear archivos `.ps1`.
  ```

Skills afectadas:
- `SddIA/skills/iniciar-rama/spec.md`
- `SddIA/skills/finalizar-git/spec.md`
- `SddIA/skills/invoke-command/spec.md`

**Acción 4.3.2:** Actualizar specs de tools

Para cada tool en `SddIA/tools/<tool-id>/spec.md`:
- Agregar en sección "Implementación" (o crear si no existe):
  ```markdown
  ## Implementación
  
  **Formato:** Ejecutable Rust (`.exe`)  
  **Ubicación:** `scripts/tools/<tool-id>/bin/<nombre>.exe`  
  **Estándar:** Solo se generan ejecutables `.exe`. No se deben crear archivos `.ps1`.
  ```

Tools afectadas:
- `SddIA/tools/invoke-mysql-seeds/spec.md`
- `SddIA/tools/prepare-full-env/spec.md`
- `SddIA/tools/start-api/spec.md`
- `SddIA/tools/postman-mcp-validation/spec.md` (si aplica)
- `SddIA/tools/run-tests-local/spec.md` (si aplica)

**Acción 4.3.3:** Actualizar contrato de skills

Archivo: `SddIA/skills/skills-contract.json`

Verificar o añadir la siguiente clave en el contrato:

```json
{
  "implementation": {
    "format": "Rust executable (.exe)",
    "location_pattern": "scripts/skills/{skill-id}/bin/{nombre}.exe",
    "prohibited_formats": [".ps1", ".bat", ".sh"],
    "standard": "Solo se deben generar ejecutables .exe compilados desde Rust."
  }
}
```

**Acción 4.3.4:** Actualizar contrato de tools

Archivo: `SddIA/tools/tools-contract.json`

Verificar o añadir la siguiente clave en el contrato:

```json
{
  "implementation": {
    "format": "Rust executable (.exe)",
    "location_pattern": "scripts/tools/{tool-id}/bin/{nombre}.exe",
    "prohibited_formats": [".ps1", ".bat", ".sh"],
    "standard": "Solo se deben generar ejecutables .exe compilados desde Rust."
  }
}
```

### 4.4. Fase de Actualización de Procesos

**Acción 4.4.1:** Actualizar proceso `create-tool`

Archivo: `SddIA/process/create-tool/spec.md`

Añadir o reforzar en la sección de "Entregables":
```markdown
### Implementación

La herramienta debe implementarse **únicamente como ejecutable Rust** (`.exe`).

**Prohibido:**
- Archivos `.ps1`
- Archivos `.bat`
- Scripts shell (`.sh`)

**Estructura esperada:**
```
scripts/tools/<tool-id>/
├── bin/
│   └── <tool-name>.exe    # Ejecutable Rust compilado
├── src/                   # Fuentes Rust (opcional, puede estar en tools-rs)
└── README.md
```

**Acción 4.4.2:** Actualizar proceso `create-skill` (si existe spec)

Aplicar la misma restricción que en `create-tool` para skills.

## 5. Criterios de Aceptación

### 5.1. Eliminación

- [ ] Todos los archivos `.ps1` listados han sido eliminados.
- [ ] Los archivos `install.ps1` de `skills-rs` y `tools-rs` permanecen intactos.

### 5.2. Especificaciones actualizadas

- [ ] Todas las specs de skills incluyen la sección "Implementación" con el estándar `.exe`.
- [ ] Todas las specs de tools incluyen la sección "Implementación" con el estándar `.exe`.
- [ ] Los contratos `skills-contract.json` y `tools-contract.json` reflejan el estándar `.exe`.

### 5.3. Procesos actualizados

- [ ] El proceso `create-tool` documenta que solo se deben generar `.exe`.
- [ ] El proceso `create-skill` (si existe) documenta que solo se deben generar `.exe`.

### 5.4. Compilación y funcionamiento

- [ ] El proyecto compila sin errores (`dotnet build`).
- [ ] Las skills pueden ejecutarse mediante sus `.exe` (verificar al menos `iniciar-rama` y `invoke-command`).
- [ ] Las tools pueden ejecutarse mediante sus `.exe` (verificar al menos `start-api` y `prepare-full-env`).

## 6. Impacto

### 6.1. Sistemas afectados

- **Cápsulas de skills:** `iniciar-rama`, `finalizar-git`, `invoke-command`.
- **Cápsulas de tools:** `invoke-mysql-seeds`, `prepare-full-env`, `start-api`, (y potencialmente `postman-mcp-validation`, `run-tests-local`).
- **Especificaciones SddIA:** Archivos `spec.md` y contratos JSON.
- **Procesos SddIA:** `create-tool`, potencialmente `create-skill`.

### 6.2. Riesgos

| Riesgo | Probabilidad | Impacto | Mitigación |
|--------|--------------|---------|------------|
| Tools sin `.exe` quedan inoperativas | Media | Alto | Verificar existencia de `.exe` antes de eliminar `.ps1`. |
| Referencias a `.ps1` en documentación | Baja | Bajo | Búsqueda global de referencias tras eliminación. |
| Scripts que invocan `.ps1` directamente | Media | Medio | Auditar logs y código que pueda invocar `.ps1`. |

## 7. Plan de Rollback

Si se detectan problemas tras la eliminación:

1. **Revertir commit:** `git revert <commit-hash>`.
2. **Restaurar archivos:** Los `.ps1` estarán en el historial de Git.
3. **Documentar el problema:** Crear issue con el motivo del rollback.

## 8. Métricas de Éxito

- **Reducción de archivos:** -14 archivos `.ps1` (9 skills + 5 tools).
- **Claridad documental:** 100% de specs actualizadas con estándar `.exe`.
- **Compilación exitosa:** 0 errores tras refactorización.
- **Funcionamiento de herramientas:** 100% de skills y tools ejecutables funcionando.

## 9. Notas Técnicas

### 9.1. Ubicación de ejecutables

- **Skills:** `scripts/skills/<skill-id>/bin/<nombre>.exe`
- **Tools:** `scripts/tools/<tool-id>/bin/<nombre>.exe` o `scripts/tools/<tool-id>/<nombre>.exe`

### 9.2. Fuentes Rust

Las fuentes Rust están en:
- `scripts/skills-rs/` (fuentes de skills)
- `scripts/tools-rs/` (fuentes de tools)

Los binarios compilados se copian a las carpetas `bin/` dentro de cada cápsula.

### 9.3. Instaladores

Los archivos `install.ps1` en `skills-rs` y `tools-rs` son scripts de instalación/compilación, **NO son parte de las cápsulas**. Deben mantenerse.

---

**Fin de la especificación.**
