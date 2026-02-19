using GesFer.Admin.Back.Application.Commands.State;
using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.State;
using GesFer.Admin.Back.Infrastructure.Data;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Handlers.State;

public class GetStateByIdCommandHandler : ICommandHandler<GetStateByIdCommand, StateDto?>
{
    private readonly ApplicationDbContext _context;

    public GetStateByIdCommandHandler(ApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<StateDto?> HandleAsync(GetStateByIdCommand command, CancellationToken cancellationToken = default)
    {
        var state = await _context.States
            .Include(s => s.Country)
            .Where(s => s.Id == command.Id && s.DeletedAt == null)
            .Select(s => new StateDto
            {
                Id = s.Id,
                CountryId = s.CountryId,
                CountryName = s.Country.Name,
                CountryCode = s.Country.Code,
                Name = s.Name,
                Code = s.Code,
                IsActive = s.IsActive,
                CreatedAt = s.CreatedAt,
                UpdatedAt = s.UpdatedAt
            })
            .FirstOrDefaultAsync(cancellationToken);

        return state;
    }
}

