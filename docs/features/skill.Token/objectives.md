# Feature: Encapsulamiento de skills (patrón tools)

**Rama:** feat/skill.Token  
**Ruta (Cúmulo):** paths.featurePath/skill.Token

## Objetivo

Reestructurar **SddIA/skills** y **scripts/skills** para que cada skill esté encapsulada en su propia carpeta (definición en SddIA, implementación en scripts), aplicando el mismo patrón de encapsulamiento y contrato existente en **tools**. Establecer como norma que el ejecutable script sugerido ha de ser en **Rust** y generar binarios .exe con Rust para las skills que tengan implementación ejecutable.

## Alcance

- **Proceso:** SddIA/process/feature (fase 0: rama feat/skill.Token; fases 1–8 según feature.md).
- **Cúmulo:** Añadir en `SddIA/agents/cumulo.json` los paths de skills: `skillsPath`, `skillsDefinitionPath`, `skillsIndexPath`, `skillCapsules`.
- **SddIA/skills:** Contrato actualizado (skills-contract.md / skills-contract.json) con norma Rust y encapsulamiento; cada skill en subcarpeta `<skill-id>/` con spec.md y spec.json (definición).
- **scripts/skills:** Índice `index.json`; cada skill con script ejecutable en cápsula `<skill-id>/` (manifest.json, launcher .bat, .ps1 fallback, doc, bin/ para .exe Rust).
- **scripts/skills-rs:** Proyecto Rust con binarios para skills ejecutables (ej. iniciar_rama, merge_to_master_cleanup); install.ps1 que copie .exe a cada cápsula bin/.
- **Agentes y constitution:** Adecuar referencias a skills para usar rutas Cúmulo o rutas de cápsula; actualizar constitution si corresponde.

## Ley aplicada

- **L6_CONSULTATION:** Rutas canónicas desde Cúmulo (paths.skillsPath, paths.skillCapsules, paths.skillsDefinitionPath).
- **L2_ENV:** Windows 11 + PowerShell 7+; ejecutables Rust como .exe en cápsula.

## Resumen del proceso

| Fase | Acción |
|------|--------|
| 0 | Rama feat/skill.Token (skill iniciar-rama) |
| 1 | Documentación objetivos (este documento) |
| 2 | Spec (spec.md, spec.json en carpeta de la tarea / Cúmulo) |
| 3–6 | Clarificación, plan, implementación, ejecución |
| 7 | Validación (validacion.json) |
| 8 | Cierre y PR |

## Referencias

- Contrato tools: SddIA/tools/tools-contract.md, tools-contract.json.
- Cúmulo: SddIA/agents/cumulo.json.
- Proceso feature: SddIA/process/feature.md.
