using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Domain.Entities;
using GesFer.Admin.Back.Infrastructure.Data;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;

namespace GesFer.Admin.Back.Infrastructure.Services;

/// <summary>
/// Servicio para registrar logs de auditoría administrativa.
/// Usa un scope independiente para evitar conflictos con entidades rastreadas (ej. AdminUser en login).
/// </summary>
public class AuditLogService : IAuditLogService
{
    private readonly IServiceProvider _serviceProvider;
    private readonly ILogger<AuditLogService> _logger;

    public AuditLogService(IServiceProvider serviceProvider, ILogger<AuditLogService> logger)
    {
        _serviceProvider = serviceProvider;
        _logger = logger;
    }

    /// <summary>
    /// Registra un log de auditoría con el Cursor ID del administrador.
    /// Usa Sequential GUIDs para el Id del log. Scope independiente para evitar conflictos de DbContext.
    /// </summary>
    public async Task LogActionAsync(string cursorId, string username, string action, string httpMethod, string path, string? additionalData = null)
    {
        try
        {
            using var scope = _serviceProvider.CreateScope();
            var context = scope.ServiceProvider.GetRequiredService<AdminDbContext>();

            var auditLog = new AuditLog
            {
                CursorId = cursorId,
                Username = username,
                Action = action,
                HttpMethod = httpMethod,
                Path = path,
                AdditionalData = additionalData,
                ActionTimestamp = DateTime.UtcNow,
                CreatedAt = DateTime.UtcNow,
                IsActive = true
            };

            context.AuditLogs.Add(auditLog);
            await context.SaveChangesAsync();
            _logger.LogDebug("AuditLog registrado: Action={Action}, Username={Username}, Path={Path}", action, username, path);
        }
        catch (Exception ex)
        {
            // Log el error pero no fallar la operación principal (RNF3: audit no bloquea respuesta)
            _logger.LogError(ex,
                "Error al registrar log de auditoría para CursorId: {CursorId}, Action: {Action}, Path: {Path}. Tipo: {ExceptionType}, Mensaje: {Message}, StackTrace: {StackTrace}",
                cursorId, action, path, ex.GetType().FullName, ex.Message, ex.StackTrace);
        }
    }
}
