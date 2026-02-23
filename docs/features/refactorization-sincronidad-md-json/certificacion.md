# Certificación de salida — Sincronidad MD/JSON

**Proceso:** refactorization  
**Tarea:** refactorization-sincronidad-md-json  
**Protocolo:** Racso-Tormentosa v1.2 — Certificación de salida SddIA.

## Checklist de coherencia del ecosistema

Antes de dar por finalizada la refactorización, se valida:

- [ ] **Process cargado:** SddIA/process/refactorization/ (spec.md, spec.json) utilizado como proceso rector.
- [ ] **Entidades de dominio definidas:** Norma SddIA/norms/entidades-dominio-ecosistema-sddia.md: entidades que integran el contrato/ítem Token; todas respetan estructura y sincronidad.
- [ ] **Skills referenciadas:** Definiciones en paths.skillsDefinitionPath consultadas; invoke-command, iniciar-rama, finalizar-git, documentation involucradas en la tarea o en la gobernanza.
- [ ] **Acciones vinculadas:** Todas las tareas técnicas tratadas como Acciones del proceso (spec, planning, implementation, validate, sddia-difusion).
- [ ] **Paridad MD/JSON en touchpoints:** validate y sddia-difusion actualizados con criterios de paridad spec.md ↔ spec.json para skills y process.
- [ ] **Trazabilidad atómica:** Objetivos, triaje, análisis, spec e implementación con artefactos .md y .json en la carpeta de la tarea (Cúmulo).
- [ ] **Marcado [REF-SddIA]:** Etiquetas insertadas en validate y sddia-difusion vinculando los cambios a la acción correspondiente del proceso.

## Resultado

La ejecución de este checklist y la generación de validacion.json en la carpeta de la tarea constituyen la **certificación de salida** para esta refactorización. El ecosistema de ficheros (MD ↔ JSON ↔ acciones) queda gobernado por criterios explícitos de sincronía.

**Ejecutado además:** AGENTS.md (sección 7), constitution.json (L7, domain_entities_ref), principles-contract.json, security-contract (md + json), execution.json, difusión en .cursor/rules (sddia-ssot.mdc). Ref: implementation.md y execution.json.
