# Norma: Comandos solo vía skill, herramienta, acción o proceso

**Fuente:** SddIA/norms. Aplicable a todo agente o IA que opere en el repositorio.

## Principio

**La IA nunca debe ejecutar comandos de sistema directamente** en la shell (ni `git`, `dotnet`, `npm`, `pwsh`, `cargo`, ni cualquier otro comando). Toda ejecución de comandos ha de realizarse **siempre** a través de al menos uno de los siguientes canales:

- **Skill:** paths.skillCapsules (Cúmulo), definición en paths.skillsDefinitionPath. Implementación en Rust (paths.skillsRustPath). **Invocación por agente:** ejecutable `.exe` en la raíz de la cápsula con **JSON por stdin** y lectura de **JSON por stdout** según [capsule-json-io.md](./capsule-json-io.md). Los `.bat` son opcionales y están pensados para **uso humano**, no como interfaz del agente.
- **Herramienta (tool):** paths.toolCapsules, definición en paths.toolsDefinitionPath. Implementación en Rust (paths.toolsRustPath). **Mismo contrato JSON** que las skills (mismo documento normativo).
- **Acción:** paths.actionsPath. Orquestan skills o herramientas.
- **Proceso:** paths.processPath. Invocan acciones y skills.

## Contexto de implementación (Rust + JSON)

La implementación estándar son binarios **`.exe`** en la **raíz** de cada cápsula (sin carpeta `bin/`). **No** debe usarse `.ps1` como implementación ni fallback. El agente **no** debe invocar los `.bat` salvo ausencia documentada del binario en un entorno concreto (caso excepcional a corregir).

## Justificación

- **Trazabilidad:** skills/tools registran operaciones (p. ej. Karma2Token en `meta.token` cuando aplique).
- **Contrato único:** mismo envelope para skills y tools facilita parsers y orquestación.
- **Cumplimiento:** flujo único vía artefactos SddIA.

## Aplicación

- Si la IA necesita ejecutar un comando, debe **invocar el `.exe`** correspondiente (ruta vía Cúmulo) pasando el JSON de petición por stdin y parseando la respuesta JSON.
- Referencias: AGENTS.md (ley COMANDOS), [capsule-json-io.md](./capsule-json-io.md), git-via-skills-or-process.md para git concreto.
