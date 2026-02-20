# Action: Finalize

## Propósito

La acción **finalize** (finalizar) cierra el ciclo de la feature: asegura commits atómicos en la rama, actualiza los Evolution Logs, **sube la rama al remoto (push)** y crea el Pull Request hacia `master`. Solo debe ejecutarse cuando la validación ha pasado; en caso contrario, debe advertir o bloquear. **Comportamiento obligatorio:** al realizar finalize, el ejecutor debe comprender e incluir el paso de **subir (push)**: publicar la rama actual en `origin` antes de crear el PR; sin este paso el cierre no está completo. Proporciona trazabilidad y cierre formal alineado con las Leyes Universales (no commit en master, documentación en paths.featurePath según Cúmulo).

## Principio

- **No tocar master:** Todo el trabajo permanece en la rama feat/ o fix/; el merge se hace vía PR, no con commit directo en master.
- **Documentación como SSOT:** La descripción del PR y los logs hacen referencia a la carpeta de la feature (ej. paths.featurePath/<nombre_feature>/).
- **Auditoría:** Toda finalización queda registrada en Evolution Logs y, opcionalmente, en auditoría.

## Entradas

- **Carpeta de la feature:** Ruta obtenida de Cúmulo (ej. paths.featurePath/<nombre_feature>/).
  - Se espera que existan al menos: `objectives.md`, y preferiblemente `validacion.json` con resultado global pass.
- **Rama actual:** Rama feat/ o fix/ con todos los cambios ya commiteados (o la acción puede incluir un paso de “commit pendientes” según criterio del proyecto).

## Salidas

- **Rama publicada (subir / push):** La rama actual debe quedar subida en `origin`; es una salida obligatoria de finalize antes de considerar el PR creado.
- **Evolution Logs actualizados:**
  - paths.evolutionPath + paths.evolutionLogFile (raíz docs: docs/EVOLUTION_LOG.md según proyecto): una línea con formato `[YYYY-MM-DD] [feat/<nombre>] [Descripción breve del resultado.] [Estado].`
  - paths.evolutionPath + paths.evolutionLogFile: una sección con fecha, título de la feature, resumen de acción/alcance/resultado y referencia a la carpeta de la feature (Cúmulo)/objectives.md.
- **Pull Request:** Creado hacia `master`, con descripción que enlace a la documentación de la feature (ej. paths.featurePath/<nombre_feature>/).
- **Opcional:** Referencia al PR o estado en validacion.json o finalize.json de la carpeta de la feature (Cúmulo) (ej. URL del PR, timestamp de cierre).

## Skill de referencia: finalizar-git

La acción finalize **utiliza la skill** `finalizar-git` (definición en paths.skillsDefinitionPath/finalizar-git/ (spec.md, spec.json) y `spec.json`; implementación en paths.skillCapsules[\"finalizar-git\"]) para centralizar todas las interacciones con Git. El ejecutor debe aplicar los pasos y reglas definidos en esa skill para las fases **pre_pr** (push y creación del PR) y, cuando corresponda, **post_pr** (tras aceptar el PR: unificar, eliminar rama unificada, volver a master). La skill es la única fuente de verdad para los comandos y flujos git de cierre.

## Flujo de ejecución (propuesto)

1. **Comprobación de precondiciones:**
   - Rama actual no es `master`.
   - Existe objectives.md en la carpeta de la feature (Cúmulo).
   - Existe validacion.json en la carpeta de la feature (Cúmulo) y su resultado global es pass (o se permite finalize con advertencia si el proyecto lo define).
2. **Commits atómicos:** Si hay cambios sin commitear, el agente puede agruparlos en commits atómicos según convención (un commit por ítem lógico o por fase).
3. **Actualización de Evolution Logs:**
   - Añadir entrada en docs/EVOLUTION_LOG.md (raíz) o paths.evolutionPath + paths.evolutionLogFile.
   - Añadir sección en paths.evolutionPath + paths.evolutionLogFile con resumen y enlace a la carpeta de la feature.
4. **Subir la rama (push):** Publicar la rama actual en el remoto. **El agente debe ejecutar el comando**, no solo documentarlo: en PowerShell desde la raíz del repo, `git push -u origin <rama_actual>` (obtener rama con `git branch --show-current`). Comprobar la salida: debe aparecer confirmación de envío a `origin` (ej. `branch 'feat/...' set up to track 'origin/feat/...'` o `Everything up-to-date`). Si el usuario pide «subir» sin más contexto, ejecutar este paso: push de la rama actual a origin. Sin este paso ejecutado con éxito, el cierre no está completo.
5. **Crear Pull Request (skill finalizar-git, fase pre_pr):** Invocar **Push-And-CreatePR.ps1** de la cápsula finalizar-git (paths.skillCapsules[\"finalizar-git\"]): desde la raíz del repo, parámetro -Persist con valor paths.featurePath/<nombre_feature>/ (resolver vía Cúmulo). Si **GitHub CLI (gh)** está instalado y autenticado, ejecuta `gh pr create`; si no, muestra la URL para crear el PR manualmente. La descripción del PR enlaza a la carpeta de la feature (paths.featurePath/<nombre_feature>/). (Nota: el script puede incluir el push; si ya se ejecutó el paso 4, el push redundante es idempotente.)
6. **Persistencia opcional:** Escribir finalize.json en la carpeta de la feature (Cúmulo) con { "pr_url": "...", "branch": "...", "timestamp": "..." }.
7. **Auditoría:** Registrar el evento de finalización en paths.auditsPath + paths.accessLogFile (Cúmulo).
8. **Post-PR (skill finalizar-git, fase post_pr):** Una vez el PR esté aceptado/mergeado en el remoto, el ejecutor (o el usuario) aplica la fase **post_pr** de la skill `finalizar-git` invocando la skill finalizar-git (paths.skillCapsules[\"finalizar-git\"]). Tekton invoca la implementación (fase post_pr). Ver paths.skillsDefinitionPath/finalizar-git/spec.md.

## Implementación técnica (opcional)

Puede implementarse mediante scripts (PowerShell o equivalente) que ejecuten los pasos de cierre. Parámetros típicos:

- `--persist`: ruta de la carpeta de la feature.
- `--no-pr`: solo actualizar logs y push; no crear el PR (útil si el PR se abre manualmente).
- `--token`: (opcional) token de auditoría.

## Integración con agentes

- **Tekton Developer (ejecutor del cierre):** Puede ser el responsable de ejecutar finalize: commits finales, actualización de logs, push y apertura del PR, siempre mediante invoke-command para comandos de sistema y git.
- **QA Judge:** Debe haber validado antes (validacion.json pass); si finalize se ejecuta sin validación previa, puede registrarse una advertencia.
- **Cúmulo:** Validan que la documentación de la feature esté en la ruta canónica y que los Evolution Logs referencien correctamente esa ruta (SSOT).

## Agente responsable (referencia para definición de agente)

| Concepto | Descripción |
| :--- | :--- |
| **Id sugerido** | `tekton-developer` (cierre y PR) o un agente dedicado `finalizer` / `release-agent` si se desea separar responsabilidades. |
| **Rol** | Cierre: commits atómicos, actualización de Evolution Logs, push, creación del PR. Respetar Ley GIT y SSOT. |
| **Skills necesarios** | `finalizar-git` (obligatorio para pasos Git de cierre), `git-operations`, `documentation`, `invoke-command` (y posiblemente integración con API del repositorio para crear PR). |
| **Restricciones** | Nunca commit en master; toda operación git/comando vía invoke-command; descripción del PR debe enlazar a paths.featurePath/<nombre_feature>/ (Cúmulo).**

Si se desea un agente nuevo para no mezclar “escribir código” con “cerrar y hacer PR”, se puede definir:

- **Finalizer / Release Agent:** Solo se encarga de la fase 8: leer validacion.json, actualizar logs, push y crear PR. Invocado por Tekton o por el orquestador de la acción feature.

## Estándares de calidad

- **Grado S+:** Trazabilidad completa: rama → paths.featurePath → spec/clarify/plan → implementation → execution → validacion → Evolution Logs → PR.
- **Ley GIT:** Ningún commit en master; todo el trabajo en rama feat/ o fix/ con documentación en la carpeta de la feature.
- **Single Source of Truth:** La referencia en PR y en Evolution Log es paths.featurePath/<nombre_feature>/ (Cúmulo).

## Dependencias con otras acciones

- **validate:** Debe haber ejecutado y producido `validacion.json` con pass antes de considerar el cierre seguro.
- **feature:** finalize es la última fase (8) del procedimiento feature; depende de que las fases 0–7 estén completadas.

---
*Documento de definición de la acción Finalize. Corresponde a la fase 8 del procedimiento feature (cierre, logs y PR).*
