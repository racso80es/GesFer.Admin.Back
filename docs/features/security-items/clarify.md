# Clarificación: Items de Seguridad (SddIA/security/)

Este documento registra las aclaraciones y definiciones obtenidas durante la fase de planeación de la feature `create-security-items`.

## 1. Estructura y Contrato

**P:** ¿Debe seguir el patrón de `SddIA/patterns/`?
**R:** Sí. Se creará `SddIA/security/security-contract.json` y `SddIA/security/security-contract.md`. El esquema JSON debe incluir `id`, `title`, `category`, `tags`, `metadata`, `interested_agents` y referenciar a `Karma2Token` como obligatorio.

## 2. Contenido de los Items

**P:** ¿Cómo se debe generar el contenido Markdown?
**R:** Se debe generar contenido adecuadamente estructurado (Encabezado, Descripción, Riesgo, Mitigación, Referencias) basado en el contenido proporcionado, cumpliendo con los patrones del proyecto.

## 3. Agentes Interesados

**P:** ¿Qué agentes deben listarse?
**R:** Los items serán gestionados por el `security-engineer` y consumidos por `architect`, `auditor` y `tekton-developer`. Se debe filtrar la lista según la categoría del item (e.g., `DevSecOps` -> `tekton-developer`, `Architecture` -> `architect`).

## 4. Modelo de Seguridad

**P:** ¿Es obligatorio el token `Karma2Token`?
**R:** Sí. Todos los items de seguridad deben requerir un contexto de ejecución validado por `Karma2Token`.

## 5. Proceso

**P:** ¿Cuál es el proceso a seguir?
**R:** "Finaliza fase de objetivos, e inicia #especificación". Esto implica generar los artefactos de especificación (`spec.md`, `spec.json`) en `docs/features/security-items/` antes de proceder con la implementación.
