# Action: Finalize

## Propósito

La acción **finalize** (finalizar) cierra el ciclo de la feature: asegura commits atómicos en la rama, actualiza los Evolution Logs, hace push de la rama y crea el Pull Request hacia `master`. Solo debe ejecutarse cuando la validación ha pasado; en caso contrario, debe advertir o bloquear. Proporciona trazabilidad y cierre formal alineado con las Leyes Universales (no commit en master, documentación en `docs/Feature/` o `docs/features/`).

## Principio

- **No tocar master:** Todo el trabajo permanece en la rama feat/ o fix/; el merge se hace vía PR, no con commit directo en master.
- **Documentación como SSOT:** La descripción del PR y los logs hacen referencia a la carpeta de la feature (ej. `docs/features/<nombre_feature>/`).
- **Auditoría:** Toda finalización queda registrada en Evolution Logs y, opcionalmente, en auditoría.

## Entradas

- **Carpeta de la feature (persist):** Ruta `{persist}` (ej. `docs/features/<nombre_feature>/`).
  - Se espera que existan al menos: `objectives.md`, y preferiblemente `validacion.json` con resultado global pass.
- **Rama actual:** Rama feat/ o fix/ con todos los cambios ya commiteados (o la acción puede incluir un paso de “commit pendientes” según criterio del proyecto).

## Salidas

- **Evolution Logs actualizados:**
  - `docs/EVOLUTION_LOG.md`: una línea con formato `[YYYY-MM-DD] [feat/<nombre>] [Descripción breve del resultado.] [Estado].`
  - `docs/evolution/EVOLUTION_LOG.md` (o equivalente): una sección con fecha, título de la feature, resumen de acción/alcance/resultado y referencia a `docs/features/<nombre_feature>/OBJETIVO.md` o `objectives.md`.
- **Pull Request:** Creado hacia `master`, con descripción que enlace a la documentación de la feature (ej. `docs/features/<nombre_feature>/`).
- **Opcional:** Referencia al PR o estado en `{persist}/validacion.json` o en un artefacto `{persist}/finalize.json` (ej. URL del PR, timestamp de cierre).

## Flujo de ejecución (propuesto)

1. **Comprobación de precondiciones:**
   - Rama actual no es `master`.
   - Existe `{persist}/objectives.md`.
   - Existe `{persist}/validacion.json` y su resultado global es pass (o se permite finalize con advertencia si el proyecto lo define).
2. **Commits atómicos:** Si hay cambios sin commitear, el agente puede agruparlos en commits atómicos según convención (un commit por ítem lógico o por fase).
3. **Actualización de Evolution Logs:**
   - Añadir entrada en `docs/EVOLUTION_LOG.md`.
   - Añadir sección en `docs/evolution/EVOLUTION_LOG.md` con resumen y enlace a la carpeta de la feature.
4. **Push de la rama:** `git push origin <rama>` (mediante invoke-command o script autorizado).
5. **Creación del PR:** Usando la API del proveedor (GitHub, Azure DevOps, etc.) o instrucciones claras para crearlo manualmente; la descripción del PR debe incluir la ruta a `docs/features/<nombre_feature>/`.
6. **Persistencia opcional:** Escribir `{persist}/finalize.json` con { "pr_url": "...", "branch": "...", "timestamp": "..." }.
7. **Auditoría:** Registrar el evento de finalización en `docs/audits/ACCESS_LOG.md` o equivalente.

## Implementación técnica (opcional)

Si se implementa como comando de consola, por ejemplo:

```powershell
dotnet run --project src/Console/GesFer.Console.csproj -- --finalize --persist <FEATURE_PATH> [--no-pr] [--token <AUDITOR_TOKEN>]
```

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
| **Skills necesarios** | `git-operations`, `documentation`, `invoke-command` (y posiblemente integración con API del repositorio para crear PR). |
| **Restricciones** | Nunca commit en master; toda operación git/comando vía invoke-command; descripción del PR debe enlazar a docs/features/<nombre_feature>/.**

Si se desea un agente nuevo para no mezclar “escribir código” con “cerrar y hacer PR”, se puede definir:

- **Finalizer / Release Agent:** Solo se encarga de la fase 8: leer validacion.json, actualizar logs, push y crear PR. Invocado por Tekton o por el orquestador de la acción feature.

## Estándares de calidad

- **Grado S+:** Trazabilidad completa: rama → docs/features → spec/clarify/plan → implementation → execution → validacion → Evolution Logs → PR.
- **Ley GIT:** Ningún commit en master; todo el trabajo en rama feat/ o fix/ con documentación en la carpeta de la feature.
- **Single Source of Truth:** La referencia en PR y en Evolution Log es la ruta en `docs/features/<nombre_feature>/`.

## Dependencias con otras acciones

- **validate:** Debe haber ejecutado y producido `validacion.json` con pass antes de considerar el cierre seguro.
- **feature:** finalize es la última fase (8) del procedimiento feature; depende de que las fases 0–7 estén completadas.

---
*Documento de definición de la acción Finalize. Corresponde a la fase 8 del procedimiento feature (cierre, logs y PR).*
