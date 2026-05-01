---
feature_name: start-api-contract-alignment
created: 2026-05-01
items:
  - id: IMPL-01
    action: refactor
    path: scripts/tools-rs/src/bin/start_api.rs
    proposal: Reemplazar stub por flujo contractual init → port-check → port-kill? → build → launch → healthcheck → done; envelope capsule-json-io; salida tool v2.
    dependencies:
      - gesfer-capsule (try_read_capsule_request, write_capsule_response)
      - clap (CLI), serde (config + request overlay), url (health URL con override de puerto), reqwest::blocking (health GET)
  - id: IMPL-02
    action: dependency
    path: scripts/tools-rs/Cargo.toml
    proposal: Dependencia `url = "2.5"` para mutar host/puerto de healthUrl.
  - id: IMPL-03
    action: config
    path: scripts/tools/start-api/start-api-config.json
    proposal: Añadir `portBlocked` opcional de ejemplo (fail) alineado con spec.
  - id: IMPL-04
    action: doc
    path: SddIA/tools/start-api/spec.md
    proposal: Ruta fuente `scripts/tools-rs/src/bin/start_api.rs` (D10).
  - id: IMPL-05
    action: process
    path: SddIA/evolution/
    proposal: Registro evolution por modificación de spec SddIA.
---

# Implementación (doc): start-api-contract-alignment

## Touchpoints

| Ruta | Cambio |
|------|--------|
| `scripts/tools-rs/src/bin/start_api.rs` | Implementación completa: lectura config, merge CLI/request, puerto (TcpListener + netstat/taskkill Windows), `dotnet build` sln con heartbeat stderr, `dotnet run` con pipes y buffer 64 KiB, polling health, patrones MySQL, exitCode 0–8. |
| `scripts/tools-rs/Cargo.toml` | Crate `url`. |
| `scripts/tools/start-api/start-api-config.json` | `portBlocked` ejemplo. |
| `SddIA/tools/start-api/spec.md` | Ruta fuente corregida. |
| `SddIA/evolution/*` | Entrada modificacion spec. |

## Riesgos / notas

- **`std::process::exit`** en `finish()` evita ejecutar destructores; el proceso hijo `dotnet` debe seguir vivo tras éxito (comportamiento deseado en Windows al salir del padre).
- **Puerto IPv6** (`[::]:5010`): el parser `netstat` simplificado puede no matar todos los casos; IPv4 localhost es el objetivo principal del config actual.
- **TLS health**: cliente HTTP con `danger_accept_invalid_certs(true)` solo para entornos locales con certificados dev; si health es siempre HTTP en config, impacto nulo.

## Limpieza en error

Tras fallo de health o detección MySQL se invoca `child.kill()` y `wait` para no dejar servidor parcial escuchando.
