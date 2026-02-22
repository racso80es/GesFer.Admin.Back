# JSON Web Tokens (JWT) y su Integridad

## Descripción
Los JWT permiten la comunicación de información entre cliente y servidor sin necesidad de guardar el estado de la sesión en base de datos. El token contiene un *payload* con datos (como el ID del usuario).

## Riesgo de Integridad
El principal riesgo es que un usuario intercepte el token y modifique su contenido (ej. cambiando su ID para suplantar a otro).

## Mecanismo de Seguridad
El servidor firma el JWT en su creación utilizando un secreto o clave privada; si el *payload* es manipulado, el servidor detectará que la firma criptográfica ya no es válida y rechazará la petición [5, 6].
