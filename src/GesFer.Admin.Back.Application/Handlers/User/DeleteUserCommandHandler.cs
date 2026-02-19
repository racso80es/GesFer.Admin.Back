using GesFer.Admin.Back.Application.Commands.User;
using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Infrastructure.Data;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Handlers.User;

public class DeleteUserCommandHandler : ICommandHandler<DeleteUserCommand>
{
    private readonly ApplicationDbContext _context;

    public DeleteUserCommandHandler(ApplicationDbContext context)
    {
        _context = context;
    }

    public async Task HandleAsync(DeleteUserCommand command, CancellationToken cancellationToken = default)
    {
        var user = await _context.Users
            .FirstOrDefaultAsync(u => u.Id == command.Id && u.DeletedAt == null, cancellationToken);

        if (user == null)
            throw new InvalidOperationException($"No se encontró el usuario con ID {command.Id}");

        // Soft delete
        user.DeletedAt = DateTime.UtcNow;
        user.IsActive = false;

        await _context.SaveChangesAsync(cancellationToken);
    }
}

