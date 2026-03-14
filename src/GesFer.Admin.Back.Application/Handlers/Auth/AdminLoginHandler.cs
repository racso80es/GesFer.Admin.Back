using System.Text.Json;
using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.Auth;
using GesFer.Admin.Back.Application.Commands.Auth;
using MediatR;
using Microsoft.Extensions.Logging;

namespace GesFer.Admin.Back.Application.Handlers.Auth;

/// <summary>
/// Handler del caso de uso de login Admin. Valida, autentica, genera token y registra en AuditLogs.
/// </summary>
public class AdminLoginHandler : IRequestHandler<AdminLoginCommand, AdminLoginResult>
{
    private const string LoginPath = "/api/admin/auth/login";
    private const string HttpMethod = "POST";

    private readonly IAdminAuthService _authService;
    private readonly IAdminJwtService _jwtService;
    private readonly IAuditLogService _auditService;
    private readonly ILogger<AdminLoginHandler> _logger;

    public AdminLoginHandler(
        IAdminAuthService authService,
        IAdminJwtService jwtService,
        IAuditLogService auditService,
        ILogger<AdminLoginHandler> logger)
    {
        _authService = authService;
        _jwtService = jwtService;
        _auditService = auditService;
        _logger = logger;
    }

    public async Task<AdminLoginResult> Handle(AdminLoginCommand request, CancellationToken cancellationToken)
    {
        try
        {
            if (string.IsNullOrWhiteSpace(request.Usuario) || string.IsNullOrWhiteSpace(request.Contraseña))
            {
                return AdminLoginResult.ValidationError("Usuario y contraseña son requeridos");
            }

            var adminUser = await _authService.AuthenticateAsync(request.Usuario, request.Contraseña);

            if (adminUser == null)
            {
                var additionalData = BuildAdditionalData(request.ClientIp, request.UserAgent);
                await _auditService.LogActionAsync(
                    cursorId: string.Empty,
                    username: request.Usuario,
                    action: "LoginFailed",
                    httpMethod: HttpMethod,
                    path: LoginPath,
                    additionalData: additionalData);

                return AdminLoginResult.AuthFailure("Credenciales administrativas inválidas");
            }

            var cursorId = adminUser.Id.ToString();
            var token = _jwtService.GenerateAdminToken(cursorId, adminUser.Username, adminUser.Id);

            var response = new AdminLoginResponse
            {
                UserId = cursorId,
                CursorId = cursorId,
                Username = adminUser.Username,
                FirstName = adminUser.FirstName,
                LastName = adminUser.LastName,
                Email = adminUser.Email,
                Role = "Admin",
                Token = token
            };

            var successAdditionalData = BuildAdditionalData(request.ClientIp, request.UserAgent);
            await _auditService.LogActionAsync(
                cursorId: cursorId,
                username: adminUser.Username,
                action: "LoginSuccess",
                httpMethod: HttpMethod,
                path: LoginPath,
                additionalData: successAdditionalData);

            return AdminLoginResult.Success(response);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error al realizar login administrativo para usuario: {Usuario}", request.Usuario);
            return AdminLoginResult.Error("Error interno del servidor");
        }
    }

    private static string? BuildAdditionalData(string? clientIp, string? userAgent)
    {
        if (string.IsNullOrEmpty(clientIp) && string.IsNullOrEmpty(userAgent))
            return null;

        var data = new Dictionary<string, string?>();
        if (!string.IsNullOrEmpty(clientIp)) data["clientIp"] = clientIp;
        if (!string.IsNullOrEmpty(userAgent)) data["userAgent"] = userAgent;

        return data.Count > 0 ? JsonSerializer.Serialize(data) : null;
    }
}
