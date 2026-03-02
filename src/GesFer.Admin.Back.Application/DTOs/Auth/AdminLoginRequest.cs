namespace GesFer.Admin.Back.Application.DTOs.Auth;

public class AdminLoginRequest
{
    public string Usuario { get; set; } = string.Empty;
    public string Contraseña { get; set; } = string.Empty;
}
