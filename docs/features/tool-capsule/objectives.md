# Objetivos: Unificación de Herramientas (Tool-Capsule)

## 1. Propósito
Transformar las herramientas actuales de scripts dispersos a **Artefactos Atómicos** encapsulados en directorios propios. Esto facilitará la portabilidad, evitará colisiones de configuración y permitirá una validación más estricta por parte de los agentes de QA.

## 2. Herramientas Identificadas para Migración
* **Invoke-MySqlSeeds**: Encargada de migraciones EF y seeds de base de datos.
* **Prepare-FullEnv**: Encargada de levantar el entorno Docker y la API.

## 3. Metas Técnicas (Definición de Hecho)
* [x] **Aislamiento Físico**: Cada herramienta debe residir en su propia cápsula **paths.toolCapsules[&lt;tool-id&gt;]** (Cúmulo).
* [x] **Estandarización de Interfaz**: Implementar un `manifest.json` por cápsula que describa sus componentes.
* [x] **Consistencia Documental**: Los archivos `.md` de cada herramienta deben trasladarse a su cápsula correspondiente.
* [x] **Actualización de Cúmulo**: El agente **Cúmulo** debe reflejar las nuevas rutas para mantener el SSOT (Single Source of Truth).
* [x] **Compatibilidad de Contrato**: Asegurar que la salida JSON siga cumpliendo el `SddIA/tools/tools-contract.json`.

## 4. Restricciones
* **Entorno**: Mantener compatibilidad estricta con Windows 11 y PowerShell 7+.
* **Compilación**: La herramienta en Rust (si existe) debe seguir siendo ejecutable desde el nuevo `bin/` local.

## 5. Ley aplicada
- **Soberanía:** `docs/` y `SddIA/` como verdad de referencia; contrato en `SddIA/tools/tools-contract.json`.
- **Cúmulo:** Rutas de herramientas como SSOT: **paths.toolsPath**, **paths.toolCapsules** en `SddIA/agents/cumulo.json`. En documentación .md usar estas referencias, no rutas literales.

## 6. Referencia de proceso
- Proceso: **feature** (`SddIA/process/feature.md`).
- Persistencia: **paths.featurePath** tool-capsule (Cúmulo).
