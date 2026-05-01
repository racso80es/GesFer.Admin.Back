---
action_id: finalize
contract_ref: actions-contract.md
flow_steps:
- Precondiciones
- Commits atómicos
- Evolution Logs
- Git S+: git-sync-remote → git-create-pr
- finalize.json
- Auditoría
- 'post_pr: limpieza post-merge vía skill/herramienta autorizada'
inputs:
- Carpeta feature (Cúmulo)
- Rama feat/ o fix/
outputs:
- Rama en origin
- Evolution Logs
- PR
- finalize.md opcional (frontmatter YAML + Markdown)
skill_ref: git-sync-remote, git-create-pr
---

# Action: Finalize

## Propósito

La acción **finalize** (finalizar) cierra el ciclo de la feature: asegura commits atómicos en la rama, actualiza los Evolution Logs, **sincroniza la rama con el remoto** y crea el Pull Request. Solo debe ejecutarse cuando la validación ha pasado; en caso contrario, debe advertir o bloquear. **Comportamiento obligatorio:** al realizar finalize, el ejecutor debe incluir la secuencia Git S+ **git-sync-remote → git-create-pr**; sin este paso el cierre no está completo. Proporciona trazabilidad y cierre formal alineado con las Leyes Universales (no commit en rama troncal, documentación en paths.featurePath según Cúmulo).

## Principio

- **No tocar master:** Todo el trabajo permanece en la rama feat/ o fix/; el merge se hace vía PR, no con commit directo en master.
- **Documentación como SSOT:** La descripción del PR y los logs hacen referencia a la carpeta de la feature (ej. paths.featurePath/<nombre_feature>/).
- **Auditoría:** Toda finalización queda registrada en Evolution Logs y, opcionalmente, en auditoría.

## Entradas

- **Carpeta de la feature:** Ruta obtenida de Cúmulo (ej. paths.featurePath/<nombre_feature>/).
  - Se espera que existan al menos: `objectives.md`, y preferiblemente `validacion.md` con resultado global pass.
- **Rama actual:** Rama feat/ o fix/ con todos los cambios ya commiteados (o la acción puede incluir un paso de “commit pendientes” según criterio del proyecto).

## Salidas

- **Rama sincronizada (Git S+):** La rama actual debe quedar sincronizada y publicada en `origin` mediante **git-sync-remote**; es una salida obligatoria de finalize antes de considerar el PR creado.
- **Evolution Logs actualizados:**
  - paths.evolutionPath + paths.evolutionLogFile (raíz docs: docs/EVOLUTION_LOG.md según proyecto): una línea con formato `[YYYY-MM-DD] [feat/<nombre>] [Descripción breve del resultado.] [Estado].`
  - paths.evolutionPath + paths.evolutionLogFile: una sección con fecha, título de la feature, resumen de acción/alcance/resultado y referencia a la carpeta de la feature (Cúmulo)/objectives.md.
- **Pull Request:** Creado con **git-create-pr**, con descripción que enlace a la documentación de la feature (ej. paths.featurePath/<nombre_feature>/).
- **Opcional:** Referencia al PR o estado en validacion.json o finalize.json de la carpeta de la feature (Cúmulo) (ej. URL del PR, timestamp de cierre).

## Skills de referencia (Git S+)

La acción finalize centraliza las interacciones con Git mediante la suite **Git S+**:

- **git-save-snapshot** (commits atómicos por hitos, si aplica)
- **git-sync-remote** (publicación/sincronización segura con remoto)
- **git-create-pr** (creación del PR enlazando artefactos de la tarea)

Regla: el ejecutor **no** ejecuta `git push`, `git branch`, `gh pr create` ni otros comandos de sistema directamente; todo va vía skill/herramienta/acción/proceso (Ley COMANDOS).

### Ejecución (Git S+)

Para ejecutar la acción finalize (publicación y PR), el ejecutor debe invocar, en orden:

1. **git-sync-remote**
2. **git-create-pr** (cuerpo del PR enlazando `objectives.md` y `validacion.md`)

## Flujo de ejecución (propuesto)

1. **Comprobación de precondiciones:**
   - Rama actual no es `master`.
   - Existe objectives.md en la carpeta de la feature (Cúmulo).
   - Existe validacion.md en la carpeta de la feature (Cúmulo) y su resultado global es pass (o se permite finalize con advertencia si el proyecto lo define).
2. **Commits atómicos:** Si hay cambios sin commitear, el agente puede agruparlos en commits atómicos según convención (un commit por ítem lógico o por fase).
3. **Ejecutar Protocolo de Aceptación (verify-pr-protocol):**
   - **OBLIGATORIO:** Antes de sincronizar remoto o crear PR, invocar el protocolo `verify-pr-protocol` mediante skill/herramienta autorizada (sin `cargo run` directo).
   - Si falla (exitCode != 0), la acción **finalize** debe abortar inmediatamente.
4. **Actualización de Evolution Logs:**
   - Añadir entrada en docs/EVOLUTION_LOG.md (raíz) o paths.evolutionPath + paths.evolutionLogFile.
   - Añadir sección en paths.evolutionPath + paths.evolutionLogFile con resumen y enlace a la carpeta de la feature.
5. **Sincronizar remoto + crear PR (Git S+):** Invocar **git-sync-remote** seguido de **git-create-pr**. Sin este paso ejecutado con éxito, el cierre no está completo.
6. **Persistencia opcional:** Escribir finalize.md en la carpeta de la feature (Cúmulo) con frontmatter YAML (pr_url, branch, timestamp) + cuerpo Markdown.
7. **Auditoría:** Registrar el evento de finalización en paths.auditsPath + paths.accessLogFile (Cúmulo).
8. **Post-PR:** Tras aceptación/merge del PR, cualquier limpieza o post-merge debe ejecutarse mediante skill/herramienta autorizada (sin git directo), respetando la Ley COMANDOS.

## Implementación técnica

La acción finalize debe implementarse/orquestarse invocando skills Git S+ (git-save-snapshot si aplica, git-sync-remote y git-create-pr). No debe depender de scripts que ejecuten git directo.

## Integración con agentes

- **Tekton Developer (ejecutor del cierre):** Responsable de ejecutar finalize: commits finales, actualización de logs, sincronización remota y apertura del PR, siempre mediante skills (Git S+) o herramientas autorizadas; sin git directo.
- **QA Judge:** Debe haber validado antes (validacion.json pass); si finalize se ejecuta sin validación previa, puede registrarse una advertencia.
- **Cúmulo:** Validan que la documentación de la feature esté en la ruta canónica y que los Evolution Logs referencien correctamente esa ruta (SSOT).

## Agente responsable (referencia para definición de agente)

| Concepto | Descripción |
| :--- | :--- |
| **Id sugerido** | `tekton-developer` (cierre y PR) o un agente dedicado `finalizer` / `release-agent` si se desea separar responsabilidades. |
| **Rol** | Cierre: commits atómicos, actualización de Evolution Logs, push, creación del PR. Respetar Ley GIT y SSOT. |
| **Skills necesarios** | `git-workspace-recon`, `git-branch-manager`, `git-save-snapshot`, `git-sync-remote`, `git-tactical-retreat`, `git-create-pr` (Git S+), `git-operations`, `documentation`, `invoke-command` cuando aplique. |
| **Restricciones** | Nunca commit en rama troncal; toda operación git/comando vía skill/herramienta; descripción del PR debe enlazar a paths.featurePath/<nombre_feature>/ (Cúmulo).**

Si se desea un agente nuevo para no mezclar “escribir código” con “cerrar y hacer PR”, se puede definir:

- **Finalizer / Release Agent:** Solo se encarga de la fase 8: leer validacion.md, actualizar logs, push y crear PR. Invocado por Tekton o por el orquestador de la acción feature.

## Estándares de calidad

- **Grado S+:** Trazabilidad completa: rama → paths.featurePath → spec/clarify/plan → implementation → execution → validacion → Evolution Logs → PR.
- **Ley GIT:** Ningún commit en master; todo el trabajo en rama feat/ o fix/ con documentación en la carpeta de la feature.
- **Single Source of Truth:** La referencia en PR y en Evolution Log es paths.featurePath/<nombre_feature>/ (Cúmulo).

## Dependencias con otras acciones

- **validate:** Debe haber ejecutado y producido `validacion.json` con pass antes de considerar el cierre seguro.
- **feature:** finalize es la última fase (8) del procedimiento feature; depende de que las fases 0–7 estén completadas.

---
*Documento de definición de la acción Finalize. Corresponde a la fase 8 del procedimiento feature (cierre, logs y PR).*
