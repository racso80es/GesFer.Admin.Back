# SPEC: Refactorización Sincronidad MD/JSON

**Proceso:** refactorization  
**Ruta (Cúmulo):** paths.featurePath/refactorization-sincronidad-md-json/

## Propósito

Refactorizar la gobernanza para que cualquier cambio en la lógica de una Skill o Process (MD) se propague a su definición estructural (JSON) y que ninguna acción técnica quede huérfana del proceso que la supervisa. Incluye criterios de validación y difusión que exijan paridad MD/JSON.

## Alcance

- **Definir entidades de dominio (ecosistema SddIA):** Norma en SddIA/norms que establezca que son entidades de dominio las que integran el ítem o contrato de Token; todas han de respetar estructura (spec.md + spec.json) y sincronidad MD/JSON.
- Documentar la regla de sincronía MD/JSON en la acción **validate** y en **sddia-difusion**.
- Añadir criterios de aceptación que obliguen a comprobar coherencia spec.md ↔ spec.json para skills y process (y resto de entidades de dominio) tocados en la tarea.
- Insertar etiquetas // TODO: [REF-SddIA] en los touchpoints que implementen esta gobernanza (vinculadas a la acción correspondiente del proceso).

## Touchpoints

| Área | Archivo | Cambio |
|------|---------|--------|
| SddIA/norms | entidades-dominio-ecosistema-sddia.md, README.md | Definición entidades de dominio (integran Token); estructura y sincronidad; índice normas. |
| SddIA/actions/validate | spec.md, spec.json | Criterio: paridad MD/JSON en skills/process (entidades de dominio) tocados. |
| SddIA/actions/sddia-difusion | spec.md | Criterio: definiciones en skillsDefinitionPath/processPath con paridad MD/JSON en lo difundido. |
| docs/features/refactorization-sincronidad-md-json | spec, implementation, analisis, triaje, certificacion | Artefactos del proceso. |

## Trazabilidad

- Acciones del proceso usadas: spec, clarify (implícito en analisis), planning (este spec), implementation, validate, finalize.
- Skills involucradas: invoke-command (caso de sincronía ya aplicada), documentation (estándares).
