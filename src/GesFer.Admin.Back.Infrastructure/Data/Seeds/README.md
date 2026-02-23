# Seeds para Dominio Admin

Esta carpeta contiene los archivos de seed data para el dominio Admin. **Admin es SSOT (Source of Truth) para Companies.**

## Archivos

### Maestro geo (zona España)
- **languages.json**: Idiomas (ej. Español).
- **countries.json**: País España (Code ES).
- **states.json**: Comunidades Autónomas de España (19: Madrid, Andalucía, Aragón, Asturias, Illes Balears, Canarias, Cantabria, Castilla y León, Castilla-La Mancha, Catalunya, Comunitat Valenciana, Extremadura, Galicia, Región de Murcia, Navarra, País Vasco, La Rioja, Ceuta, Melilla) con códigos oficiales (MD, AN, AR, etc.).
- **cities.json**: Capitales/ciudades principales por comunidad (Madrid, Sevilla, Zaragoza, Barcelona, Valencia, etc.).
- **postal-codes.json**: Códigos postales (5 dígitos) asociados a cada ciudad; cubre zona geográfica de España.

### Negocio y usuarios
- **companies.json**: Empresas (Companies). Dependen de Language (y opcionalmente City, Country, State, PostalCode).
- **admin-users.json**: Usuarios administrativos del sistema.

## Orden de ejecución (SeedAllAsync)

1. Languages → 2. Countries → 3. States → 4. Cities → 5. PostalCodes → 6. Companies → 7. AdminUsers.

## Orden de ejecución (BD compartida)

En entornos donde Admin y Product comparten la misma base de datos:

1. **Ejecutar primero los seeds de Admin** (companies y admin-users), para que la tabla `Companies` exista y esté poblada.
2. **Después**, ejecutar los seeds de Product (Languages, Users, etc.). El seed de Product ya no inserta companies; obtiene los `validCompanyIds` desde la BD (companies creadas por Admin).

Así se garantiza que las empresas existan antes de que Product cree usuarios, proveedores o clientes que referencien `CompanyId`.

## Niveles de Seed

### Master
Datos administrativos esenciales del sistema:
- Usuarios administrativos base
- Configuraciones globales
- Permisos administrativos

### Demo
Datos de demostración:
- Usuarios admin de ejemplo
- Logs de auditoría de ejemplo
- Configuraciones de demo

### Test
Datos para testing:
- Usuarios admin de prueba
- Logs de auditoría de prueba
- Configuraciones de test

## Uso

Los seeds se cargan mediante el servicio de seeding correspondiente, que debe ser configurado en el `Program.cs` o mediante un servicio de inicialización.
