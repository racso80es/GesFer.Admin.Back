using System.Text.Json;
using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.Commands.Company;
using GesFer.Admin.Back.Application.DTOs.Company;
using MediatR;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Handlers.Company;

/// <summary>
/// Obtiene una empresa por nombre (comparación según collation del BD, típicamente case-insensitive).
/// Registra en AuditLogs el resultado (éxito / no encontrado), alineado con AdminLoginHandler.
/// </summary>
public sealed class GetCompanyByNameHandler : IRequestHandler<GetCompanyByNameCommand, CompanyDto?>
{
    private const string Path = "/api/company/by-name";
    private const string HttpMethod = "GET";

    private readonly IApplicationDbContext _context;
    private readonly IAuditLogService _auditService;

    public GetCompanyByNameHandler(IApplicationDbContext context, IAuditLogService auditService)
    {
        _context = context;
        _auditService = auditService;
    }

    public async Task<CompanyDto?> Handle(GetCompanyByNameCommand request, CancellationToken cancellationToken)
    {
        var name = request.Name?.Trim() ?? string.Empty;
        if (string.IsNullOrEmpty(name))
            return null;

        var company = await _context.Companies
            .Where(c => c.DeletedAt == null && c.Name == name)
            .Select(c => new CompanyDto
            {
                Id = c.Id,
                Name = c.Name,
                TaxId = c.TaxId.HasValue ? c.TaxId.Value.Value : null,
                Address = c.Address,
                Phone = c.Phone,
                Email = c.Email.HasValue ? c.Email.Value.Value : null,
                PostalCodeId = c.PostalCodeId,
                CityId = c.CityId,
                StateId = c.StateId,
                CountryId = c.CountryId,
                LanguageId = c.LanguageId,
                IsActive = c.IsActive,
                CreatedAt = c.CreatedAt,
                UpdatedAt = c.UpdatedAt
            })
            .FirstOrDefaultAsync(cancellationToken);

        var cursorId = request.CursorId ?? string.Empty;
        var username = request.Username ?? string.Empty;

        if (company == null)
        {
            var notFoundData = BuildAdditionalData(request.ClientIp, request.UserAgent, name, null);
            await _auditService.LogActionAsync(
                cursorId: cursorId,
                username: username,
                action: "CompanyGetByNameNotFound",
                httpMethod: HttpMethod,
                path: Path,
                additionalData: notFoundData);
            return null;
        }

        var successData = BuildAdditionalData(request.ClientIp, request.UserAgent, name, company.Id);
        await _auditService.LogActionAsync(
            cursorId: cursorId,
            username: username,
            action: "CompanyGetByNameSuccess",
            httpMethod: HttpMethod,
            path: Path,
            additionalData: successData);

        return company;
    }

    private static string? BuildAdditionalData(string? clientIp, string? userAgent, string requestedName, Guid? companyId)
    {
        var data = new Dictionary<string, string?>();
        if (!string.IsNullOrEmpty(clientIp)) data["clientIp"] = clientIp;
        if (!string.IsNullOrEmpty(userAgent)) data["userAgent"] = userAgent;
        data["requestedName"] = requestedName;
        if (companyId.HasValue) data["companyId"] = companyId.Value.ToString();

        return data.Count > 0 ? JsonSerializer.Serialize(data) : null;
    }
}
