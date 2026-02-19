using GesFer.Admin.Back.Application.Commands.Customer;
using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.Customer;
using GesFer.Admin.Back.Infrastructure.Data;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Handlers.Customer;

public class GetCustomerByIdCommandHandler : ICommandHandler<GetCustomerByIdCommand, CustomerDto?>
{
    private readonly ApplicationDbContext _context;

    public GetCustomerByIdCommandHandler(ApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<CustomerDto?> HandleAsync(GetCustomerByIdCommand command, CancellationToken cancellationToken = default)
    {
        var customer = await _context.Customers
            .Where(c => c.Id == command.Id && c.DeletedAt == null)
            .Select(c => new CustomerDto
            {
                Id = c.Id,
                CompanyId = c.CompanyId,
                Name = c.Name,
                TaxId = c.TaxId.HasValue ? c.TaxId.Value.Value : null,
                Address = c.Address,
                Phone = c.Phone,
                Email = c.Email.HasValue ? c.Email.Value.Value : null,
                SellTariffId = c.SellTariffId,
                PostalCodeId = c.PostalCodeId,
                CityId = c.CityId,
                StateId = c.StateId,
                CountryId = c.CountryId,
                IsActive = c.IsActive,
                CreatedAt = c.CreatedAt,
                UpdatedAt = c.UpdatedAt
            })
            .FirstOrDefaultAsync(cancellationToken);

        return customer;
    }
}

