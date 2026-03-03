# Reporte de Auditoría S+
**Fecha:** 2026-03-03 (UTC)

## 1. Métricas de Salud
- **Arquitectura:** 100%
- **Nomenclatura:** 100%
- **Estabilidad Async:** 100%

*(Análisis Estructural - Fase A validada: El proyecto compila correctamente sin dependencias cíclicas, no hay bloqueos de hilos ni `async void`, y se respeta el diseño DDD/Clean Architecture para este microservicio).*

## 2. Pain Points

### 🟡 Medio: Parseo inseguro de Guids en Data Seeders
- **Hallazgo:** El servicio de poblado inicial de datos (`AdminJsonDataSeeder`) utiliza el método estático `Guid.Parse` de forma intensiva al leer valores desde los archivos JSON (por ejemplo, para mapear IDs, LanguageId, CountryId, etc). Si el JSON contiene un formato incorrecto o algún valor no válido, la aplicación lanzará una `FormatException` y la inicialización del contenedor fallará por completo.
- **Ubicación:** `src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs`
  - Línea 190 (`Guid.Parse(item.Id)`)
  - Línea 230 (`Guid.Parse(item.Id)`)
  - Línea 239 (`Guid.Parse(item.LanguageId)`)
  - Línea 270 (`Guid.Parse(item.Id)`)
  - Línea 277 (`Guid.Parse(item.CountryId)`)
  - Línea 310 (`Guid.Parse(item.Id)`)
  - Línea 317 (`Guid.Parse(item.StateId)`)
  - Línea 352 (`Guid.Parse(item.Id)`)
  - Línea 359 (`Guid.Parse(item.CityId)`)
  - Línea 531 (`Guid.Parse(companyData.Id)`)
  - Línea 577 (`Guid.Parse(companyData.Id)`)

## 3. Acciones Kaizen (Hoja de Ruta para el Executor)

**Instrucciones para el Kaizen Executor:**
Se debe reemplazar cualquier invocación de `Guid.Parse(item.Prop)` por una comprobación de tipo `Guid.TryParse` en los bucles de poblado de la clase `AdminJsonDataSeeder`. Si la validación falla, se debe ignorar el registro (mediante `continue`) y opcionalmente registrar un log de advertencia (`_logger.LogWarning`), de modo que un registro corrupto no evite que el sistema levante.

**Ejemplo de implementación requerida:**
```csharp
// Antes:
// var id = Guid.Parse(item.Id);

// Ahora:
if (!Guid.TryParse(item.Id, out var id))
{
    _logger.LogWarning("AdminJsonDataSeeder: El Guid '{Id}' no es válido. Omitiendo registro.", item.Id);
    continue;
}
```

Es necesario aplicar este patrón en los métodos:
- `SeedLanguagesAsync()`
- `SeedCountriesAsync()`
- `SeedStatesAsync()`
- `SeedCitiesAsync()`
- `SeedPostalCodesAsync()`
- `SeedCompaniesAsync()`

**Definition of Done (DoD):**
1. Todos los `Guid.Parse` directos han sido eliminados de `AdminJsonDataSeeder`.
2. Todo dato inválido proveniente de JSON es ignorado y notificado vía `_logger.LogWarning`.
3. Todos los Data Seeders ejecutan con normalidad su población y la solución pasa todos sus Unit Tests e Integration Tests.
4. El proceso cumple con los estándares SddIA de documentación.
