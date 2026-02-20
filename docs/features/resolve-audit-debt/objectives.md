# Objetivo: Resolver Deuda Técnica de Auditoría (2026-02-20)

**Proceso:** feature
**Persist:** docs/features/resolve-audit-debt/
**Rama:** feat/resolve-audit-debt
**Ley aplicada:** Ley GIT (trabajo en rama feat), Ley COMPILACIÓN (código debe compilar), Soberanía documental.

## Objetivo
Resolver los hallazgos críticos de auditoría reportados en `AUDITORIA_2026_02_19.md` y `AUDITORIA_2026_02_20.md`, priorizando la corrección de la compilación y las violaciones arquitectónicas graves.

## Hallazgos a Resolver

1.  **Compilación Rota (Crítico):** Falta de DTOs en `GesFer.Admin.Back.Application.DTOs.Logs`.
2.  **Violación de Capas (Crítico):** `GesFer.Admin.Back.Application` referencia directamente a `GesFer.Admin.Back.Infrastructure`.
3.  **Acoplamiento Api -> Infraestructura (Crítico):** `LogController` depende de `AdminDbContext` (Infrastructure).
4.  **Interfaces Mal Ubicadas (Crítico):** Interfaces de servicio (`IAdminAuthService`, etc.) ubicadas en `Infrastructure` en lugar de `Application`.
5.  **Limpieza Legacy (Medio):** Eliminar código y carpetas legacy (`src/tests`, `GesFer.Product.IntegrationTests`).

## Alcance
- Crear DTOs faltantes en `Application`.
- Mover interfaces de `Infrastructure` a `Application`.
- Implementar patrón Mediator o `IApplicationDbContext` para desacoplar `LogController`.
- Eliminar referencia `Application -> Infrastructure`.
- Eliminar carpeta `src/tests`.

## Referencias
- `docs/audits/AUDITORIA_2026_02_20.md`
- `docs/audits/AUDITORIA_2026_02_19.md`
