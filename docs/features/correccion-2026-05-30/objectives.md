---
type: objectives
status: pending
audit_ref: docs/audits/AUDITORIA_2026_05_30.md
---

# Objetivos de Corrección: Auditoría 2026-05-30

## Hallazgos Priorizados
1. Faltan CancellationToken en llamadas a la base de datos Entity Framework.
   - Crítico para la estabilidad asíncrona.
   - Ubicaciones: `IAdminAuthService`, `AdminAuthService`, `AuditLogService`.
2. Falta .AsNoTracking() en consultas de solo lectura.
   - Medio (optimización y prevención de fugas de memoria).
   - Ubicaciones: `AdminAuthService.cs:36`.

## Criterios de Cierre
- Todos los métodos asíncronos en las clases identificadas deben propagar el `CancellationToken`.
- La consulta en `AdminAuthService` debe usar `AsNoTracking()`.
- Los tests relacionados deben ser actualizados.
- El código debe compilar correctamente.
- Las pruebas deben pasar.
