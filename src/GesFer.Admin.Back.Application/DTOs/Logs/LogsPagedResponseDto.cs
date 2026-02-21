namespace GesFer.Admin.Back.Application.DTOs.Logs;

public class LogsPagedResponseDto
{
    public List<LogDto> Logs { get; set; } = new();
    public int TotalCount { get; set; }
    public int PageNumber { get; set; }
    public int PageSize { get; set; }
    public int TotalPages { get; set; }
}
