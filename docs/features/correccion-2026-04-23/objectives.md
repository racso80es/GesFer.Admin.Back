---
feature_name: correccion-2026-04-23
---
# Objetivos: Corrección Auditoría 2026-04-23

## Objetivo
Formalizar y resolver el resultado de la auditoría del 2026-04-23. Se detectó un hallazgo crítico relacionado con la configuración permisiva de CORS en la API, la cual debe estar basada en configuración y no permitir todos los orígenes de manera incondicional.

## Alcance
- Modificar `src/GesFer.Admin.Back.Api/appsettings.json` para incluir la sección de configuración de orígenes permitidos de CORS.
- Modificar `src/GesFer.Admin.Back.Api/Program.cs` para leer los orígenes desde la configuración en lugar de usar `AllowAnyOrigin()`.
- Registrar la resolución en el log de evolución.
