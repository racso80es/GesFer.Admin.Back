# Outbox Pattern

El Outbox Pattern es una estrategia de resiliencia utilizada en sistemas distribuidos que asegura la publicación fiable de eventos al mundo exterior [35]. Funciona insertando el evento de dominio en una tabla auxiliar de la misma base de datos y dentro de la misma transacción en la que se modifica la entidad principal [35]. Posteriormente, un trabajador asíncrono o sistema externo lee esa tabla y consolida la publicación hacia el broker de mensajería [35].
