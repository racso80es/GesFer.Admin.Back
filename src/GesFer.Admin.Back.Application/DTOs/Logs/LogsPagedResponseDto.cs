namespace GesFer.Admin.Back.Application.DTOs.Logs;

public record LogsPagedResponseDto
{
    public IEnumerable<LogDto> Logs { get; set; } = [];
    public int TotalCount { get; set; }
    public int PageNumber { get; set; }
    public int PageSize { get; set; }
    public int TotalPages { get; set; }
}
