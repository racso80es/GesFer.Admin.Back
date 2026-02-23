# Objetivos: Refactorización Sincronidad MD/JSON

**Proceso activado:** Refactorization (paths.processPath/refactorization/)  
**Plantilla:** Estándar refactorización de dominio — sin proceso "Gestión de Entidades", se usa refactorization según SddIA/process/README.md.  
**Ruta de tarea (Cúmulo):** paths.featurePath/refactorization-sincronidad-md-json/

## Regla de Oro aplicada

Antes de cualquier modificación se han cargado:
- **Process:** SddIA/process/refactorization/ (spec.md, spec.json).
- **Skills afectadas:** Definiciones en paths.skillsDefinitionPath (SddIA/skills/) — invoke-command, iniciar-rama, finalizar-git, documentation, etc.

## Objetivos concretados

1. **Definición de entidades de dominio (ecosistema SddIA):** Definir como **entidades de dominio** —o **entidades del ecosistema SddIA**— aquellas que integran el ítem o contrato de **Token** (paths.tokensPath; Karma2Token y contratos en paths.tokensPath). Son las que están sujetas a la gobernanza de estructura y sincronía. Todas han de respetar **estructura** (spec.md + spec.json según su contrato) y **sincronidad** (paridad MD ↔ JSON).
2. **Sincronización MD/JSON:** Garantizar que todo cambio en la lógica de una Skill o Process (descrita en Markdown) se propague a su definición estructural (JSON) y viceversa.
3. **Gobernanza de Acciones:** Asegurar que ninguna acción técnica de una entidad de dominio quede huérfana de un proceso que la supervise y valide; las tareas técnicas se tratan como Acciones del proceso seleccionado.
4. **Certificación de salida:** Validar que el ecosistema de ficheros (MD ↔ JSON ↔ código) es coherente antes de dar por cerrada la refactorización.

## Vinculación de Acciones

Todas las tareas técnicas de esta refactorización se tratan como **Acciones** del proceso refactorization: spec, clarify, planning, implementation, execution, validate, finalize. Trazabilidad MD-JSON-Código atómica.

## Ley aplicada

- **SSOT (SddIA):** docs/ y SddIA/ son la verdad; Cúmulo es la fuente de rutas.
- **Comandos:** Solo vía skill/herramienta/acción/proceso (SddIA/norms/commands-via-skills-or-tools.md).
