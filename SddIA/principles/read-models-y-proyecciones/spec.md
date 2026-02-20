# Read Models y Proyecciones

**principle_id:** `read-models-y-proyecciones`

## Resumen

En sistemas maduros, la separación explícita de cómo escribimos la información frente a cómo la consultamos ayuda a esquivar las costosas y frágiles operaciones con múltiples uniones de tablas (joins) en la base de datos central. Mediante la ingesta de eventos de dominio, un sistema alternativo puede mantener vivas representaciones o proyecciones preparadas para ser leídas de un modo puramente optimizado según los criterios específicos del consumidor (Read Models).

## Objetivo

Optimizar lecturas con modelos dedicados (vistas, proyecciones) actualizados por eventos, evitando que las consultas complejas y los requisitos de UI distorsionen el modelo de escritura o la transaccionalidad del dominio.

## Aplicación para Arquitecto

- Diseñar Read Models por necesidad de consulta (pantallas, reportes, APIs de solo lectura) y definir proyecciones que se alimenten de eventos de dominio o de un store de eventos.
- Documentar la consistencia eventual y las estrategias de reconstrucción de proyecciones; evitar que la capa de escritura dependa de los read models.

## Aplicación para Tekton

- Implementar proyectores que escuchen eventos de dominio y actualicen tablas o documentos de lectura; mantener la lógica de proyección idempotente y tolerante a reprocesado.
- No realizar consultas de lectura complejas (muchos joins) sobre el modelo transaccional principal cuando exista un read model apropiado; exponer APIs de consulta que lean de las proyecciones.

## Referencias

[33], [34], [35] — CQRS, proyecciones, read models.

---
*Definición en paths.principlesPath/read-models-y-proyecciones/ (contrato paths.principlesPath/principles-contract.md).*
