# SPEC: Limpieza de Referencias a Consola (Herencia Monolito)

**Proceso:** refactorization  
**Ruta:** docs/features/refactorization-limpieza-referencias-consola/  
**Fecha:** 2026-02-21  

---

## 1. Requerimientos

1. Eliminar todas las referencias a `GesFer.Console` y `src/Console/GesFer.Console.csproj` en documentación y scripts.
2. Sustituir invocaciones obsoletas por alternativas válidas (p. ej. `dotnet test` para la suite de tests).
3. **Las acciones SddIA han de hacer referencias exclusivamente a skills o tools** (paths.skillCapsules, paths.toolCapsules o paths.skillsDefinitionPath). Nunca a GesFer.Console ni herramientas inexistentes.
4. Actualizar manifesto.json para que Utils no mencione "Console" en Admin.Back.
5. Cerrar o actualizar DT-2025-001 (opción 2: documentación actualizada).

---

## 2. Regla de Referencia para Acciones

Toda acción en paths.actionsPath ha de especificar qué **skill** o **tool** la implementa o soporta. Fuentes canónicas (Cúmulo):
- paths.skillCapsules: iniciar-rama, finalizar-git, invoke-command
- paths.toolCapsules: invoke-mysql-seeds, prepare-full-env
- paths.skillsDefinitionPath: documentation, dotnet-development, frontend-test, etc.

---

## 3. Mapeo Acción → Skills/Tools (reemplazo de GesFer.Console)

| Acción | Skill/Tool de referencia | Implementación |
|--------|--------------------------|----------------|
| **spec** | invoke-command, documentation | Documentación manual en carpeta de tarea (paths.featurePath, paths.fixPath). Comandos de sistema vía invoke-command. Estándares según skill documentation. |
| **clarify** | invoke-command, documentation | Idem: documentación manual; comandos vía invoke-command. |
| **planning** | invoke-command, documentation | Idem: documentación manual; comandos vía invoke-command. |
| **validate** | invoke-command, dotnet-development | Build y tests vía invoke-command (dotnet build, dotnet test). |
| **finalize** | finalizar-git, invoke-command | Cierre de rama vía skill finalizar-git; comandos vía invoke-command. |

---

## 4. Touchpoints (plan de implementación)

### 4.1 SddIA/actions/

| Archivo | Cambio |
|---------|--------|
| `spec/spec.md` | Sustituir "GesFer.Console --spec" por referencia a **invoke-command** (paths.skillCapsules["invoke-command"]) y **documentation** (paths.skillsDefinitionPath/documentation/). Implementación: documentación manual; comandos vía invoke-command. Eliminar comando dotnet run src/Console/... |
| `clarify/spec.md` | Idem: referenciar invoke-command y documentation; implementación manual; eliminar GesFer.Console. |
| `planning/spec.md` | Idem: referenciar invoke-command y documentation; implementación manual; eliminar GesFer.Console. |

### 4.2 SddIA/agents/

| Archivo | Cambio |
|---------|--------|
| `architect.json` | Eliminar "GesFer.Console --spec". Sustituir por referencia a skill invoke-command y documentation. |
| `clarifier.json` | Eliminar referencias a GesFer.Console. Sustituir por invoke-command y documentation. |
| `auditor/process-interaction.json` | Eliminar "Enforce Spec Generation via GesFer.Console --spec". Sustituir por validación de spec según invoke-command + documentation. |
| `security-engineer.json` | Revisar: solo menciona logs (gesfer-console_*.log). Mantener o aclarar que es nombre de archivo legacy; no referir GesFer.Console como herramienta. |

### 4.3 SddIA/norms/

| Archivo | Cambio |
|---------|--------|
| `patterns-in-planning-implementation-execution.md` | Sustituir "GesFer.Console --plan" por referencia a invoke-command y documentation (planificación manual). |

### 4.4 SddIA/templates/

| Archivo | Cambio |
|---------|--------|
| `spec-template.md` | Eliminar línea "src/Console: Comandos y lógica de interfaz" si existe. Sustituir "GesFer.Console --spec" por referencia a invoke-command y documentation. |

### 4.5 scripts/skills/

| Archivo | Cambio |
|---------|--------|
| `pr-skill.md` | Sustituir "opción 11 de la Consola" y "Consola: src/Console/...". Referenciar skill **invoke-command** para ejecución de tests (dotnet test) y **dotnet-development** para estándares. Ruta: paths.skillCapsules["invoke-command"]. |
| `pr-skill.sh` | Sustituir `dotnet run --project src/Console/GesFer.Console.csproj --no-build -- 11` por `dotnet test src/GesFer.Admin.Back.IntegrationTests/GesFer.Admin.Back.IntegrationTests.csproj --no-build -v q`. Ajustar ruta según estructura real. |

### 4.6 SddIA/manifesto.json

| Elemento | Cambio |
|----------|--------|
| Utils rules | Eliminar "Contains Console" de las reglas de Utils (Admin.Back no incluye Console). |

### 4.7 docs/DeudaTecnica/

| Archivo | Cambio |
|---------|--------|
| `DT-2025-001-MissingConsoleTool.md` | Actualizar estado a "Cerrada" con decisión: "Documentación actualizada; proceso manual. No se implementa GesFer.Console en Admin.Back." |

### 4.8 docs/audits/ y docs/features/standardize-nomenclature/

| Archivo | Cambio |
|---------|--------|
| `validacion-main-20260217.json` | Referencia a PLAN-CONSOLA-ACCION-1-FRONT-OPTIMIZACION.json: mantener como historial o aclarar que es plan legacy; no requiere cambio crítico. |
| `clarify.md` (standardize-nomenclature) | Ya menciona que GesFer.Console no existe; consistencia con la limpieza. |

---

## 5. Criterios de aceptación

- [ ] No queda ninguna referencia a `GesFer.Console` como herramienta CLI en SddIA, scripts ni docs.
- [ ] `pr-skill.sh` ejecuta tests mediante `dotnet test` sobre el proyecto de IntegrationTests real.
- [ ] Las acciones spec/clarify/planning referencian skills o tools (invoke-command, documentation); sin GesFer.Console.
- [ ] DT-2025-001 cerrada con decisión documentada.
- [ ] Build y tests pasan tras los cambios.

---

## 6. Restricciones

- **NO** eliminar Serilog.Sinks.Console, WriteTo.Console(), ni Console.WriteLine en scripts de utilidad.
- **NO** tocar `--logger 'console;verbosity=detailed'` en dotnet test.
- Entorno: Windows 11 + PowerShell 7+; pr-skill.sh usa Bash (Git Bash en Windows).

---

*Especificación técnica de refactorización. Cumple interfaz de proceso (spec.md + spec.json).*
