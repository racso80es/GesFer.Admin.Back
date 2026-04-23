---
feature_name: correccion-2026-04-23
---
# Especificación: Corrección Auditoría 2026-04-23

## Detalles Técnicos
La auditoría del 2026-04-23 determinó que el sistema requiere restringir los orígenes de CORS.

## Acciones Realizadas
- Añadir a `appsettings.json` la propiedad `Cors:AllowedOrigins` con un array vacío como valor inicial.
- En `Program.cs`, recuperar los orígenes permitidos mediante `builder.Configuration.GetSection("Cors:AllowedOrigins").Get<string[]>() ?? Array.Empty<string>()`.
- Modificar la política `AllowAll` para que utilice `WithOrigins(allowedOrigins)` en lugar de `AllowAnyOrigin()`.
