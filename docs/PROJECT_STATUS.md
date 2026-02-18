# Estado del Proyecto GesFer.Admin.Back

**Fecha:** 14/11/2024
**Estado:** Análisis Inicial & Limpieza
**Propósito:** Este documento refleja el estado actual del repositorio tras su desacoplamiento de un monorepo anterior, identificando discrepancias y acciones requeridas para estabilizar el proyecto como `GesFer.Admin.Back` (API Administrativa Aislada).

---

## 1. Análisis de Infraestructura (Docker)

**Auditor:** Infrastructure Architect

### Situación Actual
El archivo `docker-compose.yml` (y sus variantes) es una copia heredada de una arquitectura de microservicios acoplada que ya no refleja la realidad de este repositorio.

*   **Servicios Inexistentes Referenciados:**
    *   `gesfer-product-api` (Contexto: `src/Product/Back/Dockerfile` - NO EXISTE)
    *   `gesfer-product-front` (Contexto: `src/Product/Front/Dockerfile` - NO EXISTE)
    *   `gesfer-admin-front` (Contexto: `src/Admin/Front/Dockerfile` - NO EXISTE)
*   **Servicios Válidos (pero mal configurados):**
    *   `gesfer-admin-api`: Apunta a `src/Admin/Back/Dockerfile` pero el código real está en `src/Api` y `src/Dockerfile`.
    *   `gesfer-db`: MySQL 8.0 (Correcto, necesario).
    *   `cache`: Memcached (Correcto, necesario).
    *   `adminer`: (Utilidad, opcional pero útil).

### Acción Requerida
Limpieza profunda de la orquestación de contenedores.

1.  **Eliminar** servicios `product-api`, `product-front`, `admin-front`.
2.  **Corregir** `gesfer-admin-api` para construir desde el contexto raíz (`.`) usando el `src/Dockerfile` correcto (o mover el Dockerfile a una ubicación estándar).
3.  **Actualizar** `src/Dockerfile` si referencia rutas antiguas (`Product/Back/...`).
4.  **Reducir** `docker-compose.yml` a lo esencial: API, DB, Cache.

---

## 2. Análisis de Código y Nomenclatura

**Auditor:** Architect / Tekton

### Situación Actual
La estructura de carpetas en `src/` es funcional pero inconsistente en nomenclatura y casing, y contiene vestigios de la estructura anterior.

*   **Espacio de Nombres Objetivo:** `GesFer.Admin.Back`
*   **Estructura Actual:**
    *   `src/Api` (PascalCase) -> Contiene `GesFer.Admin.Api.csproj`.
    *   `src/application` (camelCase) -> Contiene `GesFer.Application.csproj`.
    *   `src/domain` (camelCase) -> Contiene `GesFer.Domain.csproj`.
    *   `src/Infrastructure` (PascalCase) -> Contiene `GesFer.Infrastructure.csproj`.

### Discrepancias
1.  **Inconsistencia de Casing:** Mezcla de PascalCase y camelCase en carpetas de primer nivel.
2.  **Nombres de Proyectos:**
    *   `GesFer.Application` debería ser idealmente `GesFer.Admin.Application` o `GesFer.Back.Application` para alinearse con `GesFer.Admin.Back`.
    *   `GesFer.Domain` -> `GesFer.Admin.Domain` / `GesFer.Back.Domain`.
    *   `GesFer.Infrastructure` -> `GesFer.Admin.Infrastructure` / `GesFer.Back.Infrastructure`.

### Acción Requerida
1.  **Refactorización de Estructura:** Unificar nombres de carpetas a PascalCase (`Application`, `Domain`).
2.  **Limpieza de Namespaces:** Verificar que los namespaces internos del código coincidan con la estructura física y lógica deseada.

---

## 3. Análisis de Tests

**Auditor:** QA Judge

### Situación Actual
Existen dos jerarquías de tests, una válida y una obsoleta.

*   **Tests Válidos:**
    *   `src/IntegrationTests/`: Contiene `GesFer.IntegrationTests.csproj`. Apunta correctamente a `GesFer.Admin.Api`. Parece ser la suite de pruebas activa.
*   **Tests Obsoletos/Rotos:**
    *   `src/tests/GesFer.Product.UnitTests/`: Referencia proyectos inexistentes (`GesFer.Api`). Es código muerto heredado.
    *   `src/tests/GesFer.Product.IntegrationTests/`: Carpeta vacía o con estructura rota.

### Acción Requerida
1.  **Eliminar** completamente el directorio `src/tests/`.
2.  **Consolidar** `src/IntegrationTests/` como la suite principal (quizás moviéndola a `src/Tests/Integration` para seguir convenciones estándar).
3.  **Verificar** ejecución de `dotnet test` sobre la suite válida.

---

## 4. Análisis de Documentación y Agentes

**Auditor:** Knowledge Architect

### Situación Actual
El sistema multi-agente (`AGENTS.md`) apuntaba a una ruta inexistente (`openspecs/`).

*   **Corrección Realizada:** Se ha actualizado `AGENTS.md` para apuntar a `SddIA/`.
*   **Faltantes:**
    *   El archivo de especificación `SddIA/agents/knowledge-architect.json` **no existe**, aunque es referenciado en `AGENTS.md`. Esto impide que el agente de documentación opere bajo sus propias reglas estrictas.

### Acción Requerida
1.  **Crear/Restaurar** `SddIA/agents/knowledge-architect.json`.
2.  **Mantener** `docs/` como fuente de verdad.
