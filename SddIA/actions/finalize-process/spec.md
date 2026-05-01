---
action_id: finalize-process
contract_ref: actions-contract.md
flow_steps:
- Precondiciones (rama, objectives.md, validacion.md)
- Commits atómicos (git-save-snapshot cuando aplique)
- Evolution Logs (producto y auditoría según norma)
- Protocolo verify-pr-protocol (skill/herramienta autorizada)
- Git S+ obligatorio: git-sync-remote → git-create-pr
- Auditoría
- 'post_pr: limpieza post-merge vía skill/herramienta autorizada'
inputs:
- Carpeta de la tarea (paths.featurePath o paths.fixPath, Cúmulo)
- Rama feat/ o fix/
outputs:
- Rama publicada en origin
- Evolution Logs actualizados
- Pull Request creado
- finalize-process.md opcional en carpeta de tarea (frontmatter YAML + Markdown)
skill_ref: git-sync-remote, git-create-pr
---

# Action: finalize-process (cierre de proceso / tarea)

## Propósito

La acción **finalize-process** cierra el **proceso de tarea** (feature, bug-fix, refactorización, etc.): asegura commits atómicos en la rama, actualiza los Evolution Logs, **sincroniza la rama con el remoto** y crea el Pull Request. Solo debe ejecutarse cuando la validación ha pasado; en caso contrario, debe advertir o bloquear.

**Disparadores conceptuales (lenguaje natural):** «proceso finalizado», «tarea finalizada», «cierre del ciclo», «fase 8», además de peticiones de publicación («subir rama», «subir al remoto») cuando el contexto sea el **cierre formal** tras validación.

**Comportamiento obligatorio:** el ejecutor debe completar la secuencia Git S+ **git-sync-remote → git-create-pr**. Sin este paso el cierre no está completo. Toda orquestación es **solo** vía skills y tools registradas (Ley COMANDOS); **prohibido** documentar o implementar esta acción como invocación directa de `.ps1`, `.bat`, `pwsh`, `git` u otros binarios del SO fuera de las cápsulas.

## Principio

- **No tocar la rama troncal:** El trabajo permanece en feat/ o fix/; el merge es vía PR.
- **Documentación como SSOT:** PR y logs referencian la carpeta de la tarea (paths.featurePath o paths.fixPath según Cúmulo).
- **Auditoría:** La finalización queda registrada donde corresponda (Evolution Logs, ACCESS_LOG, etc.).

## Entradas

- **Carpeta de la tarea:** Ruta desde Cúmulo (ej. paths.featurePath/&lt;nombre_feature&gt;/).
  - Se espera `objectives.md` y, para cierre seguro, `validacion.md` con resultado global pass (frontmatter/cuerpo según contrato de la tarea).
- **Rama actual:** feat/ o fix/ con cambios consolidados en commits.

## Salidas

- **Rama sincronizada:** Publicada en `origin` mediante **git-sync-remote**.
- **Evolution Logs:** Entradas en paths.evolutionPath y convenciones del proyecto.
- **Pull Request:** Creado con **git-create-pr**, cuerpo enlazando `objectives.md` y `validacion.md`.
- **Opcional:** `finalize-process.md` en la carpeta de la tarea (metadatos de cierre: pr_url, branch, timestamp).

## Skills de referencia (Git S+)

- **git-save-snapshot** (hitos previos al cierre)
- **git-sync-remote**
- **git-create-pr**

Regla: el ejecutor **no** invoca `git push`, `gh pr create` ni scripts sueltos; solo cápsulas autorizadas.

### Ejecución (Git S+)

1. **git-sync-remote**
2. **git-create-pr** (objectives + validacion en el cuerpo)

## Flujo de ejecución (propuesto)

1. **Precondiciones:** Rama no es troncal; existe `objectives.md`; `validacion.md` con pass (o criterio explícito del proyecto).
2. **Commits atómicos:** Agrupar cambios pendientes con **git-save-snapshot** si aplica.
3. **verify-pr-protocol:** Invocar mediante skill/herramienta autorizada (sin `cargo run` ni shell directo). Si falla, **abortar**.
4. **Evolution Logs:** Actualizar según normas de producto y SddIA/evolution si hubo mutación bajo `SddIA/`.
5. **git-sync-remote** y **git-create-pr**.
6. **Opcional:** Persistir `finalize-process.md` en la carpeta de la tarea.
7. **Auditoría:** Registrar en paths.auditsPath cuando aplique.
8. **Post-PR:** Solo skills/herramientas autorizadas.

## Implementación técnica

La acción se **materializa** únicamente como orquestación de skills/tools (capsule-json-io v2). No hay «implementación» en forma de scripts de cierre en esta definición.

## Integración con agentes

- **Tekton:** Ejecuta el cierre vía Git S+ y herramientas registradas.
- **QA Judge:** Exige evidencia de validación antes del cierre.
- **Cúmulo:** Valida rutas canónicas y SSOT.

## Dependencias con otras acciones

- **validate:** Debe existir evidencia de validación antes del cierre seguro.
- **Procesos (feature, bug-fix, …):** **finalize-process** es la acción de cierre de ciclo documentada en la fase final cuando el proceso la referencia.

---
*Definición de la acción finalize-process (cierre de proceso/tarea). Sustituye a la acción histórica `finalize` en paths.actionsPath.*
