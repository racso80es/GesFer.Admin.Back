# Consultas Parametrizadas (Prepared Statements)

## Descripción
Para protegerse de inyecciones SQL, no basta con intentar limpiar comillas manualmente, ya que es un proceso propenso a errores. El estándar en la industria es utilizar consultas parametrizadas o *Prepared Statements*.

## Funcionamiento
En lugar de introducir el valor directamente en el string de la consulta SQL, se colocan marcadores (como `?` o `:name`) y se pasa la lista de parámetros al driver de la base de datos. La librería subyacente (como PDO en PHP o Dapper en .NET) se encarga de escapar y parsear de forma segura la entrada, neutralizando cualquier intento de inyección [3, 4].
