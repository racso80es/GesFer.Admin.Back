namespace GesFer.Admin.Back.Application.DTOs.Logs;

public record PurgeLogsResponseDto
{
    public int DeletedCount { get; set; }
    public DateTime DateLimit { get; set; }
}
