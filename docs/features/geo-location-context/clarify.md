# Clarificaciones y Decisiones

## Q1: ¿Alcance del Runner en Rust?
**A:** El objetivo inicial es establecer la estructura y el proyecto en Rust (`scripts/tools-rs`). No se migrarán todas las skills existentes (PowerShell) a Rust en este momento, pero se dejará la "norma" y el "ejecutable base" listos. El runner debe ser capaz de compilar y ejecutar un "Hello World" o similar para demostrar la capacidad.

## Q2: ¿Fuente de Datos Geográficos?
**A:** Se generarán archivos JSON seed con una muestra representativa (ej. España, sus comunidades/provincias y capitales) para validar el funcionamiento. No se requiere migración masiva de datos legacy en esta fase, sino la estructura para soportarlo.

## Q3: ¿CRUD o Read-Only?
**A:** Solo lectura (Consumption). La gestión de estas entidades maestras se realizará vía Seeds o procesos de fondo por ahora. La API es para consumo por otros sistemas/frontends.

## Q4: ¿Estructura de Endpoints?
**A:** Se opta por controladores separados o agrupados lógicamente. Dado el volumen potencial, separar en `CountriesController`, `StatesController`, `CitiesController` podría ser excesivo si solo hay lectura básica. Se usará un `GeoLocationController` con rutas jerárquicas o bien `CountriesController` que sirva de punto de entrada. *Decisión:* Usar `GeoLocationController` o `LocationsController` para agrupar, o endpoints específicos si crece. *Refinamiento:* Para seguir REST estricto, usaremos `CountriesController`, `StatesController`, `CitiesController` es lo más limpio, o bien `CountriesController` con sub-recursos.
*   `GET /api/countries`
*   `GET /api/countries/{id}`
*   `GET /api/countries/{id}/states` (Relación directa)
*   `GET /api/states/{id}/cities`
*   Esta estructura jerárquica es preferible.

## Q5: ¿Autenticación?
**A:** `[AuthorizeSystemOrAdmin]` (Policy basada en Role 'Admin' o Shared Secret para sistemas).
