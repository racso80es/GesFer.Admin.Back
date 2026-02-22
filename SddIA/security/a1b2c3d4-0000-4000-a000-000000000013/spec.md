# Fuga de Secretos en Repositorios y ficheros .env

## Descripción
Subir credenciales, tokens de AWS o contraseñas a un repositorio público (o incluso privado, si se filtra) es un error catastrófico y sumamente habitual. A menudo, esto ocurre al ignorar el uso de gitignore o al dejar claves en ficheros de configuración locales como los `.env` o `config.yml` [18].

## Riesgo Automatizado
Los ataques asistidos por Inteligencia Artificial y crawlers rastrean GitHub constantemente; pueden agrupar tus ficheros `.env` expuestos y automatizar robos de bases de datos o inyecciones masivas en minutos [19].
