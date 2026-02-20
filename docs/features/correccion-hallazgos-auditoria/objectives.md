# Objetivo: Corrección de hallazgos de auditoría

**Proceso:** feature  
**Ruta (Cúmulo):** paths.featurePath/correccion-hallazgos-auditoria/  
**Rama:** feat/correccion-hallazgos-auditoria  
**Ley aplicada:** Ley GIT (trabajo en feat/), Ley COMPILACIÓN (código que compila), Soberanía documental (docs/audits/).

## Objetivo

Corregir todo lo indicado en las auditorías realizadas, en particular el informe `docs/audits/validacion-main-20260217.json`, de modo que se eliminen los hallazgos bloqueantes y las advertencias corregibles.

## Hallazgos de la auditoría (validacion-main-20260217.json)

| Check         | Resultado | Mensaje / causa |
| :---          | :---      | :---             |
| **build**     | fail      | La solución depende de proyectos externos (Shared/Back) fuera del repositorio. `dotnet build` falla por GesFer.Shared.Back.Domain no encontrado. |
| **test**      | skip      | Tests no ejecutados: build falló. |
| **documentation** | warn | Modo sin documentación: no existe carpeta de tarea (Cúmulo). Cambios analizados por diff frente a rama base. |
| **git_branch**| warn      | Rama actual es main. Según Ley GIT el trabajo debería estar en feat/ o fix/. |

**Global:** fail — **Blocking:** true.

## Plan de corrección

1. **Build (bloqueante):** Corregir referencias a Shared en los .csproj y en la solución. El repositorio contiene `src/Shared/Back/` con GesFer.Shared.Back.Domain e Infrastructure; Domain e Infrastructure referenciaban `../../../Shared/Back/` (fuera del repo) y la solución `GesFer.Product.sln` apuntaba a `..\..\Shared\Back\`. **Hecho:** referencias actualizadas a rutas dentro del repo (`../Shared/Back/` en Domain e Infrastructure, `Shared\Back\` en la .sln). Build y tests pasan.
2. **Test:** Se desbloqueará al pasar el build; no requiere acción adicional.
3. **Documentation:** Esta feature tiene ya carpeta de tarea (Cúmulo: paths.featurePath/correccion-hallazgos-auditoria/); las validaciones futuras con documentación de tarea cumplirán el check.
4. **Git branch:** Todo el trabajo de esta tarea se realiza en rama feat/correccion-hallazgos-auditoria (no en main).

## Alcance

- Alcance mínimo: solo los cambios necesarios para que build pase y se respete la rama de trabajo.
- Sin refactor ni ampliación de funcionalidad en esta rama.

## Referencias

- Informe de auditoría: [docs/audits/validacion-main-20260217.json](../../audits/validacion-main-20260217.json)
- Cúmulo paths: SddIA/agents/cumulo.json
- Proceso feature: SddIA/process/feature.md
