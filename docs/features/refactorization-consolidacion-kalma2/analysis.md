# Análisis: Referencias a Kalma2 en la solución

## Resumen ejecutivo

En el repositorio **GesFer.Admin.Back** existen dos familias de referencias que conviene distinguir:

| Término    | Significado                         | Alcance de este proceso |
|-----------|--------------------------------------|---------------------------|
| **Kalma2** | Nombre del proyecto/sistema (Constitución, rutas tipo `src/Kalma2`) | **Sí — eliminar/sustituir** |
| **Karma2** | Token de seguridad (Karma2Token) en contratos SddIA                | **No — se mantiene**       |

El código fuente bajo `src/` **no contiene** ninguna carpeta `Kalma2`; la solución es 100% .NET (GesFer.Admin.Back.*). Las referencias a Kalma2 aparecen solo en documentación (SddIA, constitución, skills, acciones, docs/features).

---

## 1. Ubicaciones de referencias a "Kalma2" (proyecto/rutas)

### 1.1 Constitución y arquitectura

| Archivo | Contenido relevante |
|---------|----------------------|
| `SddIA/CONSTITUTION.md` | Título "Constitución del Proyecto: Kalma2"; múltiples párrafos con "Kalma2" (Core, MCP, Consciencia, etc.). |
| `SddIA/constitution/constitution.architect.md` | "Kalma2" y rutas `src/Kalma2`, `src/Kalma2/Interface/Desktop`, `src/Kalma2/Core/Contracts`. |
| `SddIA/constitution/constitution.architect.json` | `location`, `contractsPath` y descripción con `src/Kalma2`. |
| `SddIA/constitution/constitution.audity.md` | Logger y reglas en `src/Kalma2/Core/Audit/...`. |
| `SddIA/constitution/constitution.audity.json` | `loggerPath`, `rulesPath` con `src/Kalma2/...`. |
| `SddIA/constitution/constitution.cognitive.md` | "Dotar a Kalma2 de una capacidad de memoria"; almacén `src/Kalma2/Core/Memory`. |
| `SddIA/constitution/constitution.cognitive.json` | `storagePath`: `src/Kalma2/Core/Memory`. |
| `SddIA/constitution/constitution.duality.md` | IPC y modos en `src/Kalma2/Desktop/...` y `src/Kalma2/Core/Modes`. |
| `SddIA/constitution/constitution.duality.json` | `ipcBridge`, `modesPath` con `src/Kalma2/...`. |

### 1.2 Acciones y skills

| Archivo | Contenido relevante |
|---------|----------------------|
| `SddIA/actions/clarify/spec.md` | Feature en `Kalma2/Docs/Feature/` y carpeta `Kalma2/Docs/Feature/{SpecName}/`. |
| `SddIA/skills/frontend-test/spec.md` | "Vitest para Kalma2 Desktop". |
| `SddIA/skills/frontend-test/spec.json` | `cwd`: `Kalma2/Interfaces/Desktop` en varios ítems de test. |

### 1.3 Otros

- No se han encontrado referencias a "Kalma2" en código C# bajo `src/`.

---

## 2. Referencias a "Karma2" (token) — excluidas del alcance

Las apariciones de **Karma2Token** / **karma2-token** en contratos (skills, tools, actions, process, patterns, principles, templates), normas (entidades-dominio, commands-via-skills, git-via-skills, obediencia-procesos), agentes (security-engineer, auditor), y docs/features (auditoría-interacciones, security-items, etc.) **no** se modifican en este proceso de consolidación, salvo decisión explícita en contrario.

---

## 3. Inventario de archivos a tocar (solo Kalma2)

Para el proceso de consolidación, los archivos que requieren cambio o revisión son (**10 archivos**; skill frontend-test excluida por clarify 1.2):

1. `SddIA/CONSTITUTION.md`
2. `SddIA/constitution/constitution.architect.md`
3. `SddIA/constitution/constitution.architect.json`
4. `SddIA/constitution/constitution.audity.md`
5. `SddIA/constitution/constitution.audity.json`
6. `SddIA/constitution/constitution.cognitive.md`
7. `SddIA/constitution/constitution.cognitive.json`
8. `SddIA/constitution/constitution.duality.md`
9. `SddIA/constitution/constitution.duality.json`
10. `SddIA/actions/clarify/spec.md`

**Fuera de alcance:** `SddIA/skills/frontend-test/spec.md`, `SddIA/skills/frontend-test/spec.json` (no se modifican en esta refactorización).

---

## 4. Próximos pasos recomendados

1. **Spec:** Redactar spec.md/spec.json de la refactorización (acción spec) con touchpoints por archivo y reglas de sustitución (nombre del proyecto, rutas por Cúmulo o por estructura real).
2. **Clarify:** Resolver dudas (p. ej. nombre final del proyecto en Constitución: "GesFer.Admin.Back", "GesFer", "el proyecto").
3. **Planning:** Plan de cambios por archivo y orden de edición.
4. **Implementation (doc):** Documento de implementación con ítems concretos.
5. **Execution:** Aplicar cambios en repo.
6. **Validate:** Búsqueda final de "Kalma2" y rutas; build/docs OK.
7. **Finalize:** Cierre según proceso (skill finalizar-git cuando corresponda).

---

*Análisis generado para el proceso de consolidación (refactorization-consolidacion-kalma2). Rutas según Cúmulo (paths.featurePath).*
