# Especificación Técnica para fix-seeder-guid-parse

## Contexto
El servicio de inicialización de la base de datos `AdminJsonDataSeeder` utiliza llamadas directas a `Guid.Parse` para convertir strings provenientes de archivos JSON (por ejemplo, `languages.json`, `countries.json`, etc.) en objetos `Guid`. Si algún string está malformado, la aplicación falla al iniciarse y no completa el proceso de arranque.

## Requisitos Técnicos
1. Identificar todos los usos de `Guid.Parse(string)` en `AdminJsonDataSeeder.cs`.
2. Reemplazarlos por `Guid.TryParse(string, out var parsedValue)`.
3. Si el método devuelve `false`, loggear un warning indicando la causa, por ejemplo: `"AdminJsonDataSeeder: El Guid '{Id}' no es válido. Omitiendo registro."`
4. Usar la instrucción `continue;` para omitir el procesamiento del registro malformado y continuar con el siguiente.

## Métodos Afectados
- `SeedLanguagesAsync()`
- `SeedCountriesAsync()`
- `SeedStatesAsync()`
- `SeedCitiesAsync()`
- `SeedPostalCodesAsync()`
- `SeedCompaniesAsync()`

## Criterios de Aceptación (DoD)
1. Todos los `Guid.Parse` directos han sido eliminados de `AdminJsonDataSeeder`.
2. Todo dato inválido proveniente de JSON es ignorado y notificado vía `_logger.LogWarning`.
3. Todos los Data Seeders ejecutan con normalidad su población y la solución pasa todos sus Unit Tests e Integration Tests.
4. El proceso cumple con los estándares SddIA de documentación.