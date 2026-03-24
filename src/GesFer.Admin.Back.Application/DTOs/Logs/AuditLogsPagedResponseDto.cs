namespace GesFer.Admin.Back.Application.DTOs.Logs;

public record AuditLogsPagedResponseDto
{
    public IEnumerable<AuditLogDto> AuditLogs { get; set; } = new List<AuditLogDto>();
    public int TotalCount { get; set; }
    public int PageNumber { get; set; }
    public int PageSize { get; set; }
    public int TotalPages { get; set; }
}
