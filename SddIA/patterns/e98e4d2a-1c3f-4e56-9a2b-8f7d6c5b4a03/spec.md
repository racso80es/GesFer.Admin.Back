# Patrón Repository

El patrón Repository centraliza el almacenamiento de datos empleando un lenguaje propio del dominio y de negocio [8], [9]. Actúa conceptualmente como una colección de objetos en memoria, aislando a la aplicación de los detalles técnicos concretos de la persistencia subyacente, ya sea SQL, Redis o memoria [10], [11]. Favorece enormemente la testabilidad del sistema porque permite invertir el control e inyectar implementaciones en memoria durante la ejecución de pruebas unitarias [12], [13].
