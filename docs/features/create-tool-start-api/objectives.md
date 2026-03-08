# Objetivos: create-tool-start-api

**Proceso:** create-tool  
**Rama:** feat/create-tool-start-api  
**tool-id:** start-api

## Objetivo principal

Poder hacer uso de una **herramienta de sistema programada en Rust** que permita **levantar la API** del proyecto GesFer.Admin.Back, completando el ciclo de vida junto con las herramientas existentes de infraestructura y entorno.

## Objetivos específicos

1. **Definir** la herramienta `start-api` según el proceso create-tool y el contrato `SddIA/tools/tools-contract.json`.
2. **Contrato adecuado:** especificación clara de entradas, salida JSON, fases de feedback y dependencias opcionales con `prepare-full-env` e `invoke-mysql-seeds`.
3. **Ejecución por terceros:** la herramienta debe poder ser invocada desde agentes, CI/CD o sistemas externos para gestionar la solución (levantar la API de forma trazada y con salida estándar).
4. **Implementación por defecto en Rust:** binario en paths.toolsRustPath, cápsula en paths.toolCapsules.start-api con manifest, config, documentación y launcher .bat/.ps1.

## Contexto

- **prepare-full-env:** prepara infraestructura (Docker, MySQL, cache, Adminer).
- **invoke-mysql-seeds:** prepara datos (migraciones y seeds).
- **start-api (nueva):** levanta la API del proyecto → cierra el ciclo: infra → datos → API.

## Criterios de éxito

- Definición en SddIA (spec.md, spec.json) con implementation_path_ref y contrato.
- Cápsula en paths.toolCapsules.start-api con manifest.json, config, doc, launcher.
- Índice (scripts/tools/index.json) y Cúmulo (toolCapsules) actualizados.
- Salida JSON conforme a tools-contract; ejecución bajo Karma2Token.
