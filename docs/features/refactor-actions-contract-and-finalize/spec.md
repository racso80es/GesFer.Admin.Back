---
feature_name: refactor-actions-contract-and-finalize
created: "2026-05-01"
base:
  - objectives.md
scope:
  in_scope:
    - Contrato actions-contract.md (norma innegociable de orquestación).
    - Renombrado acción finalize → finalize-process y saneamiento de referencias en SddIA y difusión.
    - Eliminación de referencias a ejecución directa de scripts en la spec de cierre.
  out_scope:
    - Cambio de nombres de artefactos históricos en cada feature cerrada del repo.
    - Nueva skill; se reutiliza Git S+ existente.
functional_requirements:
  - id: FR-01
    text: El contrato de acciones declara que ninguna acción puede invocar binarios del SO, intérpretes ni scripts (.ps1, .bat, .sh) salvo a través de skills o tools que encapsulen esa ejecución según norma.
  - id: FR-02
    text: Existe la acción finalize-process (kebab-case) en paths.actionsPath/finalize-process/ con spec.md coherente; la carpeta finalize deja de ser canónica.
  - id: FR-03
    text: Todos los procesos en paths.processPath que referencien finalize como acción relacionada o en narrativa de fase 8 referencian finalize-process.
  - id: FR-04
    text: interaction-triggers y listados de acciones en .cursor/rules reflejan finalize-process y describen cierre con Git S+.
non_functional_requirements:
  - id: NFR-01
    text: Sin referencias rotas a paths.actionsPath/finalize/ en documentación canónica activa.
  - id: NFR-02
    text: Registro en SddIA/evolution/ cumpliendo contrato al mutar ./SddIA/.
touchpoints:
  - path: SddIA/actions/actions-contract.md
    change: Ampliar restricciones y lista de action_id (finalize-process).
  - path: SddIA/actions/finalize-process/spec.md
    change: Nueva ubicación; contenido derivado de finalize sin scripts directos.
  - path: SddIA/process/*/spec.md
    change: related_actions y texto de fase 8.
  - path: SddIA/norms/interaction-triggers.md
    change: Tabla #Action y menciones a finalize-process.
  - path: .cursor/rules/*.mdc
    change: Listados y enlaces a la acción de cierre.
validation_criteria:
  - Búsqueda global en SddIA y .cursor/rules sin `paths.actionsPath/finalize/` como ruta canónica activa (salvo notas históricas explícitas si se mantienen).
  - Carpeta SddIA/actions/finalize-process/ presente; finalize/ eliminada o vaciada según plan de migración.
  - Contrato actions-contract.md menciona explícitamente prohibición de ejecución OS/scripts desde acciones.
---

# Especificación técnica

## Contexto

Hoy el contrato `actions-contract.md` define estructura y consumidores, pero no cierra el hueco semántico: una acción podría interpretarse como “documento que autoriza” llamadas directas a PowerShell. La Ley COMANDOS ya exige skills/tools; esta feature hace la regla **explícita a nivel de acción** y renombra el cierre para reflejar que se cierra el **proceso** (feature/bug-fix/refactor), no un “finalize” genérico ambiguo.

## Comportamiento deseado de finalize-process

- **Disparadores lingüísticos (documentales):** frases como «proceso finalizado», «tarea finalizada», «cierre del ciclo», «fase 8», además de los existentes de publicación («subir rama») cuando el contexto sea cierre formal post-validación.
- **Orquestación obligatoria:** precondiciones (rama no troncal, `validacion.md` pass donde aplique), `verify-pr-protocol` si sigue siendo requisito, actualización de Evolution Logs de producto, luego **git-sync-remote** → **git-create-pr** vía cápsulas; **sin** `.ps1` / `.bat` / comandos git literales en la spec como mecanismo de implementación.
- **Salidas:** mismas que la acción finalize actual (rama en origin, PR, logs), con nomenclatura actualizada.

## Migración de contenido

- Mover/renombrar `SddIA/actions/finalize/spec.md` → `SddIA/actions/finalize-process/spec.md` con frontmatter `action_id: finalize-process`.
- Actualizar `contract_ref` y referencias internas (`validacion.md` en lugar de `validacion.json` donde aún aparezca el nombre legacy en el texto).
- Eliminar menciones a `finalize.json` como artefacto obligatorio si contradice el patrón actual solo-md en carpeta de tarea.

## Riesgos

- **Hiperónimos:** agentes y documentación histórica que digan “finalize”; mitigar con búsqueda `finalize` acotada a `actions`, `process`, `norms`, `.cursor`.
- **Enlaces externos:** PRs o issues que apunten a la ruta antigua; aceptable; en repo, preferir redirección textual “antes finalize, ahora finalize-process”.
