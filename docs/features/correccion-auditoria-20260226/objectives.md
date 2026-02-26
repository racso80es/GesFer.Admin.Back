# Objetivos: Corrección Auditoría 2026-02-26

**Fecha:** 2026-02-26
**Responsable:** Jules (Executor)
**Estado:** En Progreso

Este documento define los objetivos para la corrección de los hallazgos reportados en la auditoría `AUDITORIA_2026_02_26.md`.

## 1. Implementación de Value Objects (KAIZEN-1) - Alta Prioridad

**Problema:**
La auditoría identificó la ausencia de Value Objects nucleares `ProcessId` y `ActionStatus` en el Dominio, lo cual viola el patrón de Value Objects establecido y expone el dominio al uso de primitivos.

**Objetivo:**
Implementar `ProcessId` y `ActionStatus` como `readonly record struct` en `src/GesFer.Admin.Back.Domain/ValueObjects/`. Deben incluir `TypeConverter` y `JsonConverter` para asegurar su correcta serialización y uso en APIs y persistencia.

**DoD (Definition of Done):**
- [ ] `ProcessId.cs` creado con `TypeConverter` y `JsonConverter`.
- [ ] `ActionStatus.cs` creado con valores permitidos (Pending, InProgress, Completed, Failed).
- [ ] Compilación exitosa.

## 2. Reparación de Tests de Integración (KAIZEN-2) - Media Prioridad

**Problema:**
El test de integración `GeoControllerTests.GetCitiesByState_ShouldReturnList` falla porque no encuentra la ciudad "Madrid". La causa raíz es que `AdminJsonDataSeeder` no localiza correctamente los archivos JSON de semillas debido a una lógica de resolución de rutas incorrecta para la estructura actual del proyecto (busca `GesFer.sln` en lugar de `GesFer.Admin.Back.sln` o la carpeta `src` correcta).

**Objetivo:**
Corregir la lógica de `AdminJsonDataSeeder` para que localice robustamente la carpeta `src/GesFer.Admin.Back.Infrastructure/Data/Seeds/` independientemente del entorno de ejecución (Development/Test).

**DoD (Definition of Done):**
- [ ] `AdminJsonDataSeeder.cs` modificado para buscar `GesFer.Admin.Back.sln` o la estructura `src/GesFer.Admin.Back.Infrastructure`.
- [ ] `dotnet test src/GesFer.Admin.Back.IntegrationTests/` ejecuta exitosamente todos los tests, incluyendo `GeoControllerTests`.

## 3. Plan de Migración a Rust (KAIZEN-3) - Baja Prioridad

**Problema:**
La auditoría señaló la violación de la política de tooling al tener scripts en PowerShell (`.ps1`) y C#. Todo el tooling interno debe ser migrado a Rust.

**Objetivo:**
Crear un documento de análisis y planificación para la migración de estos scripts a herramientas Rust.

**DoD (Definition of Done):**
- [ ] Documento `docs/analysis/MIGRACION_RUST_SCRIPTS.md` creado con el inventario de scripts y estrategia de migración.
