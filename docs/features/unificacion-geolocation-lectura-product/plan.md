---
feature_name: unificacion-geolocation-lectura-product
created: 2026-03-20
updated: 2026-03-20
git_commit_policy: >-
  Un commit por fase (P1..P6), tras build coherente de esa fase.
  Mensaje sugerido: feat(geolocation): <fase> — <resumen breve> (Conventional Commits).
phases:
  - id: P1
    name: Contrato Application (DTOs y comandos)
    order: 1
    commit_message_example: "feat(geolocation): P1 DTOs geo y comando postal"
  - id: P2
    name: Handlers y filtrado IsActive + padres
    order: 2
    depends_on: [P1]
    commit_message_example: "feat(geolocation): P2 handlers geo IgnoreQueryFilters + IsActive"
  - id: P3
    name: API (controlador único y limpieza legacy)
    order: 3
    depends_on: [P2]
    commit_message_example: "feat(geolocation): P3 GeolocationController y eliminación legacy"
  - id: P4
    name: Tests DDD (arquitectura / dependencias entre capas)
    order: 4
    depends_on: [P3]
    commit_message_example: "feat(geolocation): P4 suite tests arquitectura DDD"
  - id: P5
    name: Pruebas funcionales (integración, unitarios geo, Swagger)
    order: 5
    depends_on: [P4]
    commit_message_example: "feat(geolocation): P5 tests integración y unitarios geolocation"
  - id: P6
    name: Cierre documentación de tarea
    order: 6
    depends_on: [P5]
    commit_message_example: "docs(geolocation): P6 implementation execution validacion"
tasks:
  - id: T1
    phase: P1
    title: Definir DTOs de lectura geo según spec §3
    detail: "CountryGeoReadDto (Id, Name, Code); StateGeoReadDto (Id, CountryId, Name, Code); CityGeoReadDto (Id, StateId, Name); PostalCodeGeoReadDto (Id, CityId, Code). Sustituir o renombrar DTOs actuales en GeoDtos.cs; actualizar comandos MediatR que referencien tipos antiguos."
  - id: T2
    phase: P1
    title: Comando y contrato para postal codes
    detail: "Añadir GetPostalCodesByCityIdCommand (o nombre alineado al proyecto) en GeoQueries.cs; IRequest<List<PostalCodeGeoReadDto>>."
  - id: T3
    phase: P2
    title: Refactor handlers países y país por id
    detail: "En cada query: IgnoreQueryFilters(). Where(IsActive). Lista ordenada por Name; by-id además Where(Id). Proyección a DTO §3.1. Eliminar cualquier DeletedAt == null del handler."
  - id: T4
    phase: P2
    title: Refactor handler estados por país
    detail: "IgnoreQueryFilters(). Where(s => s.CountryId == countryId && s.IsActive). Orden por Name. Proyección StateGeoReadDto."
  - id: T5
    phase: P2
    title: Refactor handler ciudades por estado
    detail: "IgnoreQueryFilters(). Where(c => c.StateId == stateId && c.IsActive). Orden por Name. Proyección CityGeoReadDto (sin Code)."
  - id: T6
    phase: P2
    title: Implementar handler códigos postales por ciudad
    detail: "IgnoreQueryFilters(). Where(p => p.CityId == cityId && p.IsActive). Orden por Code. Proyección PostalCodeGeoReadDto (sin Name)."
  - id: T7
    phase: P3
    title: Crear GeolocationController
    detail: "[Route(api/geolocation)], [AuthorizeSystemOrAdmin], cinco acciones GET alineadas a spec §2; inyectar IMediator; manejo de errores coherente con otros controladores."
  - id: T8
    phase: P3
    title: Eliminar controladores legacy
    detail: "Borrar CountriesController.cs y StatesController.cs; revisar Program.cs o registro de ensamblado si hubiera referencias explícitas."
  - id: T13
    phase: P4
    title: Proyecto o carpeta de tests de arquitectura DDD
    detail: "Añadir proyecto de test dedicado (recomendado: GesFer.Admin.Back.ArchitectureTests en src/) referenciando ensamblados Domain, Application, Infrastructure, Api; registrar en la solution .sln."
  - id: T14
    phase: P4
    title: Reglas de dependencia entre capas
    detail: "Implementar pruebas que fallen si se viola DDD/Clean del repo: p. ej. Domain no referencia Application/Infrastructure/Api; Application no referencia Infrastructure/Api; Infrastructure puede referenciar Application y Domain; Api puede referenciar Application e Infra según el solution actual. Usar NetArchTest.Rules (o equivalente) o verificación con System.Reflection si se prefiere sin paquete extra."
  - id: T15
    phase: P4
    title: Baseline y convención nombres
    detail: "Añadir README breve en el proyecto de arquitectura explicando qué protege; ejecutar dotnet test y dejar verde antes del commit P4."
  - id: T9
    phase: P5
    title: Tests integración
    detail: "GeoControllerTests u homólogo: URLs /api/geolocation/...; asserts con nuevos DTOs; mismo header interno que hoy."
  - id: T10
    phase: P5
    title: Tests unitarios handlers geo
    detail: "Escenarios con IsActive true/false; hijos con padre correcto; opcional fila con DeletedAt != null e IsActive true para validar IgnoreQueryFilters; eliminar dependencia de exclusión solo por DeletedAt."
  - id: T11
    phase: P5
    title: Verificación manual Swagger
    detail: "Comprobar esquemas de respuesta sin campos extra (City sin Code, Postal sin Name)."
  - id: T12
    phase: P6
    title: Artefactos de proceso
    detail: "Redactar implementation.md / execution.md tras codificar; validacion.md tras build+tests; avisar a consumidores Product (breaking routes + JSON)."
---

# Planificación — Unificación geolocalización (lectura)

## Política de commits (obligatoria en esta feature)

Cada **fase P1…P6** debe cerrarse con **exactamente un commit** (o un único commit que agrupe solo el trabajo de esa fase si ya estaba en curso). Orden sugerido:

| Fase | Momento del commit | Ejemplo de mensaje |
|------|--------------------|--------------------|
| P1 | Tras T1–T2 y proyecto compilando | `feat(geolocation): P1 DTOs geo y comando postal` |
| P2 | Tras T3–T6 y tests de handlers en verde si ya existían | `feat(geolocation): P2 handlers geo IgnoreQueryFilters + IsActive` |
| P3 | Tras T7–T8 | `feat(geolocation): P3 GeolocationController y eliminación legacy` |
| P4 | Tras T13–T15 (`dotnet test` incl. ArchitectureTests) | `feat(geolocation): P4 suite tests arquitectura DDD` |
| P5 | Tras T9–T11 | `feat(geolocation): P5 tests integración y unitarios geolocation` |
| P6 | Tras T12 y revisiones | `docs(geolocation): P6 implementation execution validacion` |

**Nota:** Los commits deben hacerse vía skill **invoke-commit** / proceso del repo (no commits directos en `master`).

## Objetivo del plan

Implementar **`GeolocationController`** bajo **`api/geolocation`**, DTOs por entidad (spec §3) y criterio de visibilidad **único**: **`IsActive == true`** más **filtrado por padre** donde aplique (C-08 cerrado). Sin filtros por **`DeletedAt`** en estos handlers; usar **`IgnoreQueryFilters()`** para no dejar que el soft-delete global de EF condicione el resultado.

## Aclaración detallada — Fase P4 (Tests DDD)

### Qué significa «DDD» aquí

En **P4**, «tests DDD» no se refieren a tests unitarios de **entidades de dominio** (agregados, value objects) ni a specs de negocio escritas en Given/When/Then sobre el modelo rico. Se refieren a **tests de arquitectura**: comprobar que el **código respeta los límites entre capas** del estilo Clean/DDD que ya usa el repo (Domain / Application / Infrastructure / Api). Es decir, **reglas de dependencia entre ensamblados** (`*.dll` / proyectos `.csproj`).

- **P4** = *¿quién puede referenciar a quién?* → fallo rápido si alguien mete un `using` o `ProjectReference` prohibido.
- **P5** = *¿el comportamiento y la API son correctos?* → integración, handlers, Swagger.

Así **P4 no sustituye P5**; las encadena: primero la “valla” de arquitectura, luego las pruebas funcionales.

### Por qué va después de P3

Tras P3 el **árbol de dependencias del producto** está completo para esta feature (Api + Application + handlers + controlador). Introducir el proyecto de arquitectura en ese momento refleja el estado real del solution y evita escribir reglas sobre ensamblados que aún no existen o están a medias.

### Entregables concretos de P4

| Tarea | Entregable |
|-------|------------|
| **T13** | Proyecto de test (recomendado: `GesFer.Admin.Back.ArchitectureTests` bajo `src/`) referenciando solo los ensamblados necesarios para inspeccionar tipos; **registrado en el `.sln`**. Alternativa aceptada: carpeta `Architecture/` dentro de `GesFer.Admin.Back.UnitTests` con las mismas reglas, **si** el commit P4 agrupa únicamente ese trabajo y `dotnet test` cubre el proyecto. |
| **T14** | Una o más clases de test (p. ej. `LayerDependencyTests`) que ejecuten reglas; ver ejemplo de reglas mínimas abajo. |
| **T15** | `README.md` breve en el proyecto (o sección en README de tests) + **`dotnet test` en verde** incluyendo este proyecto → luego **commit P4**. |

### Reglas mínimas sugeridas (ajustar al `.sln` real)

Definir en el código de test el **grafo permitido** y fallar si `Assembly.GetReferencedAssemblies()` (o **NetArchTest.Rules**, si se añade el paquete) viola:

1. **Domain** no debe referenciar Application, Infrastructure ni Api.
2. **Application** no debe referenciar Infrastructure ni Api (solo Domain y libs neutras).
3. **Infrastructure** puede referenciar Application y Domain (como hoy).
4. **Api** puede referenciar lo que el solution ya permita (típicamente Application, Infrastructure, etc.) — **no inventar límites nuevos**: copiar el grafo **actual y deseado** del repo, no un ideal irrealizable.

Si una regla choca con el estado actual del código, **acotar P4** a las reglas ya cumplidas documentando en el README qué falta para el resto (evita bloquear la feature).

### Herramientas

- **Opción A — NetArchTest.Rules:** API declarativa (`Types.InAssembly(...).Should().NotHaveDependencyOn(...)`).
- **Opción B — Solo BCL:** reflexión sobre ensamblados y `GetReferencedAssemblies()`; más código manual, sin dependencia extra.

Elegir en **T14** según preferencia del equipo; P4 no impone el paquete.

### Qué no es P4

- No sustituye tests de **GeoController** / handlers (P5).
- No valida reglas de negocio (“un país debe tener código ISO”).
- No es obligatorio en esta feature añadir **Sonar** o análisis estático adicional; solo el bloque de tests acordado.

### Contenido del commit P4

Un único commit con: proyecto/carpeta + reglas + README + referencia en `.sln` si aplica. Mensaje tipo: `feat(geolocation): P4 suite tests arquitectura DDD`.

---

## Fase P4 — Resumen operativo (tareas)

- **T13:** Crear proyecto o carpeta de tests de arquitectura e integrar en la solution / runner de tests.
- **T14:** Implementar reglas de dependencia entre capas (mínimas, alineadas al grafo real).
- **T15:** Documentar + `dotnet test` verde → commit P4.

*Texto largo de aclaración: sección **«Aclaración detallada — Fase P4»** arriba.*

## Criterio de filtrado (resumen operativo)

| Endpoint | `IgnoreQueryFilters` | Condiciones `Where` |
|----------|----------------------|----------------------|
| Lista países | Sí | `IsActive` |
| País por id | Sí | `Id` **y** `IsActive` |
| Estados | Sí | `CountryId` **y** `IsActive` |
| Ciudades | Sí | `StateId` **y** `IsActive` |
| Códigos postales | Sí | `CityId` **y** `IsActive` |

## Orden de ejecución sugerido

1. **P1** → commit — T1, T2.  
2. **P2** → commit — T3 → T6.  
3. **P3** → commit — T7, T8.  
4. **P4** → commit — T13, T14, T15 (tests DDD / arquitectura).  
5. **P5** → commit — T9, T10, T11.  
6. **P6** → commit — T12 y **finalize** + Evolution Log cuando proceda.

## Riesgos y mitigación

| Riesgo | Mitigación |
|--------|------------|
| Registros `DeletedAt != null` visibles si `IsActive` | Aceptado por decisión C-08; coordinar con datos y Product. |
| Clientes en `/api/countries` | Comunicar breaking change; despliegue coordinado. |
| InMemory vs MySQL en tests | Validar que `IgnoreQueryFilters` se comporte igual en tests unitarios. |
| P4 rompe CI por reglas demasiado estrictas | Empezar con reglas mínimas consensuadas; ampliar en otra feature. |

## Referencias

- `spec.md` §2–§4 (API, DTOs, filtros).  
- `clarify.md` C-01..C-08.  
- Código actual: `GeoHandlers.cs`, `CountriesController.cs`, `StatesController.cs`, `GeoControllerTests.cs`.
