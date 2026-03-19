namespace GesFer.Admin.Back.Application.DTOs.Logs;

public record AuditLogsPagedResponseDto
{
    public List<AuditLogDto> AuditLogs { get; set; } = new();
    public int TotalCount { get; set; }
    public int PageNumber { get; set; }
    public int PageSize { get; set; }
    public int TotalPages { get; set; }
}
