---
feature_name: refactorization-skills-exe-root-json-io
created: 2026-03-20
process: feature
scope:
  in_scope:
    - Contrato v2 skills y tools (exe en raíz, sin bin/, sin ps1, JSON stdin/stdout).
    - Norma SSOT capsule-json-io.md compartida.
    - Actualización normas y reglas Cursor de invocación.
  out_scope:
    - Implementación Rust y migración de cápsulas (plan/execution posteriores).
functional_requirements:
  - RF-01: skills-contract y tools-contract en v2.0.0 con referencia cruzada al mismo envelope JSON.
  - RF-02: Documentar request/result por entidad en cada spec de skill/tool.
validation_criteria:
  - Contratos y normas alineados; sin referencias contradictorias a bin/ o ps1 como fallback del agente.
---

# Especificación: Cápsulas v2 — raíz, JSON stdin/stdout, envelope compartido

## 1. Decisiones de producto (cerradas)

| ID | Decisión |
|----|-----------|
| D1 | Transporte **A**: JSON único por **stdin**; respuesta única por **stdout**. |
| D2 | Alcance **skills y tools** (misma normativa). |
| D3 | **Corte limpio** `contract_version: 2.0.0`; sin período dual `data`/`bin` salvo nota histórica en norma. |
| D4 | **Envelope compartido:** `meta` + `request` (entrada); `meta` + `success` + `exitCode` + `message` + `feedback` + `result` + `duration_ms` (salida). Personalización solo dentro de `request` y `result`. **Token:** `meta.token` obligatorio cuando el security_model de la entidad exija Karma2Token. |
| D5 | **Sí:** agente → `.exe` + JSON; `.bat` solo humano; sin `.ps1`. |

## 2. SSOT del formato

**Archivo:** `SddIA/norms/capsule-json-io.md`

Los contratos `SddIA/skills/skills-contract.md` y `SddIA/tools/tools-contract.md` remiten a ese documento y suben a **2.0.0**.

## 3. Trabajo de ingeniería pendiente (fuera de este spec documental)

- Ajustar `install.ps1` de skills-rs y tools-rs para copiar `.exe` a la **raíz** de cada cápsula.
- Eliminar `.ps1` de cápsulas y actualizar `.bat` (solo humano, sin PowerShell).
- Implementar en cada binario lectura/escritura del envelope (y migrar campo `data` → `result` en tools).

## 4. Referencias

- `docs/features/refactorization-skills-exe-root-json-io/clarify.md` (decisiones cerradas).
- Cúmulo: paths.skillCapsules, paths.toolCapsules, paths.skillsRustPath, paths.toolsRustPath.
