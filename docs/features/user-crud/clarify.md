---
feature_name: user-crud
created: "2026-04-29"
purpose: "Cerrar decisiones y riesgos antes de plan/implementation."
decisions:
  - id: D1
    title: "Multi-tenant por CompanyId desde token"
    status: accepted
    rationale: "El documento base indica que el scope de lectura/escritura se limita por CompanyId del invocador; evita acceso cross-tenant."
  - id: D2
    title: "Soft delete como eliminación oficial"
    status: accepted
    rationale: "DeleteUserCommandHandler debe setear DeletedAt y desactivar IsActive; no se borra físicamente."
  - id: D3
    title: "BCrypt WorkFactor 11 + seed determinista admin123"
    status: accepted
    rationale: "Compatibilidad con test-data/JsonDataSeeder; mantener hash fijo para admin123 en seeds."
  - id: D4
    title: "Validación de CompanyId vía Admin API (sincrónica)"
    status: accepted
    rationale: "Acoplamiento explícito indicado; se mantiene (no se rediseña en esta feature)."
  - id: D5
    title: "Integridad referencial geográfica local con OnDelete Restrict"
    status: accepted
    rationale: "Evita cascadas no deseadas al depender de catálogos compartidos."
---

## Clarificaciones cerradas

### Password / seguridad

- `PasswordHash` no se expone en `UserDto`.
- En `UpdateUser`, si `Password` es `null`/no provista, **no** se rehashea ni se toca el hash.

### Unicidad

- Constraint/índice único: `(CompanyId, Username)`.
- En update, la verificación de unicidad excluye el usuario actual.

### Borrado

- Delete aplica soft delete: `DeletedAt = UtcNow` y `IsActive = false`.

## Riesgos / notas

- **Acoplamiento**: `IAdminApiClient.GetCompanyAsync` es síncrono en el flujo de creación/actualización; puede impactar latencia/fiabilidad.
- **Catálogos locales**: PostalCode/City/State/Country/Language residen en Product; la dependencia permanece.

## Pendientes para planificación (plan.md)

- Determinar ubicaciones exactas en el código para:
  - Entidad EF + configuración (constraints/índices)
  - DTOs
  - Handlers y endpoints
  - JsonDataSeeder mapping para `Users`
