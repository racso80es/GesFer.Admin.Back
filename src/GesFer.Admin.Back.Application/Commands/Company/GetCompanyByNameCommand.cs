using GesFer.Admin.Back.Application.DTOs.Company;
using MediatR;

namespace GesFer.Admin.Back.Application.Commands.Company;

/// <summary>
/// Obtiene una empresa por nombre (case-insensitive).
/// Product usa este contrato para login; Admin expone como mucho la empresa que corresponde al contexto.
/// CursorId/Username/ClientIp/UserAgent alimentan el registro en AuditLogs (handler).
/// </summary>
public record GetCompanyByNameCommand(
    string Name,
    string? CursorId = null,
    string? Username = null,
    string? ClientIp = null,
    string? UserAgent = null) : IRequest<CompanyDto?>;
