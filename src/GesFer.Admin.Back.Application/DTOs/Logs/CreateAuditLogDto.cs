namespace GesFer.Admin.Back.Application.DTOs.Logs;

public class CreateAuditLogDto
{
    public string? CursorId { get; set; }
    public string? Username { get; set; }
    public string? Action { get; set; }
    public string? HttpMethod { get; set; }
    public string? Path { get; set; }
    public string? AdditionalData { get; set; }
    public DateTime ActionTimestamp { get; set; }
}
