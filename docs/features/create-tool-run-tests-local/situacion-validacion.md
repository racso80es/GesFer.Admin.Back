# Situación de validación — create-tool-run-tests-local

**Fecha:** 2026-02-24  
**Criterio de cierre:** Todos los tests (unit, integration, E2E) en verde al ejecutar `run-tests-local` con `TestScope all`.

## Estado por scope

| Scope         | Estado   | Evidencia |
|---------------|----------|-----------|
| **Unit**      | En verde | `Run-Tests-Local.bat -OnlyTests -TestScope unit` → exitCode 0, success true. Ver `last-run-unit.json`. |
| **Integration** | En verde | `Run-Tests-Local.bat -OnlyTests -TestScope integration` → exitCode 0, success true. Ver `last-run-integration.json`. |
| **E2E**       | Pendiente de confirmación | Requiere API en http://localhost:5010 (prepare-full-env o arranque en proceso). Ejecuciones con TestScope all o e2e en curso o por ejecutar. |
| **All**       | Pendiente de confirmación | `Run-Tests-Local.bat -TestScope all` tarda varios minutos (unit + integration + E2E). Salida en `last-run-all.json` cuando finalice. |

## Ejecuciones de referencia (2026-02-24)

- **Unit:** `last-run-unit.json` — exitCode 0, success true, duration_ms ~12s.
- **Integration:** `last-run-integration.json` — exitCode 0, success true, duration_ms ~16s.
- **E2E / All:** Ejecuciones lanzadas; resultado en `last-run-e2e.json` o `last-run-all.json` cuando terminen.

## Cambios en la herramienta

- La API se arranca en el mismo proceso (Start-Job) cuando `TestScope` es `all` o `e2e` y no se usa `-OnlyTests`, de modo que sigue en ejecución durante `dotnet test`.
- Variables `E2E_BASE_URL` y `E2E_INTERNAL_SECRET` se fijan para `TestScope all` y `e2e`.
- Limpieza del job de la API en el bloque `finally`.

## Conclusión

- **Unit e integration:** validados en verde con la herramienta.
- **E2E / All:** pendiente de confirmar que la ejecución completa termina con exitCode 0. Cuando `last-run-all.json` (o `last-run-e2e.json`) muestre success true y exitCode 0, actualizar `finalize.json` y dar la tarea por finalizada.
