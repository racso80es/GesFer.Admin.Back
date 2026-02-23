# Norma: Entidades de dominio (ecosistema SddIA)

**Fuente:** SddIA/norms. Aplicable a todo agente o IA que opere en el repositorio y a la gobernanza de artefactos SddIA.

## Definición

Se denominan **entidades de dominio** o **entidades del ecosistema SddIA** a aquellas que **integran el ítem o contrato de Token** (paths.tokensPath; Cúmulo). Es decir, son las entidades cuyos contratos o definiciones exigen operar bajo un contexto de Token (p. ej. Karma2Token) para trazabilidad y seguridad.

**Incluyen, según sus contratos:**

- **Skills** (paths.skillsDefinitionPath, paths.skillCapsules) — contrato: paths.skillsDefinitionPath/skills-contract.json (required_token: Karma2Token).
- **Tools** (paths.toolsDefinitionPath, paths.toolCapsules) — contrato: paths.toolsDefinitionPath/tools-contract.json (required_token: Karma2Token).
- **Actions** (paths.actionsPath) — contrato: actions-contract.json (required_token: Karma2Token).
- **Process** (paths.processPath) — contrato: paths.processPath/process-contract.json (required_token: Karma2Token).
- **Patterns** (paths.patternsPath) — contrato: patterns-contract.json (required_token: Karma2Token).
- **Principles** (paths.principlesPath) — contrato: principles-contract.json (required_token: Karma2Token).
- **Templates** (paths.templatesPath) — contrato: templates-contract (required_token: Karma2Token).
- **Tokens** (paths.tokensPath) — contrato: paths.tokensPath/tokens-contract.json (definición de los propios tokens).

## Obligaciones de estructura y sincronidad

Todas las **entidades de dominio** han de:

1. **Respetar la estructura** definida en su contrato: típicamente `spec.md` y `spec.json` en la carpeta de la entidad (paths según Cúmulo), con los campos y artefactos que exija el contrato.
2. **Mantener sincronidad** entre la documentación legible (MD) y la definición estructural (JSON): cualquier cambio en la lógica o descripción en un `.md` debe propagarse al `.json` correspondiente en la misma transacción, y viceversa cuando el contrato sea JSON-first.

La validación de esta sincronía puede realizarse mediante el check opcional `sddia_md_json_parity` (acción validate) cuando el diff toque paths.skillsDefinitionPath, paths.processPath u otras rutas de entidades de dominio. La acción sddia-difusion incluye entre sus criterios la paridad MD/JSON en las definiciones que se difunden.

## Referencias

- **Token (Karma2Token):** paths.tokensPath; SddIA/tokens/karma2-token/spec.json.
- **Contrato de tokens:** paths.tokensPath/tokens-contract.json (Cúmulo).
- **Sincronía MD/JSON:** paths.featurePath/refactorization-sincronidad-md-json/; SddIA/actions/validate (optional_checks.sddia_md_json_parity); SddIA/actions/sddia-difusion (criterios de aceptación).

---
*Definición canónica de entidades de dominio para gobernanza SddIA. Ref: refactorization-sincronidad-md-json.*
