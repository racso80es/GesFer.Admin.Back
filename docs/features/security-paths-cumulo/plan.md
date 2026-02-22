# PLAN: Patrón paths Cúmulo para SddIA/security

**Proceso:** feature  
**Ruta (Cúmulo):** docs/features/security-paths-cumulo/  
**Especificación:** spec.md | **Clarificación:** clarify.md  
**Fecha:** 2026-02-21  
**Estado:** Implementado (commit feat(security): patron paths Cumulo para SddIA/security)

---

## 1. Objetivos del plan

- Añadir `securityPath` a cumulo.paths.json como ruta canónica para SddIA/security.
- Añadir mapeo [SEC] en cumulo.instructions.json para consumo por agentes.
- Declarar securityContract y pathsContract en security-engineer.json; instrucción Security items.
- Actualizar security-contract.md para referenciar paths.securityPath (Cúmulo) y custodio security-engineer.

---

## 2. Fases y tareas técnicas

### Fase 1 — Cúmulo paths e instructions

| Id  | Tarea | Criterio |
|-----|-------|----------|
| 1.1 | Modificar `SddIA/agents/cumulo.paths.json` | Añadir `"securityPath":"./SddIA/security/"` entre principlesPath y tokensPath. |
| 1.2 | Modificar `SddIA/agents/cumulo.instructions.json` | Añadir instrucción: `Map [SEC] -> paths.securityPath (items por UUID; custodio security-engineer).` |

### Fase 2 — Security-engineer y contrato

| Id  | Tarea | Criterio |
|-----|-------|----------|
| 2.1 | Modificar `SddIA/agents/security-engineer.json` | Añadir `securityContract` (SddIA/security/security-contract.json) y `pathsContract` (cumulo.paths.json). |
| 2.2 | Modificar `SddIA/agents/security-engineer.json` | Añadir instrucción: `Security items: consultar paths.securityPath (Cúmulo); custodio paths.securityPath.` |
| 2.3 | Modificar `SddIA/security/security-contract.md` | Sustituir rutas literales por referencia a `paths.securityPath` (Cúmulo); añadir custodio security-engineer. |

### Fase 3 — Verificación

| Id  | Tarea | Criterio |
|-----|-------|----------|
| 3.1 | Verificar estructura SddIA/security | Sin modificación de los 20 items existentes (spec.md y spec.json por UUID). |

---

## 3. Verificación final

- **Cúmulo:** cumulo.paths.json incluye securityPath; cumulo.instructions incluye Map [SEC].
- **Security-engineer:** Declara securityContract, pathsContract e instrucción Security items.
- **Contrato:** security-contract.md referencia paths.securityPath y custodio.
- **Regresión:** Sin cambios en carpetas de items (20 items conservan estructura).

---

## 4. Seguridad y trazabilidad

- paths.securityPath no expone rutas sensibles; es directorio de documentación.
- Karma2Token: items de security operan bajo security-contract.
- Rutas según Cúmulo (paths.featurePath, paths.securityPath).

---

## 5. Orden de ejecución (resumen)

1. Fase 1 — cumulo.paths.json y cumulo.instructions.json.  
2. Fase 2 — security-engineer.json y security-contract.md.  
3. Fase 3 — Verificación (opcional; sin cambios en items).

**Commit atómico:** feat(security): patron paths Cumulo para SddIA/security (ya ejecutado).

---

*Plan generado a partir de spec.md y clarify.md. Acción planning. Implementación completada.*
