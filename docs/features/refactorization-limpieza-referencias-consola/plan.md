# PLAN: Limpieza de Referencias a Consola (Herencia Monolito)

**Proceso:** refactorization  
**Ruta:** docs/features/refactorization-limpieza-referencias-consola/  
**Especificación:** spec.md | **Clarificación:** clarify.md  
**Fecha:** 2026-02-21  

---

## 1. Objetivos del plan

- Eliminar referencias a GesFer.Console en SddIA, scripts y docs.
- Sustituir por referencias a skills/tools (invoke-command, documentation, dotnet-development).
- Asegurar que pr-skill.sh ejecute tests vía dotnet test (proyecto IntegrationTests Admin.Back).
- Cerrar DT-2025-001 con decisión documentada.

---

## 2. Fases y tareas técnicas

### Fase 1 – Acciones SddIA (spec, clarify, planning)

| Id | Tarea | Criterio |
|----|-------|----------|
| 1.1 | Actualizar SddIA/actions/spec/spec.md | Sustituir GesFer.Console por invoke-command + documentation; eliminar comando dotnet run src/Console; mantener Flujo, Integración, Estándares. |
| 1.2 | Actualizar SddIA/actions/clarify/spec.md | Idem: invoke-command + documentation; eliminar GesFer.Console. |
| 1.3 | Actualizar SddIA/actions/planning/spec.md | Idem: invoke-command + documentation; eliminar GesFer.Console. |

### Fase 2 – Agentes SddIA

| Id | Tarea | Criterio |
|----|-------|----------|
| 2.1 | Actualizar SddIA/agents/architect.json | Eliminar "GesFer.Console --spec" en tools e instructions; referenciar invoke-command y documentation. |
| 2.2 | Actualizar SddIA/agents/clarifier.json | Eliminar referencias a GesFer.Console en triggers y tools; sustituir por invoke-command y documentation. |
| 2.3 | Actualizar SddIA/agents/auditor/process-interaction.json | Eliminar "Enforce Spec Generation via GesFer.Console --spec"; sustituir por validación de spec según invoke-command + documentation. |
| 2.4 | Revisar SddIA/agents/security-engineer.json | Mantener gesfer-console_*.log como nombre de archivo de logs; no referir GesFer.Console como herramienta (opcional aclaración). |

### Fase 3 – Normas y plantillas

| Id | Tarea | Criterio |
|----|-------|----------|
| 3.1 | Actualizar SddIA/norms/patterns-in-planning-implementation-execution.md | Sustituir "GesFer.Console --plan" por referencia a invoke-command y documentation (planificación manual). |
| 3.2 | Actualizar SddIA/templates/spec-template.md | Eliminar "src/Console: Comandos y lógica de interfaz"; sustituir "GesFer.Console --spec" por invoke-command y documentation. |

### Fase 4 – PR Skill (scripts)

| Id | Tarea | Criterio |
|----|-------|----------|
| 4.1 | Actualizar scripts/skills/pr-skill.md | Sustituir "opción 11 de la Consola" y "Consola: src/Console/..."; referenciar invoke-command y dotnet-development; describir ejecución de tests vía dotnet test. |
| 4.2 | Actualizar scripts/skills/pr-skill.sh | Sustituir invocación a src/Console/GesFer.Console.csproj -- 11 por dotnet test al proyecto GesFer.Admin.Back.IntegrationTests. |

### Fase 5 – Manifesto y deuda técnica

| Id | Tarea | Criterio |
|----|-------|----------|
| 5.1 | Actualizar SddIA/manifesto.json | En Utils rules, eliminar "Contains Console". |
| 5.2 | Actualizar docs/DeudaTecnica/DT-2025-001-MissingConsoleTool.md | Estado "Cerrada"; decisión: documentación actualizada; no se implementa GesFer.Console en Admin.Back. |

---

## 3. Verificación

- Build: `dotnet build` sin errores.
- Tests: `dotnet test` (proyecto IntegrationTests) pasan.
- Búsqueda: no debe quedar "GesFer.Console" como herramienta CLI (sí permitido gesfer-console_*.log como nombre de archivo).

---

## 4. Seguridad y trazabilidad

- No se eliminan Serilog.Sinks.Console, WriteTo.Console(), ni Console.WriteLine en scripts de utilidad.
- No se modifica `--logger 'console;verbosity=detailed'` en dotnet test.
- Rutas: Cúmulo paths; acciones referencian paths.skillCapsules y paths.skillsDefinitionPath.

---

*Plan generado a partir de spec.md y clarify.md. Acción planning soportada por invoke-command y documentation (paths.skillCapsules, paths.skillsDefinitionPath).*
