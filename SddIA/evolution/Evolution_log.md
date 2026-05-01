# Evolution log — protocolo SddIA

Índice maestro de cambios registrados bajo `./SddIA/`. Rutas normativas: `paths.sddiaEvolutionPath`, `paths.sddiaEvolutionLogFile` (Cúmulo).

| ID (GUID) | Fecha | Descripción breve |
| :--- | :--- | :--- |
| 2a5a0e5a-4ecc-40c8-ac2c-8b7299ca637a | 2026-03-27T12:38:31.819376800+00:00 | Introduce norma evolution SddIA: Cúmulo, contrato v1.1, binarios Rust, CI y difusión IDE |
| d50066f8-59e9-48c8-83ef-c24f74efb78e | 2026-03-27T16:44:30.204823857+00:00 | Añadida acción alternativa Kaizen si docs/TASKS está vacío |
| f8e2d4c1-7b3a-4f9e-8c6d-1a2b3c4d5e6f | 2026-03-27T00:00:00+00:00 | automatic_task v1.1.0 — cola KAIZEN por antigüedad antes de nueva Kaizen; rutas paths.tasksPath |
| b6a79723-db30-461c-94aa-e98e8c1e90ec | 2026-04-17T00:00:00+00:00 | Alta tool run-test-e2e-local (E2E HTTP local); spec SddIA; toolCapsules en cumulo.paths |
| 31993fe2-3488-4fc7-9bdf-6dbeac94bf5c | 2026-04-28T17:44:34.054925400+00:00 | prepare-full-env: retirar StartApi/--start-api del spec |
| 515247ce-c0c4-425c-bd61-da3e9a5c2911 | 2026-04-28T17:57:00.428598500+00:00 | prepare-full-env: spec describe result (no data) |
| e37c5a45-043c-43a1-a42d-9a7950f9e901 | 2026-04-28T18:38:21.474315200+00:00 | modificacion: invoke-mysql-seeds spec añade DropCreateDb |
| 9d4a6c4b-8e52-4e2c-a1a7-1e4d9a6b7c31 | 2026-04-30T00:00:00+00:00 | alta: proceso create-skill + difusión (#Process) |
| 3c1d9d7a-1f8e-4c5d-8f2c-9a0a3d2b1c4e | 2026-04-30T00:00:00+00:00 | alta: skill git-workspace-recon (definición + integración Cúmulo) |
| 0c7a1b2e-7c2b-45f9-8f9c-0e05d8bff1c4 | 2026-04-30T00:00:00+00:00 | alta: skill git-branch-manager (definición + integración Cúmulo) |
| 8f3c2b1a-4d5e-4f90-9c1a-2b3c4d5e6f70 | 2026-04-30T00:00:00+00:00 | alta: skill git-save-snapshot (definición + integración Cúmulo) |
| 1b2c3d4e-5f60-4a7b-8c9d-0e1f2a3b4c5d | 2026-04-30T00:00:00+00:00 | alta: skill git-sync-remote (definición + integración Cúmulo) |
| 7a6b5c4d-3e2f-4a10-9b8c-7d6e5f4a3b2c | 2026-04-30T00:00:00+00:00 | alta: skill git-tactical-retreat (definición + integración Cúmulo) |
| 4d3c2b1a-0f9e-4c8d-8b7a-6e5d4c3b2a1f | 2026-04-30T00:00:00+00:00 | alta: skill git-create-pr (definición + integración Cúmulo) |
| 6f5e4d3c-2b1a-4c0d-9e8f-7a6b5c4d3e2f | 2026-04-30T00:00:00+00:00 | modificacion: interaction-triggers lista nuevas git skills |
| 9a8b7c6d-5e4f-4a3b-9c2d-1e0f9a8b7c6d | 2026-05-01T00:00:00+00:00 | alta: guía reproducir-skills-en-otros-entornos-sddia.md (paths.skillsDefinitionPath) |
| c2d3e4f5-a6b7-48c9-d0e1-f2a3b4c5d6e7 | 2026-05-01T00:00:00+00:00 | alta: guía portabilidad proceso create-skill; ajuste spec; enlace guía skills |
| 79a1f7a0-d30c-444b-82b3-92af90394769 | 2026-05-01T11:16:44.850285200+00:00 | modificacion: spec run-test-e2e-local (user CRUD) y proceso feature (evaluacion impacto SddIA pre-sync/PR) |
| 176945b3-e741-48ff-9d97-30d562945054 | 2026-05-01T11:42:32.083145+00:00 | Renombrar acción finalize a finalize-process; blindar actions-contract (solo orquestación skills/tools) |
| 840c1d87-9086-4775-b351-c133dc51468b | 2026-05-01T11:53:01.606923700+00:00 | Hotfix git-sync-remote: upstream + push -u; norma git-operations Ley de Hierro |
| 8aba101c-93b5-4b4f-b053-4108e952688c | 2026-05-01T14:05:29.628359200+00:00 | alta: skill git-close-cycle; finalize-process orquesta post-fusion |
| b8c7d6e5-f4a3-4b2c-9d8e-7f6a5b4c3d2e | 2026-05-01T00:00:00+00:00 | alta: cápsula verify-pr-protocol + index + Cúmulo + difusión |
| e4d5f6a7-b8c9-40d0-9f1a-2b3c4d5e6f7a | 2026-05-01T00:00:00+00:00 | modificacion: spec start-api ruta fuente Rust src/bin/start_api.rs |
