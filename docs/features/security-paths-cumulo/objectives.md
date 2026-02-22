# Objetivos: Patrón paths Cúmulo para SddIA/security

**Proceso:** feature  
**Ruta (Cúmulo):** paths.featurePath/security-paths-cumulo/  
**Rama:** feat/security-paths-cumulo  
**Ley aplicada:** Ley GIT, Soberanía documental (Cúmulo SSOT).

---

## 1. Objetivo

Aplicar el patrón paths de Cúmulo a los items de seguridad en SddIA/security, declarando `paths.securityPath` como ruta canónica y asignando al agente **security-engineer** como custodio.

---

## 2. Cambios realizados

- **cumulo.paths.json:** Añadido `securityPath: "./SddIA/security/"`.
- **cumulo.instructions.json:** Añadido mapeo `[SEC] -> paths.securityPath` (items por UUID; custodio security-engineer).
- **security-engineer.json:** Añadido `securityContract`, `pathsContract`; instrucción "Security items: consultar paths.securityPath".
- **security-contract.md:** Sustituidas rutas literales por `paths.securityPath` (Cúmulo); referencia a custodio.

---

## 3. Alcance

- Sin modificación de los 20 items existentes (spec.md/spec.json por UUID).
- Consumidores futuros (spec, clarify, validate) pueden resolver rutas vía Cúmulo (paths.securityPath).

---

## 4. Referencias

- paths.securityPath (Cúmulo)
- SddIA/security/security-contract.json
- Agente: SddIA/agents/security-engineer.json
