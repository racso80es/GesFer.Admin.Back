# Patrón Criteria / Specification

El patrón Criteria o Specification permite abstraer lógicas complejas de búsquedas, combinaciones de filtros, ordenaciones y paginación en objetos de dominio dedicados [18], [19]. Este patrón evita la creación descontrolada de métodos de búsqueda en los repositorios, habilitando la creación de un Lenguaje Específico de Dominio (DSL) para consultar datos [20], [21]. Posteriormente, un adaptador o convertidor se encarga de traducir este objeto Criteria a la consulta nativa de la infraestructura de destino, como SQL nativo o ElasticSearch [22], [23].
