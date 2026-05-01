---
feature_name: create-skill-git-workspace-recon
created: "2026-04-30"
base:
  - objectives.md
functional_requirements:
  - "FR1: Ejecutar `git status -s` y devolver entradas parseadas."
  - "FR2: Ejecutar `git diff --stat` y devolver resumen parseado."
  - "FR3: Permitir `request.target_path` opcional para ejecutar en otra ruta."
non_functional_requirements:
  - "RNF1: Capturar stdout y stderr de cada comando."
  - "RNF2: Estados no críticos (salidas vacías / 'Everything up-to-date') no deben fallar."
  - "RNF3: Cumplir envelope JSON v2 (capsule-json-io) para agentes."
validation_criteria:
  - "VC1: Con workspace limpio, success=true y listas vacías."
  - "VC2: Con cambios locales, result contiene archivos detectados por status y diff --stat."
  - "VC3: Si target_path no es repo git, success=false con feedback error en fase git."
---

## Especificación técnica

### Entrada (envelope `request`)

```json
{
  "target_path": "c:\\ruta\\opcional\\a\\repo"
}
```

- `target_path` (string, opcional): directorio donde ejecutar Git. Si no se indica, se usa el directorio actual del proceso.

### Lógica interna

1. Resolver `cwd`:
   - si `target_path` existe y no es vacío → `Command::current_dir(target_path)`
   - si no → `cwd` actual
2. Ejecutar:
   - `git status -s`
   - `git diff --stat`
3. Parsear salidas y construir `result`.

### Salida (envelope `result`)

```json
{
  "targetPath": "c:\\ruta\\resuelta",
  "status": {
    "raw": "<stdout+stderr>",
    "entries": [
      { "code": "M", "path": "file.txt" }
    ]
  },
  "diffStat": {
    "raw": "<stdout+stderr>",
    "files": [
      { "path": "file.txt", "insertions": 2, "deletions": 1, "changes": 3 }
    ]
  }
}
```

### Manejo de estados no críticos

- Si `git status -s` o `git diff --stat` devuelve salida vacía con `exitCode=0`, se considera **éxito**.
- La skill solo falla si un comando devuelve `exitCode != 0` (p. ej. no es repo git).

