---
process_id: audit-tool
spec_version: 3.0.0
tool_id: prepare-full-env
audit_date: 2026-04-27
audit_id: audit-2026-04-27-02
phase: 4
---

# Execution — Fase 4 (prepare-full-env)

**Working directory obligatorio:** raíz del repositorio (`c:\Proyectos\GesFer.Admin.Back`) — necesario para que `docker-compose.yml` resuelva correctamente frente a `prepare-env.json` → `dockerComposePath: docker-compose.yml`.

**Nota (2026-04-28):** La flag `--start-api` fue retirada de la tool `prepare-full-env` (feature `prepare-full-env-drop-start-api`). El caso T4 queda **obsoleto** para futuras ejecuciones.

## T1 — `--docker-only` + `--output-json`

```powershell
Set-Location 'c:\Proyectos\GesFer.Admin.Back'
.\scripts\tools\prepare-full-env\prepare_full_env.exe `
  --docker-only `
  --output-json `
  --config-path .\scripts\tools\prepare-full-env\prepare-env.json
```

**Salida capturada:** `paths.auditsPath/tools/prepare-full-env/T1-stdout.json`

## T2 — `--docker-only` + `--output-path` (sin `--output-json`)

```powershell
Set-Location 'c:\Proyectos\GesFer.Admin.Back'
.\scripts\tools\prepare-full-env\prepare_full_env.exe `
  --docker-only `
  --output-path .\docs\audits\tools\prepare-full-env\T2-result.json `
  --config-path .\scripts\tools\prepare-full-env\prepare-env.json
```

**Salida capturada:** `paths.auditsPath/tools/prepare-full-env/T2-result.json`

## T3 — `--no-docker` + `--output-json` (aislado; sin `--docker-only`)

```powershell
Set-Location 'c:\Proyectos\GesFer.Admin.Back'
.\scripts\tools\prepare-full-env\prepare_full_env.exe `
  --no-docker `
  --output-json `
  --config-path .\scripts\tools\prepare-full-env\prepare-env.json
```

**Salida capturada:** `paths.auditsPath/tools/prepare-full-env/T3-stdout.json`

## Evidencias Batch (post T1, aplicables a escenarios con Docker)

```powershell
Set-Location 'c:\Proyectos\GesFer.Admin.Back'
docker compose ps --format "table {{.Name}}\t{{.Service}}\t{{.Status}}\t{{.Ports}}"
```

→ `paths.auditsPath/tools/prepare-full-env/evidence-docker-compose-ps.txt`

```powershell
docker inspect gesfer_db --format '{{json .State.Health}}'
```

→ `paths.auditsPath/tools/prepare-full-env/evidence-mysql-inspect.txt`  
(`gesfer_db` coincide con `result.mysql_container` en T1/T2.)

---

## Fase 8 — Cierre y limpieza (reversión Docker)

**Directriz Director:** detener contenedores levantados durante la auditoría.

```powershell
Set-Location 'c:\Proyectos\GesFer.Admin.Back'
docker compose down
```

**Evidencia:** `paths.auditsPath/tools/prepare-full-env/evidence-docker-compose-down.txt`
