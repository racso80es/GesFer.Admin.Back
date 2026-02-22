# IMPL: Patrón paths Cúmulo para SddIA/security

**Plan:** plan.md | **SPEC:** spec.md | **Clarificación:** clarify.md  
**Ruta (Cúmulo):** docs/features/security-paths-cumulo/  
**Rama:** feat/security-paths-cumulo  
**Estado:** Ejecutado (commit feat(security): patron paths Cumulo para SddIA/security)

---

## Ítems de implementación

### Fase 1 — Cumulo paths e instructions

#### 1.1 – Modificar: cumulo.paths.json

- **Id:** 1.1
- **Acción:** Modificar
- **Ruta:** `SddIA/agents/cumulo.paths.json`
- **Cambio aplicado:** Añadido `"securityPath":"./SddIA/security/"` entre principlesPath y tokensPath.
- **Estado:** ✅ Ejecutado

#### 1.2 – Modificar: cumulo.instructions.json

- **Id:** 1.2
- **Acción:** Modificar
- **Ruta:** `SddIA/agents/cumulo.instructions.json`
- **Cambio aplicado:** Añadida instrucción `Map [SEC] -> paths.securityPath (items por UUID; custodio security-engineer).`
- **Estado:** ✅ Ejecutado

---

### Fase 2 — Security-engineer y contrato

#### 2.1 – Modificar: security-engineer.json (contratos)

- **Id:** 2.1
- **Acción:** Modificar
- **Ruta:** `SddIA/agents/security-engineer.json`
- **Cambio aplicado:** Añadidos `securityContract` (SddIA/security/security-contract.json) y `pathsContract` (SddIA/agents/cumulo.paths.json).
- **Estado:** ✅ Ejecutado

#### 2.2 – Modificar: security-engineer.json (instrucción)

- **Id:** 2.2
- **Acción:** Modificar
- **Ruta:** `SddIA/agents/security-engineer.json`
- **Cambio aplicado:** Añadida instrucción `Security items: consultar paths.securityPath (Cúmulo); custodio paths.securityPath.`
- **Estado:** ✅ Ejecutado

#### 2.3 – Modificar: security-contract.md

- **Id:** 2.3
- **Acción:** Modificar
- **Ruta:** `SddIA/security/security-contract.md`
- **Cambio aplicado:** Sustituidas rutas literales por referencia a `paths.securityPath` (Cúmulo); añadida referencia a custodio security-engineer.
- **Estado:** ✅ Ejecutado

---

### Fase 3 — Verificación

#### 3.1 – Verificar: estructura SddIA/security

- **Id:** 3.1
- **Acción:** Verificar
- **Propuesta:** Sin modificación de los 20 items existentes (carpetas por UUID con spec.md y spec.json). Verificación opcional en fase validate.
- **Estado:** ✅ Cumplido (no se modificaron items)

---

## Verificación final (execution)

- **Cúmulo:** cumulo.paths.json incluye securityPath; cumulo.instructions incluye Map [SEC]. ✅
- **Security-engineer:** securityContract, pathsContract e instrucción Security items. ✅
- **Contrato:** security-contract.md referencia paths.securityPath y custodio. ✅

---

## Orden de ejecución y commits

| Orden | Ítems   | Commit |
|-------|---------|--------|
| 1     | 1.1–2.3 | feat(security): patron paths Cumulo para SddIA/security |

---

*Implementación alineada con plan.md y clarify.md. paths.featurePath/security-paths-cumulo/ (Cúmulo).*
