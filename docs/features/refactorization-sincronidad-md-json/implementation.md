# Implementación: Sincronidad MD/JSON

**Proceso:** refactorization  
**Acción:** implementation (touchpoints) + execution (marcado y cambios aplicados).

## Touchpoints aplicados

| # | Área | Archivo | Cambio | Etiqueta |
|---|------|---------|--------|----------|
| 1 | SddIA/norms | entidades-dominio-ecosistema-sddia.md, README.md | Definición entidades de dominio (Token); índice normas | — |
| 2 | SddIA/actions/validate | spec.md, spec.json | Criterio opcional sddia_md_json_parity y estándar de sincronía SddIA | TODO: [REF-SddIA] validate |
| 3 | SddIA/actions/sddia-difusion | spec.md | Criterio paridad MD/JSON en skills/process difundidos | TODO: [REF-SddIA] sddia-difusion |
| 4 | AGENTS.md | — | Entidades de dominio en sección 7 (Karma2Token) | — |
| 5 | SddIA/constitution.json | — | Ley L7_DOMAIN_ENTITIES; configuration.domain_entities_ref | — |
| 6 | SddIA/principles/principles-contract.json | — | Constraint: principios como entidades de dominio | — |
| 7 | SddIA/security/security-contract.md, .json | — | Entidades de dominio en modelo Karma2Token; domain_entities | — |
| 8 | docs/features/refactorization-sincronidad-md-json | objectives, triaje, analisis, spec, implementation, certificacion, execution | Artefactos del proceso | — |

## Marcado de cambios (Racso-Tormentosa)

- **validate/spec.md:** Inserción del check opcional y del estándar de sincronía; comentario // TODO: [REF-SddIA] vinculado a la acción validate del proceso refactorization.
- **sddia-difusion/spec.md:** Inserción del criterio de paridad MD/JSON; comentario // TODO: [REF-SddIA] vinculado a la acción sddia-difusion.

## Trazabilidad MD-JSON-Código

- Objetivos (objectives.md ↔ objectives.json).
- Triaje (triaje.md ↔ triaje.json).
- Spec (spec.md ↔ spec.json).
- Certificación (certificacion.md + validacion.json).
