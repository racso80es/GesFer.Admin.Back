---
constraints:
- Cola, ACTIVE, DONE, KAIZEN y CLARIFY solo bajo paths.tasksPath (Cúmulo); no mezclar con rutas literales.
- Operaciones git según SddIA/norms/git-via-skills-or-process.md (skills, acciones o proceso).
- Por defecto el ciclo de implementación delega en el proceso feature; documentación de feature en paths.featurePath.
contract_ref: paths.processPath/process-contract.md
default_delegate_process: feature
default_delegate_ref: paths.processPath/feature/
description: Procesa una unidad de tarea del backlog (fichero o carpeta-tarea) con triaje, activación, ejecución y archivo.
name: Automatic Task
paths:
  evolution_ref: paths.evolutionPath y paths.evolutionLogFile (Cúmulo)
  featurePath_ref: paths.featurePath/<nombre_feature> (ciclo feature por defecto)
  tasksPath_ref: paths.tasksPath (Cúmulo)
persist_ref: paths.tasksPath
phases:
- description: Candidatos en raíz de paths.tasksPath y KAIZEN/; prioridad, fecha o indicación del usuario; sin colisión con ACTIVE/.
  id: '1'
  name: Identificación y triaje
- description: Ejecutar git-workspace-recon para validar entorno limpio. Crear rama feat/ o fix/ con git-branch-manager. Mover unidad a paths.tasksPath/ACTIVE/; consolidar con git-save-snapshot; publicación incremental con git-sync-remote según política de bloqueo.
  id: '2'
  name: Activación y bloqueo
- description: Ejecutar proceso objetivo (por defecto feature); leer carpeta-tarea si existe spec/plan; generar artefactos en paths.featurePath si aplica. Hitos atómicos con git-save-snapshot; emergencia git-tactical-retreat.
  id: '3'
  name: Ejecución
- description: Mover unidad a DONE/; Evolution Log; finalize-process.md cuando aplique. Cierre git-sync-remote y git-create-pr enlazando documentación de tarea al cuerpo del PR cuando corresponda.
  id: '4'
  name: Finalización y archivo
principles_ref: paths.principlesPath
process_id: automatic_task
process_interface_compliance: >-
  Unidades carpeta-tarea o documentación generada bajo paths.featurePath cumplen process_interface: .md con frontmatter YAML
  (objectives, spec, clarify, plan, implementation, execution, validacion). Sin .json separados en esa ruta. Patrón:
  SddIA/norms/features-documentation-pattern.md.
related_actions:
- triage
- activation
- execution
- finalization
related_skills:
- git-workspace-recon
- git-branch-manager
- git-save-snapshot
- git-sync-remote
- git-tactical-retreat
- git-create-pr
- invoke-command
spec_version: 2.0.0
---

# Proceso: Automatic Task

Este documento define el **proceso de tarea** para que una unidad de ejecución SDDIA procese una tarea del backlog de forma autónoma. Está ubicado en paths.processPath/automatic_task/ (Cúmulo). Asegura la integridad del repositorio y la visibilidad del progreso. Las rutas de cola y estados se obtienen de **Cúmulo** (paths.tasksPath); el ciclo de implementación por defecto usa paths.featurePath y el proceso **feature** (paths.processPath/feature/).

**Interfaz de proceso:** Las unidades **carpeta-tarea** bajo paths.tasksPath y la documentación generada al ejecutar el proceso **feature** cumplen la interfaz en Cúmulo (`process_interface`): artefactos **`.md`** con frontmatter YAML donde aplique el ciclo estándar. Patrón: SddIA/norms/features-documentation-pattern.md.

**Rutas de carpetas:** usar siempre la ruta de tareas del Cúmulo (`paths.tasksPath`), no literales fijos en documentación nueva. En disco, la carpeta puede coincidir con `docs/tasks/` o `docs/TASKS/` según el sistema de archivos; resolver siempre la ruta del contrato (paths.tasksPath).

## Fases del Proceso

### 1. Identificación y Triaje (Triage)

**1.1 Bandeja principal (tareas no Kaizen en cola)**  
Localiza en la **raíz** de `paths.tasksPath` los archivos `.md` sueltos (no dentro de subcarpetas). Entre ellos, elige el de prioridad más alta, el que el usuario indique o el de **fecha más antigua** (según nombre del fichero o metadatos en el contenido).

- Verifica que cumple con un análisis suficiente para poder realizar la tarea.
- Si la tarea no tiene un ID único (ej. T-26-001), asígnale uno basado en la fecha actual en el nombre del fichero o en su contenido.
- Comprueba que la tarea no está ya en ejecución (no existe en `paths.tasksPath/ACTIVE/` en ninguna rama activa ni master).

**1.2 Cola Kaizen (solo si 1.1 no devuelve ninguna tarea)**  
Si **no** hay ningún `.md` pendiente en la raíz de `paths.tasksPath`, revisa la subcarpeta **`paths.tasksPath/KAIZEN/`** (cola de tareas Kaizen ya especificadas).

- Si hay uno o más ficheros `.md` en `KAIZEN/`, selecciona **el más antiguo** (criterio preferente: prefijo de fecha en el nombre, p. ej. `Kaizen_YYYY_MM_DD_*.md`; alternativa: campo `created` / fecha en frontmatter del fichero).
- Esa tarea se ejecuta con el **mismo procedimiento** que una tarea normal (activación, ejecución, finalización; ver §2–4).

**1.3 Nueva Kaizen (solo si 1.1 y 1.2 no ofrecen trabajo)**  
- Si no hay tareas en la raíz de `paths.tasksPath` **ni** en `paths.tasksPath/KAIZEN/`, analiza el proyecto en busca de acciones de mejora continua (Kaizen), elige una, **regístrala** como nuevo fichero `.md` en `paths.tasksPath/KAIZEN/` (convención de nombre recomendada: `Kaizen_YYYY_MM_DD_<slug>.md`) y procédela igual que en §2–4.
- Comprueba que el kaicen (fichero u objetivo) no está ya en ejecución (no existe en `paths.tasksPath/ACTIVE/` en ninguna rama activa ni master).

### 2. Activación y Bloqueo (Activation)
Transición a estado `ACTIVE` para evitar colisiones con otras IAs (Jules/Cursor).

- **git-workspace-recon** antes de mutar el repositorio.
- Mueve el archivo de la tarea **desde su origen** (raíz de `paths.tasksPath` o `paths.tasksPath/KAIZEN/`) hacia `paths.tasksPath/ACTIVE/`.
- **Sincronización inmediata:** Consolidar la reubicación con **git-save-snapshot** y publicar con **git-sync-remote** en la rama gestionada por **git-branch-manager** (`feat/<nombre_feature>` o `fix/<nombre_fix>`). Esto bloquea el TODO.

### 3. Ejecución (Execution)
Inicia y continúa las instrucciones definidas en el proceso correspondiente, por defecto el proceso `feature` (definido en `SddIA/process/feature/spec.md`).

- Esto implica generar la documentación de la tarea (objectives, spec, clarify, plan, implementation, execution, validacion) en `paths.featurePath/<nombre_feature>`.
- Consolidar hitos con **git-save-snapshot**; ante corrupción de contexto, **git-tactical-retreat**.

### 4. Finalización y Archivo (Finalization)
Transición a estado `DONE` tras el cumplimiento del proceso.

- Mueve el archivo de la tarea de `paths.tasksPath/ACTIVE/` a `paths.tasksPath/DONE/`.
- Actualiza el log de evolución del producto (`paths.evolutionPath` / `paths.evolutionLogFile` según Cúmulo) con un resumen de la intervención, enlazando al archivo en `DONE/`.
- Genera la documentación de finalización del proceso feature (`finalize-process.md`).
- **git-sync-remote** y **git-create-pr** cuando el cierre deba publicar rama e integrar objetivos/validación en el cuerpo del PR.

## Estructura de carpetas requerida
Para el correcto funcionamiento de este proceso, el repositorio debe mantener la siguiente jerarquía bajo `paths.tasksPath`:

- Raíz de `paths.tasksPath` → Tareas pendientes generales (ficheros `.md` individuales en la raíz).
- `paths.tasksPath/KAIZEN/` → Cola de tareas Kaizen ya definidas y pendientes de ejecución.
- `paths.tasksPath/ACTIVE/` → Tareas en ejecución en la rama actual.
- `paths.tasksPath/CLARIFY/` → Tareas en ejecución que necesitan aclaración por parte del usuario.
- `paths.tasksPath/DONE/` → Histórico de éxito.

## Particularidades del proceso
- Trabajar de la forma más autónoma posible, con el fin de obtener la ejecución de la tarea sin supervisión del usuario. En caso de no ser posible este resultado, mover a la ruta de documentos a clarificar.

## Historial de versión del spec

- **2.0.0:** Arsenal táctico Git S+ (git-workspace-recon, git-branch-manager, git-save-snapshot, git-sync-remote, git-tactical-retreat, git-create-pr); obsolescencia de iniciar-rama/finalizar-git en este spec.
- **1.3.1:** Frontmatter alineado con process-contract (contract_ref, principles_ref, persist_ref, phases, paths, process_interface_compliance, related_skills); introducción estándar con ubicación e interfaz de proceso.
