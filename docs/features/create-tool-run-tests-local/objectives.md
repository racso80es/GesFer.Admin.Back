# Objetivos: Herramienta run-tests-local (create-tool)

**Tarea:** create-tool-run-tests-local  
**Proceso:** paths.processPath/create-tool/  
**Rutas Cúmulo:** paths.featurePath, paths.toolsDefinitionPath, paths.toolCapsules, paths.toolsIndexPath.

---

## Objetivo principal

Incorporar una **herramienta** `run-tests-local` al ecosistema SddIA que permita ejecutar tests (unitarios, integración, E2E) en condiciones de validación local, **sin que el agente invoque comandos de sistema directamente**: la ejecución se realiza siempre a través de esta herramienta, cumpliendo el contrato de herramientas y las normas commands-via-skills-or-tools.

## Objetivos específicos

1. **Definición en SddIA**  
   - Crear paths.toolsDefinitionPath/run-tests-local/ con spec.md y spec.json.  
   - Incluir implementation_path_ref a la cápsula, inputs (SkipPrepare, SkipSeeds, TestScope: unit | integration | e2e | all), OutputPath, OutputJson, y fases de feedback alineadas al contrato.

2. **Cápsula (implementación)**  
   - Crear paths.toolCapsules.run-tests-local (según Cúmulo) con: manifest.json, run-tests-local-config.json, documentación .md, launcher .bat y script .ps1 de fallback que orqueste:
     - Opcional: invocación a prepare-full-env (paths.toolCapsules.prepare-full-env).
     - Opcional: invocación a invoke-mysql-seeds (paths.toolCapsules.invoke-mysql-seeds).
     - Compilación de la solución (ruta desde config o Cúmulo).
     - Ejecución de tests según TestScope (unit, integration, e2e, all) vía dotnet test con filtros adecuados.
     - Variables de entorno para E2E (E2E_BASE_URL, E2E_INTERNAL_SECRET) cuando TestScope incluya e2e.
   - Salida JSON según tools-contract (toolId, exitCode, success, timestamp, message, feedback, data con resumen de tests, duration_ms).

3. **Índice y Cúmulo**  
   - Registrar run-tests-local en paths.toolsIndexPath (scripts/tools/index.json).  
   - Actualizar paths.toolCapsules en SddIA/agents/cumulo.paths.json con la entrada run-tests-local.

4. **Validación con el caso de uso**  
   - Tras crear la herramienta, **ponerla a prueba** con el caso de uso "validación en local": ejecutar la tool con parámetros que incluyan E2E (o OnlyTests equivalente) y verificar que el resultado JSON y el exitCode reflejen el resultado real de los tests, sin que el agente ejecute dotnet test directamente.

5. **Documentación de la tarea**  
   - Mantener en paths.featurePath/create-tool-run-tests-local/ los artefactos del proceso: analysis.md, objectives.md, objectives.json y, en fases siguientes, spec.md, spec.json, implementation.md, implementation.json, validacion.json según create-tool.

## Criterios de éxito

- La herramienta run-tests-local existe en paths.toolsDefinitionPath y en paths.toolCapsules.  
- Invocar la herramienta (por ejemplo desde scripts o por documentación de uso) ejecuta la secuencia definida y produce un JSON de resultado conforme al contrato.  
- El agente (o cualquier consumidor) puede usar run-tests-local como único punto de ejecución de tests en condiciones locales, sin invocar dotnet test directamente.  
- El caso de uso "validación en local" (E2E con infra y datos preparados) se valida ejecutando la tool y comprobando salida y códigos de salida.

## Alcance fuera de esta tarea

- Implementación en Rust (paths.toolsRustPath) puede quedar para una fase posterior; el fallback .ps1 en la cápsula es suficiente para cumplir el contrato y el caso de uso.  
- La plantilla "test-local-validacion" (template) que solo defina la situación puede crearse en una tarea create-template posterior si se desea.

---

*Objetivos alineados con SddIA/process/create-tool y SddIA/tools/tools-contract.json.*
