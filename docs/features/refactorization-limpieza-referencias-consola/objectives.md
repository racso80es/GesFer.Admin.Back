# Proceso: Limpieza de Referencias a Consola (Herencia Monolito)

**Proceso:** refactorization  
**Rama:** feat/refactorization-limpieza-referencias-consola  
**Fecha inicio:** 2026-02-21  
**Estado:** En curso  

---

## 1. Objetivo

Eliminar referencias y código innecesario relacionados con **GesFer.Console** (herramienta CLI del monolito original) que ya no existe en el código fuente de GesFer.Admin.Back.

## 2. Regla de Referencia (Acciones → Skills/Tools)

**Las acciones SddIA han de hacer referencias exclusivamente a skills o tools.** Fuentes canónicas (Cúmulo):
- paths.skillCapsules: iniciar-rama, finalizar-git, invoke-command
- paths.toolCapsules: invoke-mysql-seeds, prepare-full-env
- paths.skillsDefinitionPath: documentation, dotnet-development, frontend-test, etc.

Nunca referenciar GesFer.Console ni herramientas inexistentes.

## 3. Alcance

### 3.1 Contexto

- **GesFer.Console** (`src/Console/GesFer.Console.csproj`) era parte del monolito GesFer.
- En GesFer.Admin.Back **no existe** el proyecto Console (DT-2025-001 documenta esta deuda).
- La documentación SddIA y varios scripts aún **referencian** GesFer.Console como si existiera.
- Esto provoca confusión y scripts rotos (p. ej. `pr-skill.sh` que invoca un proyecto inexistente).

### 3.2 Qué se va a LIMPIAR (referencias obsoletas)

| Área | Referencias a eliminar o sustituir |
|------|-----------------------------------|
| **SddIA/actions** | `GesFer.Console --spec`, `--clarify`, `--plan` → sustituir por referencias a **invoke-command**, **documentation** (paths.skillCapsules, paths.skillsDefinitionPath) |
| **SddIA/agents** | architect.json, clarifier.json, auditor/process-interaction.json |
| **SddIA/norms** | patterns-in-planning-implementation-execution.md |
| **SddIA/templates** | spec-template.md |
| **scripts/skills** | pr-skill.md, pr-skill.sh → invocación a GesFer.Console -- 11 |
| **SddIA/manifesto.json** | Utils "Contains Console, Seeds, Migrations" |

### 3.3 Qué NO se toca (falsos positivos)

| Elemento | Motivo |
|----------|--------|
| **Serilog.Sinks.Console** / `WriteTo.Console()` | Logging estándar a stdout. No es herencia monolito. |
| **Console.WriteLine** en InitDatabase.cs, generate-password-hash.cs, GenerateHash | Scripts de línea de comandos; salida legítima a terminal. |
| `--logger 'console;verbosity=detailed'` en ejecutar-tests.ps1 | Formato de logger de `dotnet test`, no GesFer.Console. |
| `gesfer-console_*.log` en docs/operations | Nombres de archivos de log; posible renombrar en otra tarea. |

## 4. Análisis de situación actual

- **pr-skill.sh** (línea 120): invoca `dotnet run --project src/Console/GesFer.Console.csproj --no-build -- 11` → **FALLA** porque el proyecto no existe.
- **SddIA** documenta acciones (spec, clarify, plan) como "implementadas mediante GesFer.Console" → proceso actual es **manual**.
- **manifesto.json** describe Utils con "Contains Console" → Admin.Back no tiene Utils/Console.

## 5. Ley aplicada

- **Ley de Soberanía:** docs/ y SddIA/ como SSOT; documentación debe reflejar la realidad del código.
- **Ley de Compilación:** El código roto (scripts que fallan) es inaceptable.

---

*Documento de objetivos. Ruta: paths.featurePath/refactorization-limpieza-referencias-consola/*
