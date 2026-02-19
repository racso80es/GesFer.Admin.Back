using GesFer.Admin.Back.Application.Commands.Group;
using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Infrastructure.Data;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Handlers.Group;

public class DeleteGroupCommandHandler : ICommandHandler<DeleteGroupCommand>
{
    private readonly ApplicationDbContext _context;

    public DeleteGroupCommandHandler(ApplicationDbContext context)
    {
        _context = context;
    }

    public async Task HandleAsync(DeleteGroupCommand command, CancellationToken cancellationToken = default)
    {
        var group = await _context.Groups
            .FirstOrDefaultAsync(g => g.Id == command.Id && g.DeletedAt == null, cancellationToken);

        if (group == null)
            throw new InvalidOperationException($"No se encontró el grupo con ID {command.Id}");

        // Soft delete
        group.DeletedAt = DateTime.UtcNow;
        group.IsActive = false;

        await _context.SaveChangesAsync(cancellationToken);
    }
}

