# Clarificación: Patrón paths Cúmulo para SddIA/security

**Proceso:** feature  
**Ruta (Cúmulo):** docs/features/security-paths-cumulo/  
**Fecha:** 2026-02-21  
**Especificación:** spec.md  

---

## 1. Gaps y ambigüedades resueltas

### 1.1 security-contract.json: ¿Literal path vs paths.securityPath?

**Gap:** El campo `folder_structure` en security-contract.json usa literal "SddIA/security/<uuid>/". ¿Actualizar a paths.securityPath?

**Resolución:** Mantener la ruta literal en security-contract.json. El contrato JSON es machine-readable y se consume en paths.securityPath; los agentes resuelven la ruta canónica vía Cúmulo (paths.securityPath). El documento human-readable (security-contract.md) ya referencia paths.securityPath y custodio. No se exige cambio en security-contract.json.

### 1.2 Consumidores spec/clarify/validate: consultas dinámicas

**Gap:** ¿Implementar SecurityScanner o checks que consulten paths.securityPath por items?

**Resolución:** Fuera de alcance en esta feature (spec §1.2). Evolución futura. No bloquea planning ni implementation.

### 1.3 Orden de implementación

**Gap:** ¿Orden de cambios en cumulo.paths.json, cumulo.instructions.json, security-engineer.json?

**Resolución:** Sin dependencias críticas; orden sugerido: cumulo.paths.json → cumulo.instructions.json → security-engineer.json → security-contract.md. Los cambios ya están aplicados en el commit anterior.

### 1.4 Regresión en items existentes (20 items)

**Gap:** ¿Validar que los 20 items en SddIA/security/ conservan spec.md y spec.json?

**Resolución:** Verificación opcional en fase validate. No se modifica estructura de carpetas ni contenido de items.

---

## 2. Sin gaps pendientes

No quedan ambigüedades que bloqueen planning o implementation. La feature está **implementada** (commit feat(security): patron paths Cumulo para SddIA/security).

---

## 3. Referencias a skills/herramientas

| Fase        | Skill / estándar        |
|-------------|-------------------------|
| Documentación | documentation, Cúmulo |
| Validación   | invoke-command (build, tests) |

---

*Clarificación generada en el marco de la acción clarify. Cumple interfaz de proceso (clarify.md + clarify.json).*
