namespace GesFer.Admin.Back.Application.Common.Interfaces;

public interface IAuditLogService
{
    Task LogActionAsync(string cursorId, string username, string action, string httpMethod, string path, string? additionalData = null);
}
