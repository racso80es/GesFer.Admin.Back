---
document_id: reproducir-skills-en-otros-entornos-sddia
document_type: guia_portabilidad
norm_ref: SddIA/norms/paths-via-cumulo.md
contract_ref: SddIA/skills/skills-contract.md
json_io_ref: SddIA/norms/capsule-json-io.md
cumulo_ref: SddIA/agents/cumulo.json
paths_contract_ref: SddIA/agents/cumulo.paths.json
idioma: es-ES
version: "1.0.0"
descripcion: >-
  Guía para reproducir skills ejecutables (Rust + cápsula + Cúmulo) en otro repositorio o entorno SddIA.
---

# Reproducir skills en otros entornos SddIA

Esta guía describe **qué copiar, qué registrar y cómo validar** para que las skills con binario (patrón v2) funcionen en **otro repo** que siga el mismo ecosistema SddIA. Las rutas lógicas son siempre las del **Cúmulo** (`SddIA/agents/cumulo.json` → `pathsContract` → `SddIA/agents/cumulo.paths.json`); no fijar rutas literales en procesos sin pasar por Cúmulo.

## 1. Prerrequisitos del entorno destino

| Requisito | Uso |
|-----------|-----|
| **Windows 11 + PowerShell 7+** | Convención del proyecto; scripts de instalación (`install.ps1`). |
| **Rust (cargo) + toolchain C++** (si aplica) | Compilar `paths.skillsRustPath` (p. ej. `scripts/skills-rs/`). |
| **Git** | Skills que invocan `git`. |
| **GitHub CLI (`gh`)** | Solo para la skill `git-create-pr`. |

## 2. Dependencias de código compartido

Los binarios de skills en este repo dependen del crate local **`gesfer-capsule`** (ruta relativa desde `paths.skillsRustPath`, p. ej. `scripts/gesfer-capsule/`). Al portar:

- Copiar **toda** la carpeta `scripts/skills-rs/` **y** `scripts/gesfer-capsule/` (o el equivalente en el destino si ya existe con la misma API).
- Ajustar en `Cargo.toml` de skills la ruta `gesfer-capsule = { path = "..." }` si la estructura de carpetas difiere.

## 3. Checklist por skill ejecutable

Para cada `skill-id` con `.exe`:

1. **Definición (SddIA)**  
   - Carpeta `paths.skillsDefinitionPath/<skill-id>/` con **`spec.md`** (frontmatter YAML + cuerpo).  
   - Opcional: `spec.json` si el contrato del destino lo exige (`implementation_path_ref` → `paths.skillCapsules.<skill-id>`).

2. **Código Rust**  
   - Fichero `paths.skillsRustPath/src/bin/<nombre_snake>.rs` (convención: nombre del bin alineado con el `.exe`).  
   - Entrada `[[bin]]` en `paths.skillsRustPath/Cargo.toml`.

3. **Cápsula**  
   - Carpeta `paths.skillCapsules[<skill-id>]` (típicamente `paths.skillsPath/<skill-id>/`):  
     - `manifest.json`  
     - Documentación `.md`  
     - **`<nombre>.exe` en la raíz de la cápsula** (sin subcarpeta `bin/`, salvo excepciones documentadas como `sddia-evolution`).  
     - `.bat` opcional solo para humanos.

4. **Índice**  
   - Entrada en `paths.skillsIndexPath` (`scripts/skills/index.json`): `skillId`, `path`, `manifest`, `executable`, `launcher_bat` si aplica.

5. **Cúmulo (SSOT de rutas)**  
   - En `SddIA/agents/cumulo.paths.json`, clave `paths.skillCapsules["<skill-id>"]` → ruta relativa a la cápsula.

6. **Evolution SddIA** (si el destino aplica la norma)  
   - Cualquier alta/modificación bajo `./SddIA/` debe registrarse según `SddIA/norms/sddia-evolution-sync.md` y `SddIA/evolution/evolution_contract.md`.

## 4. Orden recomendado de portado

1. Copiar/ajustar `gesfer-capsule` + `skills-rs` (y `Cargo.lock` si se desea reproducibilidad).  
2. Copiar carpetas de cápsulas **sin** los `.exe` (o con ellos si son del mismo target triple).  
3. Ejecutar en el destino el script de instalación que haga **`cargo build --release`** y **copie** cada `.exe` a la raíz de su cápsula (patrón: `scripts/skills-rs/install.ps1` — mantener la tabla `capsulesRoot` / `capsulesBin` al día).  
4. Fusionar `cumulo.paths.json` e `index.json`.  
5. Añadir definiciones en `SddIA/skills/<skill-id>/`.  
6. Registrar evolution si aplica.  
7. Smoke test con envelope JSON v2 (ver §6).

## 5. Build e instalación de ejecutables

En un clon del repo destino, desde `paths.skillsRustPath`:

```powershell
Set-Location .\scripts\skills-rs
.\install.ps1
```

- Compila en modo **release**.  
- Copia cada `target\release\<bin>.exe` a `scripts/skills/<cápsula>/` según el mapeo del script.

Si añades una skill nueva, **debes** añadir su par `{ exe, capsule }` en `install.ps1` para que el `.exe` llegue a la cápsula.

## 6. Contrato de invocación (envelope JSON v2)

Referencia única: **`SddIA/norms/capsule-json-io.md`** (`schema_version: "2.0"`).

**Petición (stdin o variable de entorno `GESFER_CAPSULE_REQUEST`):**

- `meta`: `schema_version`, `entity_kind` (`"skill"`), `entity_id` (kebab-case, coincide con el skill), `token` si el contrato lo exige.  
- `request`: objeto libre definido en el `spec.md` de cada skill.

**Respuesta (stdout):**

- `meta`, `success`, `exitCode`, `message`, `feedback`, `result`, opcional `duration_ms`.  
- Coherencia: `exitCode === 0` si y solo si `success === true`.

**Modo sin stdin fiable:** usar `GESFER_CAPSULE_REQUEST` o `GESFER_SKIP_STDIN=1` + argumentos CLI documentados en cada bin (ver norma §4.1).

## 7. Skills Git del arsenal (referencia rápida)

| skill_id | Binario (raíz cápsula) | Comandos externos |
|----------|-------------------------|---------------------|
| git-workspace-recon | `git_workspace_recon.exe` | `git` |
| git-branch-manager | `git_branch_manager.exe` | `git` |
| git-save-snapshot | `git_save_snapshot.exe` | `git` |
| git-sync-remote | `git_sync_remote.exe` | `git` |
| git-tactical-retreat | `git_tactical_retreat.exe` | `git` |
| git-create-pr | `git_create_pr.exe` | `gh` |

Convención de **request** (camelCase en JSON): p. ej. `git-save-snapshot` usa `commitMessage`; `git-branch-manager` usa `branchName`, `create`; `git-tactical-retreat` exige `confirmDestructive: true` si `hardReset: true` (Visión Zero / acciones destructivas). Detalle campo a campo: cada `SddIA/skills/<skill-id>/spec.md`.

## 8. Verificación mínima en el destino

1. Existe el `.exe` en la ruta indicada por `paths.skillsIndexPath` / `manifest.json`.  
2. Invocación con un JSON válido devuelve **una línea** JSON en stdout y código de salida igual a `exitCode`.  
3. Para skills Git: ejecutar en un repositorio Git de prueba; para `git-create-pr`: `gh auth status` correcto.

## 9. Referencias cruzadas

- Contrato global de skills: `SddIA/skills/skills-contract.md`  
- Listado disparador `#Skill`: `SddIA/norms/interaction-triggers.md`  
- Proceso de alta de skill: `SddIA/process/create-skill/spec.md`  
- Portar el **proceso** `create-skill` a otro repo: `SddIA/process/create-skill/reproducir-create-skill-en-otros-entornos-sddia.md`  
- Patrón documentación de features (tareas `create-skill-*`): `SddIA/norms/features-documentation-pattern.md` → `docs/features/features-contract.md`

---

*Documento de portabilidad. Mantener alineado con Cúmulo y capsule-json-io al evolucionar el contrato.*
