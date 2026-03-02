namespace GesFer.Admin.Back.Application.DTOs.Auth;

public class AdminLoginResponse
{
    public string UserId { get; set; } = string.Empty;
    public string CursorId { get; set; } = string.Empty;
    public string Username { get; set; } = string.Empty;
    public string FirstName { get; set; } = string.Empty;
    public string LastName { get; set; } = string.Empty;
    public string? Email { get; set; }
    public string Role { get; set; } = string.Empty;
    public string Token { get; set; } = string.Empty;
}
