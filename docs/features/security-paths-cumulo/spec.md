# SPEC: Patrón paths Cúmulo para SddIA/security

**ID de Especificación:** SPEC-SEC-PATHS-2026-01  
**Rama:** feat/security-paths-cumulo  
**Estado:** Draft  
**Contexto (Cúmulo):** paths.featurePath/security-paths-cumulo/

---

## 1. Propósito y Contexto

### 1.1 Objetivo

Integrar SddIA/security en el patrón paths de Cúmulo: rutas canónicas vía `paths.securityPath`, custodia por el agente security-engineer, y mapeo [SEC] en cumulo.instructions para consumo por agentes y acciones.

### 1.2 Alcance (Scope)

**Incluido:**
- Añadir `securityPath` a cumulo.paths.json.
- Añadir mapeo [SEC] a cumulo.instructions.json.
- Declarar securityContract y pathsContract en security-engineer.json.
- Actualizar security-contract.md para referenciar paths.securityPath (Cúmulo).

**Fuera de alcance:**
- Modificación de los 20 items existentes en SddIA/security.
- Implementación de consultas dinámicas a paths.securityPath en spec/clarify/validate (evolución futura).

---

## 2. Arquitectura y Diseño Técnico

### 2.1 Rutas (paths.securityPath)

- **Ruta:** `./SddIA/security/` (paths.securityPath en Cúmulo).
- **Contrato:** security-contract.json en paths.securityPath.
- **Items:** paths.securityPath/<uuid>/ con spec.md y spec.json por item.

### 2.2 Custodio

- **Agente:** security-engineer.
- **Contratos:** securityContract (SddIA/security/security-contract.json), pathsContract (cumulo.paths.json).
- **Uso:** Al activarse (Auth, Login, Seeds, Inputs, etc.), consultar paths.securityPath para items relevantes por categoría/interested_agents.

### 2.3 Consumidores

| Consumidor | Uso de paths.securityPath |
|------------|---------------------------|
| security-engineer | Consulta items por categoría; checklist de seguridad. |
| spec, clarify (futuro) | SecurityScanner puede validar requisitos frente a items. |
| validate (futuro) | Checks opcionales de seguridad vía items. |

---

## 3. Requisitos de Seguridad

- Karma2Token: Los items de security operan bajo contexto Karma2Token (security-contract).
- paths.securityPath no expone rutas sensibles; es directorio de documentación.

---

## 4. Criterios de Aceptación

- [ ] cumulo.paths.json incluye securityPath.
- [ ] cumulo.instructions.json incluye mapeo [SEC].
- [ ] security-engineer.json declara securityContract y pathsContract; instrucción Security items.
- [ ] security-contract.md referencia paths.securityPath (Cúmulo) y custodio security-engineer.
- [ ] Sin regresión en estructura existente de SddIA/security (20 items).

---

## 5. Trazabilidad

- **Objetivos:** docs/features/security-paths-cumulo/objectives.md
- **Análisis previo:** Análisis de items SddIA/security y valoración patrón paths Cúmulo.
