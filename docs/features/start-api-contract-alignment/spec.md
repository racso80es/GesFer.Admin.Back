---
feature_name: start-api-contract-alignment
created: 2026-05-01
base:
  - objectives.md
  - SddIA/tools/start-api/spec.md
  - scripts/tools/start-api/start-api.md
scope:
  in_scope:
    - Implementar en Rust el flujo contractual de start-api (config, puerto, PortBlocked, build opcional, launch, healthcheck HTTP 200).
    - Parámetros CLI y objeto request del envelope capsule-json-io alineados con la spec (incl. variantes camelCase / PascalCase / snake_case donde la spec lo exige).
    - Códigos de salida y fases de feedback según SddIA/tools/start-api/spec.md.
    - Corregir documentación de ruta de fuente si la spec SddIA aún apunta a `src/start_api.rs` en lugar de `src/bin/start_api.rs`.
  out_scope:
    - Redefinir el contrato de la tool en SddIA más allá de ajustes editoriales.
    - Sustituir prepare-full-env / invoke-mysql-seeds.
functional_requirements:
  - id: FR-01
    text: El binario acepta las entradas documentadas en SddIA (ConfigPath, NoBuild, Profile, Port, PortBlocked, OutputPath, OutputJson) vía CLI y vía request JSON.
  - id: FR-02
    text: Prioridad PortBlocked — CLI > request > start-api-config.json > fail.
  - id: FR-03
    text: Antes del arranque, validar puerto; con PortBlocked=fail salir con código contractual; con kill intentar liberar el puerto en Windows según spec.
  - id: FR-04
    text: Si NoBuild es falso, ejecutar build de la solución/API según configuración; fallos mapean a exitCode 5.
  - id: FR-05
    text: Lanzar la API (dotnet run / perfil) y esperar hasta que health responda HTTP 200 o timeout; éxito solo con health OK.
  - id: FR-06
    text: Detectar indisponibilidad MySQL en salida/logs según spec y devolver exitCode 8 cuando aplique.
  - id: FR-07
    text: Salida JSON conforme a SddIA/norms/capsule-json-io.md y SddIA/tools/tools-contract.md (meta, success, exitCode, message, feedback, result, duration_ms).
non_functional_requirements:
  - id: NFR-01
    text: Mantener implementación en Rust bajo paths.toolsRustPath; ejecutable en raíz de la cápsula start-api.
  - id: NFR-02
    text: Sin nuevos .ps1 como entrega principal de la tool (estándar del repo).
touchpoints:
  - path: scripts/tools-rs/src/bin/start_api.rs
    note: Reemplazar/ampliar lógica actual (stub) para cumplir el contrato.
  - path: scripts/tools-rs/
    note: Posibles helpers compartidos (p. ej. lectura config, HTTP health) si ya existen en el crate.
  - path: scripts/tools/start-api/
    note: Config por defecto, manifest, start-api.md — coherencia con comportamiento real.
  - path: SddIA/tools/start-api/spec.md
    note: Solo si hace falta alinear ruta `implementation_path` / referencia a fuente.
validation_criteria:
  - Criterio local — build del crate tools-rs sin errores.
  - Criterio funcional — invocación de prueba con config válida alcanza success=true y health 200 cuando MySQL e infra estén disponibles (o documentar dependencia en validacion.md).
  - Criterio contractual — tabla de exitCode y fases feedback coincide con SddIA/tools/start-api/spec.md.
---

# Especificación: start-api-contract-alignment

## Contexto

La implementación actual (`start_api.rs`) solo acepta `working_dir`, `command`, `output_path`, `output_json`, lanza `cmd /C` y considera éxito si `spawn` tiene éxito. No implementa `SddIA/tools/start-api/spec.md` (puerto, kill, build, healthcheck, códigos 2–8, config JSON).

## Referencia normativa

Comportamiento objetivo: **`SddIA/tools/start-api/spec.md`** y envelope **`SddIA/norms/capsule-json-io.md`**. Detalle operativo y ejemplos: **`scripts/tools/start-api/start-api.md`**.

## Brecha resumida

| Aspecto | Spec SddIA | Implementación actual |
|---------|------------|------------------------|
| Entradas | ConfigPath, NoBuild, Profile, Port, PortBlocked, Output* | working_dir, command, Output* |
| Éxito | Health HTTP 200 | spawn OK |
| exitCode | 0–8 según tabla | 0 o 1 |
| Fases feedback | init → port-check → … → healthcheck | init, api |

## Entregable

Binario `start_api.exe` en la cápsula cuyo comportamiento y E/S sean auditables frente a la spec SddIA, sin depender del stub anterior para flujos agente/CI.
