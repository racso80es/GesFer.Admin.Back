---
feature_name: user-crud
created: "2026-04-29"
process: feature
---

## Objetivo

Implementar la entidad **User** (modelo + API CRUD) con comportamiento multi-tenant por `CompanyId`, siguiendo la especificación externa en `c:\Proyectos\GesFer.Product.Back\docs\DocumentacionUsuarios.md`.

## Alcance

- **In scope**
  - Modelo `Users` (soft delete, índices, constraints).
  - CRUD (Commands/Handlers) y endpoints `/api/User` (GET all/by id, POST, PUT, DELETE).
  - Hash de contraseña con BCrypt (work factor 11) y compatibilidad con seed determinista para `admin123`.
  - Validación de unicidad de `Username` por `CompanyId`.
  - Validación de integridad referencial de geografía opcional (PostalCode/City/State/Country/Language) con `OnDelete: Restrict`.
- **Out of scope**
  - UI/Frontend.
  - Rediseño del acoplamiento con el microservicio Admin (solo consumo del `IAdminApiClient.GetCompanyAsync` donde aplique).

## Ley aplicada

- No trabajar en `master/main`; todo en `feat/user-crud` (skill `iniciar-rama`).
- SSOT documental: esta carpeta (`docs/features/user-crud/`) es canónica para la tarea.
