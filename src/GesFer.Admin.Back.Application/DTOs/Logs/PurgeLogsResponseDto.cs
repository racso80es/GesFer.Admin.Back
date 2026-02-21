namespace GesFer.Admin.Back.Application.DTOs.Logs;

public class PurgeLogsResponseDto
{
    public int DeletedCount { get; set; }
    public DateTime DateLimit { get; set; }
}
