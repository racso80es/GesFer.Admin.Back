using GesFer.Admin.Back.Application.Commands.Company;
using GesFer.Admin.Back.Application.DTOs.Company;
using GesFer.Admin.Back.Infrastructure.Data;
using GesFer.Admin.Back.Domain.Entities;
using GesFer.Admin.Back.Domain.ValueObjects;
using MediatR;
using Microsoft.EntityFrameworkCore;
using CompanyEntity = GesFer.Admin.Back.Domain.Entities.Company;

namespace GesFer.Admin.Back.Application.Handlers.Company;

public class CreateCompanyHandler : IRequestHandler<CreateCompanyCommand, CompanyDto>
{
    private readonly AdminDbContext _context;

    public CreateCompanyHandler(AdminDbContext context)
    {
        _context = context;
    }

    public async Task<CompanyDto> Handle(CreateCompanyCommand request, CancellationToken cancellationToken)
    {
        // Validar que no exista una empresa con el mismo nombre
        var existingCompany = await _context.Companies
            .FirstOrDefaultAsync(c => c.Name == request.Dto.Name && c.DeletedAt == null, cancellationToken);

        if (existingCompany != null)
            throw new InvalidOperationException($"Ya existe una empresa con el nombre '{request.Dto.Name}'");

        // Validar y convertir TaxId si se proporciona
        TaxId? taxId = null;
        if (!string.IsNullOrWhiteSpace(request.Dto.TaxId))
        {
            taxId = TaxId.Create(request.Dto.TaxId);
        }

        // Validar y convertir Email si se proporciona
        Email? email = null;
        if (!string.IsNullOrWhiteSpace(request.Dto.Email))
        {
            email = Email.Create(request.Dto.Email);
        }

        var company = new CompanyEntity
        {
            Name = request.Dto.Name,
            TaxId = taxId,
            Address = request.Dto.Address,
            Phone = request.Dto.Phone,
            Email = email,
            PostalCodeId = request.Dto.PostalCodeId,
            CityId = request.Dto.CityId,
            StateId = request.Dto.StateId,
            CountryId = request.Dto.CountryId,
            LanguageId = request.Dto.LanguageId,
            CreatedAt = DateTime.UtcNow,
            IsActive = true
        };

        _context.Companies.Add(company);
        await _context.SaveChangesAsync(cancellationToken);

        return new CompanyDto
        {
            Id = company.Id,
            Name = company.Name,
            TaxId = company.TaxId.HasValue ? company.TaxId.Value.Value : null,
            Address = company.Address,
            Phone = company.Phone,
            Email = company.Email.HasValue ? company.Email.Value.Value : null,
            PostalCodeId = company.PostalCodeId,
            CityId = company.CityId,
            StateId = company.StateId,
            CountryId = company.CountryId,
            LanguageId = company.LanguageId,
            IsActive = company.IsActive,
            CreatedAt = company.CreatedAt,
            UpdatedAt = company.UpdatedAt
        };
    }
}
