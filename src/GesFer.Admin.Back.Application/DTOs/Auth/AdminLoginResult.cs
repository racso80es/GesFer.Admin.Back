namespace GesFer.Admin.Back.Application.DTOs.Auth;

/// <summary>
/// Resultado del caso de uso de login Admin. Permite al controlador mapear a códigos HTTP.
/// </summary>
public record AdminLoginResult(bool IsSuccess, AdminLoginResponse? Response, int HttpStatusCode, string? ErrorMessage)
{
    public static AdminLoginResult Success(AdminLoginResponse response) =>
        new(true, response, 200, null);

    public static AdminLoginResult ValidationError(string message) =>
        new(false, null, 400, message);

    public static AdminLoginResult AuthFailure(string message) =>
        new(false, null, 401, message);

    public static AdminLoginResult Error(string message) =>
        new(false, null, 500, message);
}
