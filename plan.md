1. Modificar `src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs` para agregar `.AsNoTracking()`
   - Utilizar el comando `replace_with_git_merge_diff` para añadir `.AsNoTracking()` antes de `.Where()` o después de `_context.AdminUsers` y así optimizar la consulta al login de admin.
2. Modificar `docs/features/kaizen-missing-asnotracking/spec.md` para incluir `AdminAuthService.cs`
   - Ampliar la lista de `base:` para que este servicio quede englobado en el spec de la tarea y sea consecuente con la intervención.
3. Ejecutar pruebas unitarias para asegurar que no se rompieron componentes
   - Utilizar la terminal para lanzar `dotnet test src/GesFer.Admin.Back.sln`.
4. Ejecutar validaciones pre-commit
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
5. Hacer commit y empujar los cambios
   - Terminar la tarea actualizando git y enviando las subidas.
