# Clarificación: create-tool-postman-mcp-validation

**Tarea:** create-tool-postman-mcp-validation  
**Ruta (Cúmulo):** paths.featurePath/create-tool-postman-mcp-validation/  
**Acción:** clarify (paths.actionsPath/clarify/)  
**Fase:** Especificación + Clarificación

---

## Alcance cerrado

- **Tipo de elemento:** Tool en SddIA (paths.toolsDefinitionPath, paths.toolCapsules). Proceso create-tool.
- **Motor de validación:** Newman como ejecutor de la colección Postman; la cápsula invoca Newman (o `npx newman`) y transforma su salida al formato tools-contract.
- **Colección por defecto:** docs/postman/GesFer.Admin.Back.API.postman_collection.json; ruta **relativa a la raíz del repositorio** (working directory habitual al invocar la tool).
- **Salida:** JSON según tools-contract; data.run_summary con al menos executed, passed, failed (números); opcionalmente data.failed_requests (array de { name, error }) para trazabilidad. duration_ms en data.
- **Karma2Token:** La tool se ejecuta bajo contexto Karma2Token según tools-contract; la cápsula documenta el requisito y, en implementación, puede validar token o delegar en el launcher/orquestador que invoque la tool.
- **API no levantada:** Si la API no está disponible (conexión rechazada, timeout), la tool debe devolver success: false, exitCode != 0 y al menos una entrada feedback con level "error" describiendo el fallo; opcionalmente data con detalle (ej. connection_refused).

## Gaps resueltos

| Gap / Ambigüedad | Resolución |
|------------------|------------|
| Ruta por defecto de la colección: ¿relativa a qué? | Relativa a la **raíz del repositorio** (directorio de trabajo típico al ejecutar la tool). Valor por defecto: docs/postman/GesFer.Admin.Back.API.postman_collection.json. |
| Newman: ¿global, npx o en PATH? | Prerequisito: Newman disponible (npm install -g newman o npx). La cápsula puede invocar `newman` o `npx newman run ...`; documentar en postman-mcp-validation.md de la cápsula. |
| InternalSecret por defecto | Por defecto desde postman-mcp-validation-config.json (clave internalSecret) o variable de entorno (ej. POSTMAN_INTERNAL_SECRET). Si no hay valor y la colección lo requiere, la ejecución puede fallar en esos requests; registrar en feedback. |
| Estructura exacta de data.run_summary | run_summary: { executed: number, passed: number, failed: number }; opcional: failed_requests: [{ name: string, error?: string }]. Assertions si Newman lo expone en su reporter JSON; si no, omitir o derivar de passed/failed. |
| Working directory al ejecutar | La tool debe ejecutarse con el directorio de trabajo en la raíz del repo (o la cápsula resuelve rutas relativas respecto a la raíz del repo), para que CollectionPath y EnvironmentPath por defecto funcionen. Documentar en la cápsula. |

## Pendiente / Abierto

- **Implementación concreta del script .ps1:** Orden exacta de argumentos Newman (--globals, --env, -r json), parsing del JSON de Newman y mapeo a feedback/data queda para fase de implementación (documento implementation.md / execution).
- **Variable adminToken:** La colección usa adminToken tras Login Admin; en ejecución no interactiva la cápsula puede dejar que el script de la colección lo establezca si el login va en la misma run, o documentar que para carpetas que requieran JWT debe ejecutarse el flujo completo (Health → Auth → resto). No bloquear v1; se valida con el caso de uso.

## Referencias

- SPEC: SddIA/tools/postman-mcp-validation/spec.md, spec.json; docs/features/.../spec.md, spec.json
- Contrato: SddIA/tools/tools-contract.json
- Proceso: paths.processPath/create-tool/
- Acción clarify: paths.actionsPath/clarify/
