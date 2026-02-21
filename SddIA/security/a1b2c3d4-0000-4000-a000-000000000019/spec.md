# Asegurando APIs con Helmet y CORS

## Descripción
Al desarrollar servidores web HTTP base (como en Node.js y Express), la aplicación por defecto viene sin cabeceras de protección básicas, dejándola expuesta a inyecciones tipo XSS o manipulaciones de iframes.

## Mitigación
Implementar *middlewares* de seguridad probados, como **Helmet** y una correcta configuración de políticas **CORS**, permite fortificar la API inyectando cabeceras defensivas y limitando estrictamente los orígenes desde los cuales pueden llegar peticiones al backend [28, 29].
