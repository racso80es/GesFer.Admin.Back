# Clarificación: Limpieza de Referencias a Consola (Herencia Monolito)

**Proceso:** refactorization  
**Ruta:** docs/features/refactorization-limpieza-referencias-consola/  
**Fecha:** 2026-02-21  
**Especificación:** spec.md  

---

## 1. Gaps y ambigüedades resueltas

### 1.1 pr-skill.sh y invoke-command

**Gap:** ¿Debe pr-skill.sh invocar directamente `dotnet test` o pasar por invoke-command?

**Resolución:** pr-skill.sh es un script Bash que se ejecuta en pre-push (local) y CI. La skill invoke-command tiene implementación PowerShell (.ps1) y .bat. En entornos Bash (pr-skill.sh), el script ejecuta `dotnet test` directamente; la documentación (pr-skill.md) **referenciará** invoke-command y dotnet-development como skills que rigen los estándares de ejecución de comandos en el dominio. No se modifica la arquitectura de pr-skill (Bash) para invocar invoke-command desde Bash; la referencia es documental.

### 1.2 security-engineer.json y gesfer-console_*.log

**Gap:** ¿Eliminar la mención a gesfer-console_*.log en security-engineer.json?

**Resolución:** Mantener. `gesfer-console_*.log` es un **nombre de archivo** de logs (paths de directorio de operaciones), no referencia a la herramienta GesFer.Console. Se aclara en la documentación si genera confusión. No es touchpoint prioritario.

### 1.3 Orden de ejecución (spec → clarify → planning en acciones)

**Gap:** Al actualizar SddIA/actions (spec, clarify, planning), ¿se mantiene el flujo completo (validación token, ingesta, persistencia)?

**Resolución:** Sí. El flujo de cada acción se mantiene; solo se **sustituye** la implementación "GesFer.Console" por "documentación manual + invoke-command para comandos". Las secciones de Flujo, Integración con Agentes y Estándares permanecen; se reescribe la sección Implementación para referenciar invoke-command y documentation.

### 1.4 DT-2025-001 y decisión de cierre

**Resolución:** Cerrar con decisión: "Documentación actualizada; acciones referencian skills/tools. No se implementa GesFer.Console en Admin.Back. Opción 2 del documento original."

---

## 2. Sin gaps pendientes

No se identifican ambigüedades adicionales que bloqueen la fase de planning o implementation.

---

## 3. Referencias a skills/tools

| Elemento | Skill/Tool |
|----------|------------|
| Acciones spec, clarify, planning | invoke-command, documentation |
| Acción validate | invoke-command, dotnet-development |
| Acción finalize | finalizar-git, invoke-command |
| pr-skill (documentación) | invoke-command, dotnet-development |

---

*Clarificación generada manualmente. Cumple interfaz de proceso (clarify.md + clarify.json).*
