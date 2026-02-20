# Failover Pattern

El patrón Failover actúa como un mecanismo reactivo para mitigar caídas de red o indisponibilidad en los sistemas de mensajería asíncrona [36]. Consiste en envolver el acto de publicar eventos en bloques de captura de errores, de tal modo que si el sistema central (como un RabbitMQ) no responde, el evento se guarda como medida de contingencia en un almacén de persistencia alternativo para ser reintentado en el futuro [36], [37].
