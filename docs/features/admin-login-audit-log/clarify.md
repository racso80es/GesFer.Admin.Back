---
feature_name: admin-login-audit-log
created: 2026-03-14
purpose: Clarificación del punto 4 (Validación) y decisiones confirmadas
---

# Clarificación: Validación (punto 4)

## Contexto

El proyecto **no tiene FluentValidation** instalado actualmente:
- `Application.csproj` no incluye el paquete `FluentValidation` ni `FluentValidation.DependencyInjectionExtensions`
- `src/README.md` lista como TODO: "Agregar validación con FluentValidation"
- La validación actual del login está en el controlador: `if (string.IsNullOrWhiteSpace(request.Usuario) || string.IsNullOrWhiteSpace(request.Contraseña)) return BadRequest(...)`

## Opciones para validación

| Opción | Descripción | Alcance de esta feature |
|--------|-------------|--------------------------|
| **4a. Incluir FluentValidation** | Añadir paquete FluentValidation, crear `AdminLoginCommandValidator`, registrar en DI, usar `ValidationBehavior` de MediatR. | Amplía el alcance: nueva dependencia y patrón en el proyecto. |
| **4b. Validación en el Handler** | El `AdminLoginHandler` valida Usuario/Contraseña no vacíos y devuelve `Result.Failure` o lanza `ValidationException` que el controlador traduce a BadRequest. | Mínimo: mantiene la lógica actual, solo movida al handler. |
| **4c. Diferir FluentValidation** | En esta feature usar validación en handler (4b). Crear feature separada "fluent-validation-admin" para introducir FluentValidation en el proyecto. | Mantiene esta feature enfocada; FluentValidation queda para después. |

## Recomendación

**Opción 4b** para esta feature:
- No introduce dependencias nuevas.
- La validación pasa del controlador al handler (coherente con CQRS).
- El handler puede devolver un tipo `Result<AdminLoginResponse>` o `OneOf` para distinguir éxito/fallo de validación/credenciales inválidas.
- FluentValidation puede añadirse en una feature posterior si se desea estandarizar.

## Decisión cerrada

**Opción 4b** — Validación en el Handler.

- El `AdminLoginHandler` valida Usuario/Contraseña no vacíos.
- Devuelve `AdminLoginResult.ValidationError` para BadRequest.
- Sin nuevas dependencias (FluentValidation diferido a feature futura).
