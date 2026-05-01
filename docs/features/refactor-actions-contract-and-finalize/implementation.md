---
feature_name: refactor-actions-contract-and-finalize
created: "2026-05-01"
items:
  - id: IMPL-01
    action: modify
    path: SddIA/actions/actions-contract.md
    location: Frontmatter constraints + sección Restricciones / nueva subsección Invariantes de ejecución
    proposal: >-
      Añadir norma innegociable: las acciones no ejecutan código del SO ni scripts; solo orquestan skills (paths.skillCapsules)
      y tools (paths.toolCapsules) según contratos. Actualizar lista de action_id ejemplo: incluir finalize-process; eliminar finalize del listado canónico.
    dependencies: []
  - id: IMPL-02
    action: create
    path: SddIA/actions/finalize-process/spec.md
    location: Nueva carpeta
    proposal: >-
      Copiar desde finalize/spec.md; cambiar action_id a finalize-process; ajustar título y triggers conceptuales (cierre de proceso/tarea);
      eliminar referencias a finalize.json donde no aplique; reforzar solo Git S+ y verify-pr-protocol vía skill/tool.
    dependencies:
      - IMPL-01
  - id: IMPL-03
    action: delete
    path: SddIA/actions/finalize/spec.md
    location: —
    proposal: Eliminar tras confirmar contenido migrado a finalize-process (o eliminar carpeta finalize completa).
    dependencies:
      - IMPL-02
  - id: IMPL-04
    action: modify
    path: SddIA/process/feature/spec.md
    location: related_actions; tabla fase 8; cuerpo
    proposal: Sustituir finalize por finalize-process en listados y narrativa.
    dependencies:
      - IMPL-02
  - id: IMPL-05
    action: modify
    path: SddIA/process/bug-fix/spec.md
    location: related_actions; fases
    proposal: Igual que feature donde cite finalize.
    dependencies:
      - IMPL-04
  - id: IMPL-06
    action: modify
    path: SddIA/process/refactorization/spec.md
    location: related_actions; cuerpo
    proposal: finalize-process.
    dependencies:
      - IMPL-04
  - id: IMPL-07
    action: modify
    path: SddIA/process/create-tool/spec.md
    location: related_actions
    proposal: finalize-process.
    dependencies:
      - IMPL-04
  - id: IMPL-08
    action: modify
    path: SddIA/process/create-template/spec.md
    location: listas de acciones
    proposal: finalize-process.
    dependencies:
      - IMPL-04
  - id: IMPL-09
    action: modify
    path: SddIA/process/correccion-auditorias/spec.md
    location: related_actions; descripción cierre
    proposal: finalize-process.
    dependencies:
      - IMPL-04
  - id: IMPL-10
    action: modify
    path: SddIA/process/automatic_task/spec.md
    location: Menciones finalize / finalize.md
    proposal: Alinear a finalize-process y vocabulario de cierre.
    dependencies:
      - IMPL-04
  - id: IMPL-11
    action: modify
    path: SddIA/process/README.md
    location: Listado de acciones
    proposal: finalize-process.
    dependencies:
      - IMPL-04
  - id: IMPL-12
    action: modify
    path: SddIA/process/create-skill/spec.json
    location: related_actions; fases
    proposal: finalize-process si aparece finalize.
    dependencies:
      - IMPL-04
  - id: IMPL-13
    action: modify
    path: SddIA/norms/interaction-triggers.md
    location: Tabla #Action; relación subir/finalizar
    proposal: finalize-process y paths.actionsPath/finalize-process/.
    dependencies:
      - IMPL-02
  - id: IMPL-14
    action: modify
    path: SddIA/norms/git-via-skills-or-process.md
    location: Ejemplos de acciones
    proposal: Mencionar finalize-process.
    dependencies:
      - IMPL-13
  - id: IMPL-15
    action: modify
    path: SddIA/norms/pr-acceptance-protocol.md
    location: finalize → finalize-process
    proposal: Sustituir action_id en texto.
    dependencies:
      - IMPL-13
  - id: IMPL-16
    action: modify
    path: SddIA/actions/README.md
    location: Tabla
    proposal: Fila finalize-process; eliminar finalize.
    dependencies:
      - IMPL-02
  - id: IMPL-17
    action: modify
    path: SddIA/actions/sddia-difusion/spec.md
    location: Listado de acciones
    proposal: finalize-process.
    dependencies:
      - IMPL-16
  - id: IMPL-18
    action: modify
    path: SddIA/skills/git-operations/spec.md
    location: Claves YAML que citan finalize
    proposal: finalize-process o texto neutro «acción de cierre».
    dependencies:
      - IMPL-16
  - id: IMPL-19
    action: modify
    path: SddIA/templates/templates-contract.md
    location: Lista de acciones
    proposal: finalize-process.
    dependencies:
      - IMPL-16
  - id: IMPL-20
    action: modify
    path: SddIA/templates/correccion-auditorias-feature/spec.md
    location: Ciclo feature
    proposal: finalize-process.
    dependencies:
      - IMPL-16
  - id: IMPL-21
    action: modify
    path: .cursor/rules/action-suggestions.mdc
    location: Tabla action_id
    proposal: finalize-process; descripción cierre.
    dependencies:
      - IMPL-13
  - id: IMPL-22
    action: modify
    path: .cursor/rules/features-documentation-pattern.mdc
    location: Tabla finalize
    proposal: Opcional renombrar fila a finalize-process / finalize.md como artefacto de tarea (evaluar redacción mínima).
    dependencies:
      - IMPL-21
  - id: IMPL-23
    action: modify
    path: .cursor/rules/sddia-ssot.mdc
    location: Lista actions
    proposal: finalize-process.
    dependencies:
      - IMPL-21
  - id: IMPL-24
    action: modify
    path: AGENTS.md
    location: Tabla procesos / menciones finalize en instrucciones
    proposal: finalize-process donde corresponda.
    dependencies:
      - IMPL-21
  - id: IMPL-25
    action: modify
    path: AGENTS.norms.md
    location: Si existe listado de acciones
    proposal: Alinear.
    dependencies:
      - IMPL-24
  - id: IMPL-26
    action: create
    path: SddIA/evolution/<uuid>.md
    location: —
    proposal: Registro evolution vía sddia_evolution_register tras mutaciones SddIA.
    dependencies:
      - IMPL-01
      - IMPL-03
  - id: IMPL-27
    action: modify
    path: docs/features/refactor-actions-contract-and-finalize/validacion.md
    location: —
    proposal: Generar en fase validate con checks y resultado global.
    dependencies:
      - IMPL-25
---

# Notas para Tekton

- Tras **IMPL-03**, comprobar que no queden imports o enlaces rotos en docs históricos; los paths en `docs/features/*` antiguos pueden seguir mencionando «finalize» en narrativa sin bloquear DoD si la documentación canónica SddIA está unificada.
- Los binarios nuevos bajo `scripts/skills/**/*.exe` generados por `install.ps1` **no** deben committearse si el repo los ignora; si aparecen como untracked, valorar `.gitignore` existente antes del snapshot.
