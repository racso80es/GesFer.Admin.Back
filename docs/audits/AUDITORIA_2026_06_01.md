1. Métricas de Salud (0-100%)
Arquitectura: 100% | Nomenclatura: 100% | Estabilidad Async: 85%

2. Pain Points (🔴 Críticos / 🟡 Medios)
Hallazgo: [🔴 Crítico] Falta de propagación del CancellationToken en los métodos asíncronos y ausencia de \`.AsNoTracking()\` en operaciones de solo lectura.
Ubicación: src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs línea 37
Ubicación: src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs (múltiples líneas: 204, 208, 371, 419, 472, 525, 580, 645, 769)

3. Acciones Kaizen (Hoja de Ruta para el Executor)
Kaizen 1: Propagación de CancellationToken y optimización de memoria (AsNoTracking)
Instrucciones:
1. En \`src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs\`, modificar \`AuthenticateAsync\` para recibir un \`CancellationToken cancellationToken = default\` y usar \`AsNoTracking()\` al consultar el usuario, así como pasar el \`cancellationToken\` a \`FirstOrDefaultAsync\`.
2. Opcional (Si el scope lo permite): En \`src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs\`, aunque es un seeder, es buena práctica usar \`AsNoTracking()\` cuando solo se lee (como en la verificación de existencia).

DoD (Definition of Done):
- Todos los métodos de solo lectura en \`AdminAuthService\` utilizan \`AsNoTracking()\`.
- El método \`AuthenticateAsync\` acepta un \`CancellationToken\` (opcional) y lo propaga a \`FirstOrDefaultAsync\`.
- El proyecto compila sin errores y los tests pasan.
