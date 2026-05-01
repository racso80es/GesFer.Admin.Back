---
feature_name: start-api-contract-alignment
created: 2026-05-01
purpose: Cerrar ambigüedades entre SddIA/tools/start-api/spec.md, start-api-config.json y el stub Rust actual antes del plan técnico
decisions:
  - id: D1
    decision: "El contrato de entrada es únicamente el de SddIA (ConfigPath, NoBuild, Profile, Port, PortBlocked, Output*). No se mantienen ni documentan working_dir/command del stub como API pública."
  - id: D2
    decision: "start-api-config.json existente es la base; se admite clave opcional portBlocked (fail|kill) con prioridad CLI > request JSON > config > fail."
  - id: D3
    decision: "Si Port (CLI o request) está presente, la URL efectiva de health es http://localhost:{Port}/health salvo que healthUrl en config defina explícitamente otra ruta de recurso; en ese caso solo se sustituye host/puerto de la URL de config por localhost:{Port}."
  - id: D4
    decision: "Si Port no se overridea, se usa healthUrl del JSON de configuración tal cual (incluye puerto y path, p. ej. /health)."
  - id: D5
    decision: "Build (cuando NoBuild=false) dotnet build sobre src/GesFer.Admin.Back.sln desde la raíz del repo (GESFER_REPO_ROOT o cwd), no solo el csproj aislado, para alinear con verify-pr-protocol y la solución real."
  - id: D6
    decision: "Arranque dotnet run con current_dir = apiWorkingDir (relativo a repo root), argumentos --no-build cuando NoBuild=true, --launch-profile {Profile} usando Profile del request/CLI o defaultProfile del config."
  - id: D7
    decision: "El proceso de la API queda en segundo plano (child) tras spawn; la herramienta espera en primer plano solo hasta health 200 o timeout. result incluye pid, port, url_base y healthy como en tools-contract; no se requiere reemplazar el stub de 'success al spawn'."
  - id: D8
    decision: "Detección MySQL (exitCode 8) según patrones citados en spec SddIA sobre salida acumulada del hijo durante la ventana de espera del healthcheck (buffer acotado razonable, p. ej. últimos N KiB o líneas, documentado en implementation.md)."
  - id: D9
    decision: "PortBlocked=kill en Windows netstat -ano + taskkill /PID según spec; si tras kill el puerto sigue ocupado, exitCode 3."
  - id: D10
    decision: "Corrección editorial en SddIA/tools/start-api/spec.md referencia de fuente a scripts/tools-rs/src/bin/start_api.rs (no src/start_api.rs)."
---

# Clarify: start-api-contract-alignment

## Preguntas que quedaban abiertas

### ¿Se conservan los flags del stub (`working_dir`, `command`)?

**No.** Evitan duplicar semántica frente a `apiWorkingDir` + `command` + perfiles en config. Toda invocación debe alinearse con la spec SddIA.

### ¿Cómo se combina `Port` con `healthUrl`?

- Con **Port** explícito: health = `http://localhost:{Port}` + path tomado de `healthUrl` (solo el path si se parsea la URL del config) o `/health` por defecto si el config no permite derivarlo — en la práctica el config actual usa path `/health`; se aplica **D3/D4**.

### ¿Qué solución compilar?

**`src/GesFer.Admin.Back.sln`** desde la raíz del repositorio, para coherencia con el resto de herramientas y con la estructura real del backend.

### ¿El binario debe mantener vivo el `dotnet run`?

Sí. El flujo contractual es lanzar, esperar health y devolver JSON de éxito con el servidor aún en ejecución (salvo que el health falle y se devuelva error sin dejar proceso huérfano ambiguo — detalle de limpieza en plan/implementation).

### ¿El JSON de ejemplo en cápsula incluye `portBlocked`?

Opcional. La implementación debe leerlo si existe; el ejemplo puede ampliarse en la fase de implementación para documentar el campo.

## Riesgos aceptados

- Entornos sin MySQL seguirán fallando con exitCode 8 hasta ejecutar prepare-full-env / invoke-mysql-seeds (ya descrito en spec).
- Liberar puerto con `kill` puede terminar procesos no deseados si el puerto es compartido; comportamiento aceptado según spec (usuario elige `fail` o `kill`).

## Siguiente paso del proceso

Elaborar **plan.md** (orden de tareas: refactor `start_api.rs`, helpers HTTP/puerto, alinear manifest/start-api.md y validación manual).
