# Contrato de Plantillas (SddIA/templates/)

**Alcance:** Todas las entidades bajo `SddIA/templates/`.

**Objetivo:** Disponer de configuraciones predefinidas (plantillas) que procedimenten el uso de procesos con un fin concreto, alineadas con los patrones de SddIA (skills, tools, actions, processes). Las plantillas facilitan arrancar una tarea con parámetros y orígenes de entrada ya definidos.

---

## 1. Propósito de las plantillas

- **Procedimentar:** Una plantilla asocia un **proceso** (feature, correccion-auditorias, etc.) con una **configuración encarada a un fin concreto**.
- **Entrada predefinida:** Origen de datos (rutas Cúmulo, rutas parciales o totales, ficheros) declarado en la plantilla.
- **Ciclo de vida:** La plantilla puede referenciar acciones y artefactos del proceso (objectives, spec, clarify, planning, etc.) para que el agente ejecute el ciclo completo.

Las plantillas son elementos de SddIA y cumplen la interfaz de proceso (al menos un `.md` y un `.json` por plantilla).

---

## 2. Estructura por plantilla

Cada plantilla reside en una carpeta nombrada con su **template_id** (kebab-case): `SddIA/templates/<template-id>/`.

### Archivos obligatorios

1. **`spec.md`**
   - **Contenido:** Descripción legible de la plantilla: propósito, proceso asociado, orígenes de entrada, uso, criterios de cierre.
   - **Idioma:** Español (es-ES).
   - **Formato:** Markdown estándar.

2. **`spec.json`**
   - **Contenido:** Metadatos estructurados para consumo por agentes.
   - **Esquema:** Definido en `templates-contract.json`.
   - **Campos clave:** template_id, name, description, process_ref, input_sources, config (opcional), related_actions, related_skills, contract_ref.

### Archivos opcionales

- **`config.json`**: Parámetros por defecto de la plantilla (valores que el usuario puede sobrescribir al instanciar la tarea).

---

## 3. Orígenes de entrada (input_sources)

Las plantillas declaran de dónde provienen los datos de la tarea:

- **Rutas Cúmulo:** Referencias canónicas (ej. `paths.auditsPath`, `paths.featurePath`). Resolver según Cúmulo.
- **Rutas parciales:** Relativas al repo (ej. `docs/audits/`, `docs/features/<nombre>/`).
- **Rutas totales:** Absolutas cuando se indique explícitamente (fichero concreto, URL de informe).
- **Fichero:** Nombre o patrón de fichero (ej. `AUDITORIA_*.md`, `validacion-*.json`).

El agente que ejecute la plantilla debe resolver estas referencias según la norma paths-via-cumulo cuando aplique.

---

## 4. Seguridad y trazabilidad

- **Karma2Token:** Toda ejecución de una plantilla debe operar bajo el contexto de un Karma2Token válido (paths.tokensPath; spec en SddIA/tokens/karma2-token/spec.json).
- **Consumidores:** paths.actionsPath, SddIA/agents, paths.processPath, .cursor/rules.

---

## 5. Referencias

- **Esquema JSON:** `SddIA/templates/templates-contract.json`
- **Proceso de creación:** `SddIA/process/create-template/` (spec.md, spec.json)
- **Cúmulo:** paths.templatesPath
