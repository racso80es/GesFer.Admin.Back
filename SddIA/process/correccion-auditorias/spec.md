# Proceso: Corrección según Auditorías

Este documento define el **proceso de tarea** para la corrección de hallazgos derivados de auditorías (paths.auditsPath). Está ubicado en paths.processPath/correccion-auditorias/ (Cúmulo). La ruta de persistencia se obtiene de **Cúmulo** (paths.featurePath/<nombre_correccion>).

**Interfaz de proceso:** Cumple la interfaz en Cúmulo (`process_interface`): solicita/genera en la carpeta de la tarea (Cúmulo) al menos un **`.md`** (objectives.md, spec.md, clarify.md) y al menos un **`.json`** (spec.json, audit-hallazgos.json o similar, validacion.json).

## Propósito

El proceso **correccion-auditorias** orquesta el ciclo de corrección de hallazgos reportados en los informes de auditoría (paths.auditsPath): análisis de auditorías recientes, priorización de hallazgos, documentación de objetivos, y ejecución de correcciones mediante feature o bug-fix según el tipo de hallazgo.

## Entrada

- **Fuentes:** Informes en paths.auditsPath (p. ej. AUDITORIA_YYYY_MM_DD.md, validacion-*.json).
- **Artefacto de análisis:** Documento de objetivos (objectives.md) que consolida hallazgos, prioridades y alcance.

## Alcance

- **Rama:** feat/correccion-segun-auditorias o feat/correccion-auditorias-<identificador> (nunca master).
- **Documentación:** Carpeta paths.featurePath/<nombre_correccion>/ con objectives.md (objetivo, hallazgos consolidados, prioridades), spec.md/spec.json, clarify.md si aplica, implementation, validacion.json.
- **Skills:** iniciar-rama, documentation, invoke-command, security-audit cuando aplique.
- **Restricciones:** Priorizar hallazgos críticos (compilación, seguridad, violación de capas); alcance acotado por lo reportado en auditorías.

## Fases

1. **Análisis de auditorías:** Revisar últimos informes en paths.auditsPath y consolidar hallazgos (críticos / medios / bajos).
2. **Documentación de objetivos:** Redactar objectives.md con hallazgos priorizados y criterios de cierre.
3. **Especificación y plan:** Acciones spec, clarify, planning según ciclo feature.
4. **Implementación y ejecución:** Aplicar correcciones; cada hallazgo puede ser un ítem de implementation.
5. **Validación y cierre:** validacion.json; registrar en paths.auditsPath o Evolution Logs que los hallazgos fueron abordados.

## Integración

- Los agentes **Auditor (Back/Front/Process)** generan los informes en paths.auditsPath.
- El ciclo de corrección lo orquestan **Arquitecto** y **Tekton** usando este proceso.
- Referencia canónica de la tarea: paths.featurePath/<nombre_correccion>/ (Cúmulo). SSOT para esa ronda de corrección.

## Referencias

- paths.auditsPath (Cúmulo)
- paths.featurePath (Cúmulo)
- Proceso feature: paths.processPath/feature/
- AGENTS.md — Leyes Universales (COMPILACIÓN, GIT, Soberanía documental)
