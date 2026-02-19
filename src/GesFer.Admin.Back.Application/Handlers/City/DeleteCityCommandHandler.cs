using GesFer.Admin.Back.Application.Commands.City;
using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Infrastructure.Data;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Handlers.City;

public class DeleteCityCommandHandler : ICommandHandler<DeleteCityCommand>
{
    private readonly ApplicationDbContext _context;

    public DeleteCityCommandHandler(ApplicationDbContext context)
    {
        _context = context;
    }

    public async Task HandleAsync(DeleteCityCommand command, CancellationToken cancellationToken = default)
    {
        var city = await _context.Cities
            .FirstOrDefaultAsync(c => c.Id == command.Id && c.DeletedAt == null, cancellationToken);

        if (city == null)
            throw new InvalidOperationException($"No se encontró la ciudad con ID {command.Id}");

        // Soft delete
        city.DeletedAt = DateTime.UtcNow;
        city.IsActive = false;

        await _context.SaveChangesAsync(cancellationToken);
    }
}

