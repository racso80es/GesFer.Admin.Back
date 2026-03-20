---
feature_name: refactorization-skills-exe-root-json-io
created: 2026-03-20
closed_at: 2026-03-20
purpose: >-
  Decisiones cerradas para contrato v2 de cápsulas (skills + tools): transporte JSON,
  layout exe, ausencia de ps1, rol del bat, envelope compartido.
decisions:
  - id: D1
    topic: Transporte JSON (entrada/salida) para invocación por IA
    status: cerrada
    resolution: >-
      Opción A: una petición JSON por stdin (UTF-8), una respuesta JSON por stdout.
      Sin ficheros obligatorios de intercambio.
  - id: D2
    topic: Alcance — skills y toolCapsules
    status: cerrada
    resolution: >-
      Skills y tools: misma norma de envelope y misma política de cápsula (exe en raíz, sin ps1).
  - id: D3
    topic: Versión de contrato (contract_version) y ventana de compatibilidad
    status: cerrada
    resolution: >-
      Corte limpio: skills-contract y tools-contract pasan a contract_version 2.0.0.
      Salida tools campo data sustituido por result (documentado en capsule-json-io.md).
  - id: D4
    topic: Esquema JSON compartido y extensión por skill/tool
    status: cerrada
    resolution: >-
      SSOT en SddIA/norms/capsule-json-io.md. Estructura fija meta + request (entrada) y
      meta + success + exitCode + message + feedback + result + duration_ms opcional (salida).
      Parámetros y salidas específicos solo en request y result. meta.token para Karma2Token
      cuando el security_model de la entidad lo exija.
  - id: D5
    topic: Comportamiento del .bat (humano) frente al .exe (agente)
    status: cerrada
    resolution: >-
      Confirmado: IA invoca siempre el .exe con JSON; .bat opcional solo para humanos;
      sin .ps1 en cápsulas.
---

# Clarificación: Skills y tools — exe en raíz, E/S JSON, sin PS1

**Persistencia:** `paths.featurePath/refactorization-skills-exe-root-json-io/` (Cúmulo)

## Decisiones cerradas (2026-03-20)

Las cinco decisiones (D1–D5) quedaron **cerradas** según el frontmatter. La especificación técnica del envelope está centralizada en **`SddIA/norms/capsule-json-io.md`**. Los contratos **`SddIA/skills/skills-contract.md`** y **`SddIA/tools/tools-contract.md`** están versionados en **2.0.0** y referencian esa norma.

## Siguiente paso recomendado

Redactar **`plan.md`** (fases: actualizar install scripts, Rust, cápsulas, manifiestos, pruebas) y ejecutar **`implementation.md` / execution** según el proceso feature.

## Resumen ejecutivo

| Tema | Acuerdo |
|------|---------|
| Transporte | stdin / stdout JSON |
| Alcance | Skills + tools |
| Compatibilidad | Corte limpio v2 |
| Payload | `request` / `result` + feedback compartido; token en `meta.token` si aplica |
| Launchers | `.exe` agente; `.bat` humano; sin `.ps1` |
