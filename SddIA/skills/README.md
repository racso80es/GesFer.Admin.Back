# Skills SddIA — Definición vs implementación

Este directorio es **paths.skillsDefinitionPath** (Cúmulo, `SddIA/agents/cumulo.json`). Separa la **capa de definición (SddIA)** de la **capa de implementación (scripts)** para desacoplar la arquitectura IA de los artefactos técnicos. **Invocación por agente:** JSON stdin/stdout según **SddIA/norms/capsule-json-io.md** (mismo criterio que tools).

## Estructura

| Ubicación | Contenido | Propósito |
|-----------|-----------|-----------|
| **SddIA/skills/** (este directorio) | Contrato global: `skills-contract.md`. Por skill: subcarpeta **&lt;skill-id&gt;/** con `spec.md` y `spec.json`. | **Definición:** qué hace el skill, contrato, entradas/salidas (`request`/`result`), flujo. Consumido por agentes y procesos. |
| **scripts/skills/** | Índice `index.json` (incluye `invocation_policy`: .bat usuarios, .exe IA/MCP/automatización, sin .ps1 como interfaz) y, por skill con ejecutable, cápsula **&lt;skill-id&gt;/** (manifest, .exe en raíz, .bat opcional humano, doc). | **Implementación:** ejecutables Rust y documentación. En **v2** no hay `bin/` ni `.ps1` como contrato. Ruta: **paths.skillCapsules[&lt;skill-id&gt;]** (Cúmulo). |

La **raíz del path de implementación** la indica Cúmulo (**paths.skillsPath**, **paths.skillCapsules**). No se usan rutas literales en la definición.

## Definición por skill (SddIA/skills/&lt;skill-id&gt;/)

Cada skill tiene en este directorio una carpeta con:

- **spec.md** — Especificación legible: objetivo, entradas, salidas, flujo, reglas. Idioma: es-ES.
- **spec.json** — Metadatos machine-readable. Si el skill tiene implementación ejecutable, debe incluir **implementation_path_ref**: referencia a la ruta de implementación en Cúmulo (ej. `paths.skillCapsules.<skill-id>`).

## Implementación por defecto: Rust

**Las implementaciones por defecto de los scripts de skills (igual que en tools) han de ser en Rust.** Los ejecutables se construyen en paths.skillsRustPath (Cúmulo) y se copian a la **raíz** de cada cápsula. El **agente** invoca el `.exe` con el JSON definido en **SddIA/norms/capsule-json-io.md**. El `.bat` es **opcional** y solo para uso **humano**.

## Listado de skills

| skill_id | Descripción breve | Cápsula (paths.skillCapsules) |
| :--- | :--- | :--- |
| iniciar-rama | Crea rama feat/ o fix/ actualizada con master; inicio de acción. | iniciar-rama |
| finalizar-git | Aceptar PR a master, unificar, eliminar rama, volver a master. | finalizar-git |
| invoke-command | Interceptor de comandos de sistema (git, dotnet, npm, pwsh). | invoke-command |
| git-operations | Uso seguro de Git (ramas feat/fix, commits convencionales). | — |
| documentation | Estándares SSOT y gestión de documentación. | — |
| filesystem-ops | Operaciones de archivo seguras (PowerShell). | — |
| dotnet-development | Estándares .NET (build, test, logging). | — |
| frontend-build | Build Next.js (Product, Admin) fallback. | — |
| security-audit | Auditoría y hooks pre-commit/pre-push. | — |

## Referencias

- Contrato global: `skills-contract.md` (en este directorio; v2 + capsule-json-io).
- Cúmulo: `SddIA/agents/cumulo.json` → **paths.skillsDefinitionPath**, **paths.skillsPath**, **paths.skillCapsules**, **paths.skillsIndexPath**.
- Índice de implementaciones: **paths.skillsIndexPath** (paths.skillsIndexPath, Cúmulo).
