# Action: Planning

## Propósito
La acción **planning** (planificación) transforma los requerimientos clarificados (`SPEC` + `CLARIFY`) en un plan de acción ejecutable y secuencial.

## Implementación
Esta acción se realiza de forma **manual** por el agente.

### Procedimiento
1.  **Entrada**: `SPEC.md` y `CLARIFY.md` aprobados.
2.  **Generación de Pasos**: Dividir el trabajo en tareas atómicas y medibles.
3.  **Ordenamiento**: Establecer dependencias y prioridades.
4.  **Verificación**: Asegurar que cada paso tiene un criterio de "hecho" claro.
5.  **Documentación**:
    -   Crear `PLAN-{Titulo}.md` (o `plan.md` en la carpeta correspondiente).
    -   Incluir la lista de tareas.
    -   Incluir verificaciones pre-commit y post-deploy.

### Flujo de Ejecución (Manual)
1.  **Análisis de Requerimientos**: El agente planificador revisa `SPEC` y `CLARIFY`.
2.  **Descomposición**: Se rompe el problema en pasos lógicos.
3.  **Persistencia**: Se guarda el plan en `docs/features/...` o `docs/bugs/...`.
4.  **Aprobación**: El usuario revisa y aprueba el plan.
5.  **Auditoría**: Registrar la acción en `docs/audits/ACCESS_LOG.md`.

## Integración con Agentes
*   **Architect / Lead Developer:** Define el plan.
*   **Tekton Developer:** Ejecuta el plan paso a paso.
*   **QA Judge:** Revisa el plan para asegurar cobertura de pruebas.

## Estándares de Calidad
*   **Atomicidad**: Pasos pequeños y manejables.
*   **Verificabilidad**: Cada paso debe poder ser validado.
