# Plan de Ejecución para fix-seeder-guid-parse

1. **Documentación SddIA:** Crear `objectives.md`, `spec.md`, `spec.json`, `clarify.md` y `plan.md` en `docs/features/fix-seeder-guid-parse/`.
2. **Modificación de Código:**
   - Editar `src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs`.
   - Ubicar todas las llamadas de `Guid.Parse(variable)`.
   - Reemplazar por `if (!Guid.TryParse(variable, out var parsedId)) { ... }`.
   - Usar `_logger.LogWarning(...)` indicando que el ID es inválido e ignorar el registro con `continue`.
3. **Verificación:** Compilar el backend y correr los tests (`dotnet test src/GesFer.Admin.Back.sln`).
4. **Finalización:** Completar `implementation.md`, `execution.json` y `validacion.json`.
5. **Commit y Push:** Subir los cambios a la rama `fix/seeder-guid-parse` usando commit semántico.