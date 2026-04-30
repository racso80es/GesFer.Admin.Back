using GesFer.Admin.Back.Application.Commands.User;
using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.User;
using MediatR;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Handlers.User;

public sealed class GetAllUsersHandler : IRequestHandler<GetAllUsersCommand, List<UserDto>>
{
    private readonly IApplicationDbContext _context;

    public GetAllUsersHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<List<UserDto>> Handle(GetAllUsersCommand request, CancellationToken cancellationToken)
    {
        var users = await _context.Users
            .Where(u => u.CompanyId == request.CompanyId && u.DeletedAt == null)
            .OrderBy(u => u.Username)
            .ToListAsync(cancellationToken);

        return users.Select(u => u.ToDto()).ToList();
    }
}

