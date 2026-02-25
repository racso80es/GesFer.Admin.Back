# Validación: run-tests-local (create-tool)

**Tarea:** create-tool-run-tests-local  
**Criterio:** La herramienta run-tests-local se ejecuta correctamente, produce JSON conforme a tools-contract y el agente no invoca dotnet test directamente.

## Verificaciones

| Id | Verificación | Resultado |
|----|--------------|-----------|
| V1 | Cápsula existe en paths.toolCapsules.run-tests-local | OK |
| V2 | Índice y Cúmulo actualizados con run-tests-local | OK |
| V3 | Invocación de la tool con -OnlyTests -TestScope unit produce exitCode 0 cuando unit tests pasan | OK (2026-02-24, last-run-unit.json) |
| V6 | TestScope integration en verde (OnlyTests -TestScope integration) | OK (2026-02-24, last-run-integration.json) |
| V7 | TestScope all o e2e en verde (requiere API en 5010) | Pendiente |
| V4 | Salida JSON contiene toolId, exitCode, success, timestamp, message, feedback, data, duration_ms | OK |
| V5 | Invocación con -TestScope e2e (caso de uso validación local) ejecuta E2E sin que el agente llame a dotnet test | OK (flujo vía tool) |

## Ejecución de validación

Se invoca la herramienta (no dotnet test directamente):

```powershell
.\scripts\tools\run-tests-local\Run-Tests-Local.bat -OnlyTests -TestScope unit -OutputJson
```

Resultado esperado: success true y exitCode 0 si los unit tests pasan; JSON válido según tools-contract.
