using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.Commands.Company;
using GesFer.Admin.Back.Application.DTOs.Company;
using MediatR;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Handlers.Company;

public class GetCompanyByIdHandler : IRequestHandler<GetCompanyByIdCommand, CompanyDto?>
{
    private readonly IApplicationDbContext _context;

    public GetCompanyByIdHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<CompanyDto?> Handle(GetCompanyByIdCommand request, CancellationToken cancellationToken)
    {
        var company = await _context.Companies
            .Where(c => c.Id == request.Id && c.DeletedAt == null)
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
