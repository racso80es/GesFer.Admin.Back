using GesFer.Admin.Back.Application.DTOs.Auth;
using MediatR;

namespace GesFer.Admin.Back.Application.Commands.Auth;

/// <summary>
/// Command para el caso de uso de login Admin. Encapsula autenticación, token y registro en AuditLogs.
/// </summary>
public record AdminLoginCommand(
    string Usuario,
    string Contraseña,
    string? ClientIp = null,
    string? UserAgent = null) : IRequest<AdminLoginResult>;
