---
type: spec
feature_name: correccion-2026-06-02
base:
  - src/GesFer.Admin.Back.Infrastructure/Services/AdminAuthService.cs
  - src/GesFer.Admin.Back.Infrastructure/Services/AdminJsonDataSeeder.cs
  - src/GesFer.Admin.Back.Infrastructure/Services/AuditLogService.cs
scope:
  in_scope: Propagar CancellationToken y AsNoTracking en los servicios de infraestructura.
  out_scope: Modificar la lógica de negocio o la firma de interfaces existentes que no requieran CancellationToken explícito.
---

# Especificación de la Tarea

Aplicar las correcciones identificadas en la auditoría del 2026-06-02:
1. Añadir AsNoTracking() a las consultas de solo lectura en AdminAuthService.cs y AdminJsonDataSeeder.cs.
2. Propagar CancellationToken a las llamadas asíncronas de Entity Framework (FirstOrDefaultAsync, ToListAsync, SaveChangesAsync) en AdminAuthService.cs, AdminJsonDataSeeder.cs y AuditLogService.cs.
