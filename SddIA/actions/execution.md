# Action: Execution

## Propósito

La acción **execution** (ejecución) aplica al código los cambios definidos en el documento de implementación. Transforma los ítems del `implementation.json` (o del documento IMPL asociado) en modificaciones reales en el repositorio: crear, modificar o eliminar archivos según la propuesta de cada ítem. No define qué cambiar —eso ya está en la fase de implementation— sino que **ejecuta** esos cambios de forma ordenada y registra el resultado para trazabilidad.

## Principio

- **Entrada única:** El documento de implementación (o su representación JSON) es la única fuente de lo que se debe hacer.
- **Orden y dependencias:** Se respetan el orden sugerido y las dependencias entre ítems (p. ej. crear entidad antes de usarla en un handler).
- **Trazabilidad:** Toda aplicación de cambio se registra en un artefacto de salida para validación y auditoría.

## Entradas

- **Documento de implementación** (obligatorio): ruta a `{persist}/implementation.json` o al documento IMPL (p. ej. `{persist}/implementation.md`). Contiene los ítems con Id, Acción, Ruta, Ubicación, Propuesta y Dependencias.

## Salidas

- **Registro de ejecución:** `{persist}/execution.json` (o `ejecution.json` si se mantiene nomenclatura de feature).
  - Debe incluir: por cada ítem aplicado, id del ítem, ruta del archivo, acción realizada (Crear | Modificar | Eliminar), estado (OK | Error), mensaje opcional y timestamp.
  - En caso de error en un ítem: registro del fallo y decisión (detener o continuar según criterio del agente).

## Flujo de ejecución (propuesto)

1. **Validación de entrada:** Comprobar que `implementation.json` (o IMPL) exista y sea válido.
2. **Carga del plan de ítems:** Ordenar ítems según dependencias y orden sugerido.
3. **Por cada ítem:**
   - Resolver la ruta del archivo (existente para Modificar/Eliminar; no existente para Crear).
   - Aplicar la propuesta (crear archivo, editar bloque, eliminar bloque/archivo).
   - Registrar el resultado en la estructura de salida.
4. **Persistencia:** Escribir `{persist}/execution.json` con el registro completo.
5. **Auditoría:** Opcionalmente registrar en `docs/audits/ACCESS_LOG.md` que se ejecutó la acción execution para la feature.

## Implementación técnica (opcional)

Si se implementa como comando de consola, por ejemplo:

```powershell
dotnet run --project src/Console/GesFer.Console.csproj -- --execution --implementation <IMPL_OR_JSON_PATH> [--persist <FEATURE_PATH>] [--token <AUDITOR_TOKEN>]
```

- `--implementation`: ruta a `implementation.json` o al documento IMPL.
- `--persist`: ruta base de la feature (ej. `docs/features/<nombre_feature>/`); donde se escribirá `execution.json`.
- `--token`: (opcional) token de auditoría.

## Integración con agentes

- **Tekton Developer (agente ejecutor):** Es el responsable de ejecutar esta acción. Aplica cada ítem del documento de implementación al código, respeta el contrato de comandos (invoke-command) y las restricciones (no master, commits atómicos, Kaizen). Consume `implementation.json` y produce `execution.json`.
- **Arquitecto / QA Judge:** Pueden revisar `execution.json` para comprobar que todos los ítems se aplicaron y que no hubo desviaciones respecto al IMPL.

## Agente responsable (referencia para definición de agente)

| Concepto | Descripción |
| :--- | :--- |
| **Id sugerido** | `tekton-developer` (ya existente). La acción execution es responsabilidad del mismo agente que implementa el plan. |
| **Rol** | Ejecutor: traducir documento de implementación en cambios de código y registro en `execution.json`. |
| **Skills necesarios** | `dotnet-development`, `filesystem-ops`, `git-operations`, `invoke-command`. |
| **Restricciones** | No trabajar en `master`; commits atómicos; todo comando de sistema vía invoke-command. |

No se requiere un agente nuevo: **Tekton Developer** asume la fase de ejecución. Si en el futuro se desea separar “planificador de cambios” de “aplicador”, podría definirse un agente **Execution Runner** que solo aplique IMPL y genere `execution.json`, invocado por Tekton.

## Estándares de calidad

- **Exhaustividad:** Todo ítem del documento de implementación debe aparecer en `execution.json` con estado OK o Error.
- **Determinismo:** Mismo IMPL en mismo contexto debe producir el mismo conjunto de cambios (salvo fallos de entorno).
- **Trazabilidad:** El PR y la validación posterior pueden apoyarse en `execution.json` para saber qué se tocó.

## Dependencias con otras acciones

- **implementation** (acción previa): Produce la entrada (`implementation.json` / IMPL).
- **validate** (acción siguiente): Puede usar `execution.json` para saber qué archivos/cambios comprobar.

---
*Documento de definición de la acción Execution. Corresponde a la fase 6 del procedimiento feature (aplicar el plan al código).*
