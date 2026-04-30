using GesFer.Admin.Back.Application.Commands.User;
using GesFer.Admin.Back.Application.Common.Interfaces;
using GesFer.Admin.Back.Application.DTOs.User;
using MediatR;
using Microsoft.EntityFrameworkCore;

namespace GesFer.Admin.Back.Application.Handlers.User;

public sealed class GetUserByIdHandler : IRequestHandler<GetUserByIdCommand, UserDto?>
{
    private readonly IApplicationDbContext _context;

    public GetUserByIdHandler(IApplicationDbContext context)
    {
        _context = context;
    }

    public async Task<UserDto?> Handle(GetUserByIdCommand request, CancellationToken cancellationToken)
    {
        var user = await _context.Users
            .FirstOrDefaultAsync(u => u.Id == request.Id && u.CompanyId == request.CompanyId && u.DeletedAt == null, cancellationToken);

        return user?.ToDto();
    }
}

