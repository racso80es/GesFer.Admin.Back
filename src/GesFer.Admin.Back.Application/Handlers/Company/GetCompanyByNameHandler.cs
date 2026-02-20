using GesFer.Admin.Back.Application.Commands.Company;
using GesFer.Admin.Back.Application.DTOs.Company;
using GesFer.Admin.Back.Infrastructure.Data;
using MediatR;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Handlers.Company;

/// <summary>
/// Obtiene una empresa por nombre (comparación según collation del BD, típicamente case-insensitive).
/// </summary>
public class GetCompanyByNameHandler : IRequestHandler<GetCompanyByNameCommand, CompanyDto?>
{
    private readonly AdminDbContext _context;

    public GetCompanyByNameHandler(AdminDbContext context)
    {
        _context = context;
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

        return company;
    }
}
